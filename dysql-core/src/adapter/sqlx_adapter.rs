use std::{marker::PhantomData, any::TypeId};

use dysql_tpl::{Content, SimpleTemplate};
use sqlx::{Executor, FromRow};
use paste::paste;

use crate::{DySqlError, ErrorInner, Kind, SqlDialect};


macro_rules! impl_bind_param_value {
    (
        $query:ident, $p_val:ident, $($vtype:ty),+
    ) => {
        paste!{
            match $p_val {
                $(
                    dysql_tpl::SimpleValue::[<t_ $vtype>](val) => $query.bind(val),
                )*
                dysql_tpl::SimpleValue::t_str(val) => $query.bind(unsafe {&*val}),
                dysql_tpl::SimpleValue::t_String(val) => $query.bind(unsafe {&*val}),
                dysql_tpl::SimpleValue::t_Utc(val) => $query.bind(val),
                dysql_tpl::SimpleValue::Null(_) => $query.bind(Option::<i32>::None),
                _ => Err(DySqlError(ErrorInner::new(Kind::BindParamterError, None, Some(format!("the type of {:?} is not support", $p_val)))))?,
            }
        }
    };
}

pub struct SqlxQuery <'q, D: Clone, DB>
{
    /// 不含模版信息的且替换掉命名参数的 sql
    pub sql: &'q str,
    pub param_names: Vec<&'q str>,
    pub dto: Option<D>,
    pub(crate) temp_db: PhantomData<DB>,
}

impl<'q, D: Clone> SqlxQuery <'q, D, sqlx::Postgres> 
where 
    D: Content + 'static + Send + Sync,
{
    pub async fn fetch_one<'e, 'c: 'e, E, U>(self, executor: E) -> Result<U, DySqlError>
    where
        E: 'e + Executor<'c, Database = sqlx::Postgres>,
        for<'r> U: FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    {
        let mut query = sqlx::query_as::<sqlx::Postgres, U>(self.sql);
        if let Some(dto) = &self.dto {
            for param_name in self.param_names {
                let stpl = SimpleTemplate::new(param_name);
                
                let param_value = stpl.apply(dto);
                if let Ok(param_value) = param_value {
                    query = impl_bind_param_value!(query, param_value, i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime);
                }
            }
        }

        let rst = query.fetch_one(executor).await;

        rst.map_err(|e| DySqlError(ErrorInner::new(Kind::QueryError, Some(Box::new(e)), None)))
    }

    pub async fn execute<'e, 'c: 'e, E>(self, executor: E) -> Result<u64, DySqlError>
    where
        E: 'e + Executor<'c, Database = sqlx::Postgres>,
    {
        let mut query = sqlx::query::<sqlx::Postgres>(self.sql);
        if let Some(dto) = &self.dto {
            for param_name in self.param_names {
                let stpl = SimpleTemplate::new(param_name);
                
                let param_value = stpl.apply(dto);
                if let Ok(param_value) = param_value {
                    query = impl_bind_param_value!(query, param_value, i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime);
                }
            }
        }

        let rst = query.execute(executor).await;
        let rst = rst.map_err(|e| DySqlError(ErrorInner::new(Kind::QueryError, Some(Box::new(e)), None)))?;

        let af_rows = rst.rows_affected();
        
        Ok(af_rows)
    }
}

pub trait SqlxExecutorAdatper<'q, DB> 
where 
    DB: sqlx::Database,
{
    fn create_query<D: Clone> (&self, sql: &'q str, param_names: Vec<&'q str>, dto: Option<D>) -> SqlxQuery<'q, D, DB>
    where 
        D: Content + 'static + Send + Sync,
        DB: sqlx::Database
    {
        println!("param_names: {:#?}", param_names);

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
