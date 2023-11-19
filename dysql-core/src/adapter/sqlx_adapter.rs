use std::{marker::PhantomData, any::TypeId};

use dysql_tpl::{Content, SimpleTemplate};
use sqlx::{Executor, FromRow};
use paste::paste;

use crate::{DySqlError, ErrorInner, Kind, SqlDialect, Pagination, PageDto, extract_params};

pub trait SqlxExecutorAdatper<DB> 
where 
    DB: sqlx::Database,
{
    fn create_query(&self) -> SqlxQuery<DB>
    where
        DB: sqlx::Database
    {
        SqlxQuery { temp_db: PhantomData}
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

// todo! 以下实现需要用tiaojian宏进行编译
impl<'q> SqlxExecutorAdatper<sqlx::Postgres> for sqlx::Transaction<'q, sqlx::Postgres> {}
impl SqlxExecutorAdatper<sqlx::Postgres> for sqlx::Pool<sqlx::Postgres> {}
impl SqlxExecutorAdatper<sqlx::Postgres> for &sqlx::Pool<sqlx::Postgres> {}
impl SqlxExecutorAdatper<sqlx::Postgres> for sqlx::PgConnection {}
impl SqlxExecutorAdatper<sqlx::Postgres> for &mut sqlx::PgConnection {}

impl<'q> SqlxExecutorAdatper<sqlx::MySql> for sqlx::Transaction<'q, sqlx::MySql> {}
impl SqlxExecutorAdatper<sqlx::MySql> for sqlx::Pool<sqlx::MySql> {}
impl SqlxExecutorAdatper<sqlx::MySql> for &sqlx::Pool<sqlx::MySql> {}
impl SqlxExecutorAdatper<sqlx::MySql> for sqlx::MySqlConnection {}
impl SqlxExecutorAdatper<sqlx::MySql> for &mut sqlx::MySqlConnection {}

impl<'q> SqlxExecutorAdatper<sqlx::Sqlite> for sqlx::Transaction<'q, sqlx::Sqlite> {}
impl SqlxExecutorAdatper<sqlx::Sqlite> for sqlx::Pool<sqlx::Sqlite> {}
impl SqlxExecutorAdatper<sqlx::Sqlite> for &sqlx::Pool<sqlx::Sqlite> {}
impl SqlxExecutorAdatper<sqlx::Sqlite> for sqlx::SqliteConnection {}
impl SqlxExecutorAdatper<sqlx::Sqlite> for &mut sqlx::SqliteConnection {}

macro_rules! impl_bind_param_value {
    (
        $query:ident, $p_val:ident, [$($vtype:ty),+]
    ) => {
        paste!{
            match $p_val {
                $(
                    dysql_tpl::SimpleValue::[<t_ $vtype>](val) => $query.bind(val),
                )*
                dysql_tpl::SimpleValue::t_str(val) => $query.bind(unsafe {&*val}),
                dysql_tpl::SimpleValue::t_String(val) => $query.bind(unsafe {&*val}),
                dysql_tpl::SimpleValue::t_Utc(val) => $query.bind(val),
                dysql_tpl::SimpleValue::None(val) => $query.bind(val),
                _ => Err(DySqlError(ErrorInner::new(Kind::BindParamterError, None, Some(format!("the type of {:?} is not support", $p_val)))))?,
            }
        }
    };
}

pub struct SqlxQuery <DB>
{
    pub(crate) temp_db: PhantomData<DB>,
}

impl SqlxQuery <sqlx::Postgres>
{
    pub async fn fetch_one<'e, 'c: 'e, E, D, U>(self, executor: E, named_sql: &str, dto: Option<D>) -> Result<U, DySqlError>
    where
        E: 'e + Executor<'c, Database = sqlx::Postgres> + SqlxExecutorAdatper<sqlx::Postgres>,
        D: Content + 'static + Send + Sync,
        for<'r> U: FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    {
        let sql_and_params = extract_params(&named_sql, executor.get_dialect());
        let (sql, param_names) = match sql_and_params {
            Ok(val) => val,
            Err(e) => Err(
                DySqlError(ErrorInner::new(Kind::ExtractSqlParamterError, Some(Box::new(e)), None))
            )?,
        };

        let mut query = sqlx::query_as::<_, U>(&sql);
        if let Some(dto) = &dto {
            for param_name in param_names {
                let stpl = SimpleTemplate::new(param_name);
                
                let param_value = stpl.apply(dto);
                if let Ok(param_value) = param_value {
                    query = impl_bind_param_value!(query, param_value, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime]);
                }
            }
        }

        let rst = query.fetch_one(executor).await;

        rst.map_err(|e| DySqlError(ErrorInner::new(Kind::QueryError, Some(Box::new(e)), None)))
    }

    pub async fn fetch_all<'e, 'c: 'e, E, D, U>(self, executor: E, named_sql: &str, dto: Option<D>) -> Result<Vec<U>, DySqlError>
    where
        E: 'e + Executor<'c, Database = sqlx::Postgres> + SqlxExecutorAdatper<sqlx::Postgres>,
        D: Content + 'static + Send + Sync,
        for<'r> U: FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    {
        let sql_and_params = extract_params(&named_sql, executor.get_dialect());
        let (sql, param_names) = match sql_and_params {
            Ok(val) => val,
            Err(e) => Err(
                DySqlError(ErrorInner::new(Kind::ExtractSqlParamterError, Some(Box::new(e)), None))
            )?,
        };

        let mut query = sqlx::query_as::<_, U>(&sql);
        if let Some(dto) = &dto {
            for param_name in param_names {
                let stpl = SimpleTemplate::new(param_name);
                
                let param_value = stpl.apply(dto);
                if let Ok(param_value) = param_value {
                    query = impl_bind_param_value!(query, param_value, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime]);
                }
            }
        }

        let rst = query.fetch_all(executor).await;

        rst.map_err(|e| DySqlError(ErrorInner::new(Kind::QueryError, Some(Box::new(e)), None)))
    }

    pub async fn fetch_scalar<'e, 'c: 'e, E, D, U>(self, executor: E, named_sql: &str, dto: Option<D>) -> Result<U, DySqlError>
    where
        E: 'e + Executor<'c, Database = sqlx::Postgres> + SqlxExecutorAdatper<sqlx::Postgres>,
        D: Content + 'static + Send + Sync,
        for<'r> U: sqlx::Decode<'r, sqlx::Postgres> + sqlx::Type<sqlx::Postgres> + Send + Unpin,
    {
        let sql_and_params = extract_params(&named_sql, executor.get_dialect());
        let (sql, param_names) = match sql_and_params {
            Ok(val) => val,
            Err(e) => Err(
                DySqlError(ErrorInner::new(Kind::ExtractSqlParamterError, Some(Box::new(e)), None))
            )?,
        };

        let mut query = sqlx::query_scalar::<_, U>(&sql);
        if let Some(dto) = &dto {
            for param_name in param_names {
                let stpl = SimpleTemplate::new(param_name);
                
                let param_value = stpl.apply(dto);
                if let Ok(param_value) = param_value {
                    query = impl_bind_param_value!(query, param_value, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime]);
                }
            }
        }

        let rst = query.fetch_one(executor).await;

        rst.map_err(|e| DySqlError(ErrorInner::new(Kind::QueryError, Some(Box::new(e)), None)))
    }

    pub async fn execute<'e, 'c: 'e, E, D>(self, executor: E, named_sql: &str, dto: Option<D>) -> Result<u64, DySqlError>
    where
        E: 'e + Executor<'c, Database = sqlx::Postgres> + SqlxExecutorAdatper<sqlx::Postgres>,
        D: Content + 'static + Send + Sync,
    {
        let sql_and_params = extract_params(&named_sql, executor.get_dialect());
        let (sql, param_names) = match sql_and_params {
            Ok(val) => val,
            Err(e) => Err(
                DySqlError(ErrorInner::new(Kind::ExtractSqlParamterError, Some(Box::new(e)), None))
            )?,
        };

        let mut query = sqlx::query::<_>(&sql);
        if let Some(dto) = &dto {
            for param_name in param_names {
                let stpl = SimpleTemplate::new(param_name);
                
                let param_value = stpl.apply(dto);
                if let Ok(param_value) = param_value {
                    query = impl_bind_param_value!(query, param_value, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime]);
                }
            }
        }

        let rst = query.execute(executor).await;
        let rst = rst.map_err(|e| DySqlError(ErrorInner::new(Kind::QueryError, Some(Box::new(e)), None)))?;

        let af_rows = rst.rows_affected();
        
        Ok(af_rows)
    }

    pub async fn insert<'e, 'c: 'e, E, D, U>(self, executor: E, named_sql: &str, dto: Option<D>) -> Result<U, DySqlError>
    where
        E: 'e + Executor<'c, Database = sqlx::Postgres> + SqlxExecutorAdatper<sqlx::Postgres>,
        D: Content + 'static + Send + Sync,
        for<'r> U: sqlx::Decode<'r, sqlx::Postgres> + sqlx::Type<sqlx::Postgres> + Send + Unpin,
    {
        let sql_and_params = extract_params(&named_sql, executor.get_dialect());
        let (sql, param_names) = match sql_and_params {
            Ok(val) => val,
            Err(e) => Err(
                DySqlError(ErrorInner::new(Kind::ExtractSqlParamterError, Some(Box::new(e)), None))
            )?,
        };

        let mut query = sqlx::query_scalar::<_, U>(&sql);
        if let Some(dto) = &dto {
            for param_name in param_names {
                let stpl = SimpleTemplate::new(param_name);
                
                let param_value = stpl.apply(dto);
                if let Ok(param_value) = param_value {
                    query = impl_bind_param_value!(query, param_value, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime]);
                }
            }
        }

        let insert_id = query.fetch_one(executor).await;

        insert_id.map_err(|e| DySqlError(ErrorInner::new(Kind::QueryError, Some(Box::new(e)), None)))
    }

    // pub async fn page<'e, 'c: 'e, E, D, U>(self, executor: E, page_dto: PageDto<D>) -> Result<Pagination<U>, DySqlError>
    // where
    //     E: 'e + Executor<'c, Database = sqlx::Postgres> + SqlxExecutorAdatper<sqlx::Postgres>,
    //     D: Content + 'static + Send + Sync,
    //     for<'r> U: FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    // {
    //     // let count_sql;
    //     // todo!


    //     // let page_sql;
        
    //     let mut query = sqlx::query_as::<_, U>(self.sql);
        
    //     for param_name in self.param_names {
    //         let stpl = SimpleTemplate::new(param_name);
            
    //         let param_value = stpl.apply(&page_dto);
    //         if let Ok(param_value) = param_value {
    //             query = impl_bind_param_value!(query, param_value, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime]);
    //         }
    //     }

    //     let rst = query.fetch_all(executor).await;
    //     let rst = rst.map_err(|e| DySqlError(ErrorInner::new(Kind::QueryError, Some(Box::new(e)), None)))?;

    //     let pg_data = Pagination::from_dto(&page_dto, rst);

    //     Ok(pg_data)
    // }
}
