use std::{marker::PhantomData, any::TypeId};

use dysql_tpl::{Content, SimpleTemplate};
use sqlx::{Executor, FromRow, Database, IntoArguments, database::HasArguments};

use crate::{DySqlError, ErrorInner, Kind, SqlDialect};

pub struct SqlxQuery <'q, D, DB> 
where 
    D: Content + 'static + Send + Sync,
    DB: Database,
{
    /// 不含模版信息的且替换掉命名参数的 sql
    pub sql: &'q str,
    pub param_names: Vec<&'q str>,
    pub dto: Option<D>,
    pub(crate) temp_db: PhantomData<DB>,
}

impl<'q, D, DB> SqlxQuery <'q, D, DB> 
where 
    D: Content + 'static + Send + Sync,
    DB: Database,
{
    pub async fn fetch_one<'e, 'c: 'e, E, U>(self, executor: E) -> Result<U, DySqlError>
    where
        E: 'e + Executor<'c, Database = DB>,
        for<'r> U: FromRow<'r, <DB as Database>::Row> + Send + Unpin,
        <DB as HasArguments<'q>>::Arguments: IntoArguments<'q, DB>
    {
        let query = sqlx::query_as::<DB, U>(self.sql);
        // todo : query.bind(), 需要在Content里取值
        for params in self.param_names {
            let stpl = SimpleTemplate::new(params);
            let val = stpl.apply(&self.dto);

            todo!()
        }

        let rst = query.fetch_one(executor).await;

        rst.map_err(|e| DySqlError(ErrorInner::new(Kind::QueryError, Some(Box::new(e)), None)))
    }
}

pub trait SqlxExecutorAdatper<'q, DB> 
where 
    DB: Database,
{
    fn create_query<D> (&self, sql: &'q str, param_names: Vec<&'q str>, dto: Option<D>) -> SqlxQuery<'q, D, DB>
    where 
        D: Content + 'static + Send + Sync,
        DB: Database
    {
        SqlxQuery {
            sql,
            param_names,
            dto,
            temp_db: PhantomData,
        }
    }

    fn get_dialect(&self) -> SqlDialect 
    {
        // todo! 以下分支需要用条件宏进行编译
        if TypeId::of::<DB>() == TypeId::of::<sqlx::Postgres>() {
            return SqlDialect::postgres
        }
        
        if TypeId::of::<DB>() == TypeId::of::<sqlx::MySql>() {
            return SqlDialect::mysql
        } 
        
        if TypeId::of::<DB>() == TypeId::of::<sqlx::Sqlite>() {
            return SqlDialect::sqlite
        }

        panic!("only support 'postgres', 'mysql', 'sqlite' sql dialect")
    }
}

macro_rules! impl_sqlx_executor_adapter_types {
    (
        $(::$executor:ident)+
        <
            $($q:lifetime,)?
            $database:path
        >
    ) => {
        impl<'q> SqlxExecutorAdatper<'q, $database> for $(::$executor)*<$($q,)* $database> {}
    }
}

// todo! 以下实现需要用tiaojian宏进行编译
impl_sqlx_executor_adapter_types!(::sqlx::Transaction<'q, sqlx::Postgres>);
impl_sqlx_executor_adapter_types!(::sqlx::Pool<sqlx::Postgres>);
impl<'q> SqlxExecutorAdatper<'q, sqlx::Postgres> for sqlx::PgConnection {}

impl_sqlx_executor_adapter_types!(::sqlx::Transaction<'q, sqlx::MySql>);
impl_sqlx_executor_adapter_types!(::sqlx::Pool<sqlx::MySql>);
impl<'q> SqlxExecutorAdatper<'q, sqlx::Postgres> for sqlx::MySqlConnection {}

impl_sqlx_executor_adapter_types!(::sqlx::Transaction<'q, sqlx::Sqlite>);
impl_sqlx_executor_adapter_types!(::sqlx::Pool<sqlx::Sqlite>);
impl<'q> SqlxExecutorAdatper<'q, sqlx::Sqlite> for sqlx::SqliteConnection {}
