use std::{marker::PhantomData, any::TypeId};

use dysql_tpl::Content;
use sqlx::{Executor, FromRow, Database, IntoArguments, database::HasArguments};

use crate::{DySqlError, ErrorInner, Kind, SqlDialect};

pub struct SqlxQuery <'q, D, DB> 
where 
    D: Content + 'static + Send + Sync,
    DB: Database,
{
    pub sql: &'q str,
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
        let rst = query.fetch_one(executor).await;

        rst.map_err(|e| DySqlError(ErrorInner::new(Kind::QueryError, Some(Box::new(e)), None)))
    }
}

pub trait SqlxExecutorAdatper<'q, D, DB> 
where 
    D: Content + 'static + Send + Sync,
    DB: Database,
{
    fn create_query(&self, sql: &'q str, dto: Option<D>) -> SqlxQuery<'q, D, DB>
    {
        SqlxQuery {
            sql,
            dto,
            temp_db: PhantomData,
        }
    }

    fn get_dialect(&self) -> SqlDialect {
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
    ($( $name:ty ),*) => {
        $(
            impl<'q, D, DB> SqlxExecutorAdatper<'q, D, DB> for $name
            where 
                D: Content + 'static + Send + Sync,
                DB: Database
            {}
        )*
    }
}

impl_sqlx_executor_adapter_types!(sqlx::Transaction<'q, DB>);
// todo! 以下实现需要用tiaojian宏进行编译
impl_sqlx_executor_adapter_types!(sqlx::Pool<sqlx::Postgres>, sqlx::PgConnection);
impl_sqlx_executor_adapter_types!(sqlx::Pool<sqlx::MySql>, sqlx::MySqlConnection);
impl_sqlx_executor_adapter_types!(sqlx::Pool<sqlx::Sqlite>, sqlx::SqliteConnection);