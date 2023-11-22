use dysql_tpl::{Content, SimpleTemplate};
use sqlx::{Executor, FromRow};

use crate::{SqlxQuery, SqlxExecutorAdatper};
use crate::{DySqlError, ErrorInner, Kind, Pagination, PageDto, extract_params};

impl<'q> SqlxExecutorAdatper<sqlx::Postgres> for sqlx::Transaction<'q, sqlx::Postgres> {}
impl SqlxExecutorAdatper<sqlx::Postgres> for sqlx::Pool<sqlx::Postgres> {}
impl SqlxExecutorAdatper<sqlx::Postgres> for &sqlx::Pool<sqlx::Postgres> {}
impl SqlxExecutorAdatper<sqlx::Postgres> for sqlx::PgConnection {}
impl SqlxExecutorAdatper<sqlx::Postgres> for &mut sqlx::PgConnection {}

impl SqlxQuery <sqlx::Postgres>
{
    /// named_sql: 是已经代入 dto 进行模版 render 后的 named sql 
    pub async fn fetch_one<'e, 'c: 'e, E, D, U>(self, executor: E, named_sql: &str, dto: Option<D>) -> Result<U, DySqlError>
    where
        E: 'e + Executor<'c, Database = sqlx::Postgres> + SqlxExecutorAdatper<sqlx::Postgres>,
        D: Content + Send + Sync,
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
            for param_name in &param_names {
                let stpl = SimpleTemplate::new(param_name);
                
                let param_value = stpl.apply(dto);
                match param_value {
                    Ok(param_value) => {
                        
                        query = impl_bind_sqlx_param_value!(query, param_value, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
                    },
                    Err(e) => Err(DySqlError(ErrorInner::new(Kind::BindParamterError, Some(e), None)))?,
                }
            }
        }

        let rst = query.fetch_one(executor).await;

        rst.map_err(|e| DySqlError(ErrorInner::new(Kind::QueryError, Some(Box::new(e)), None)))
    }

    pub async fn fetch_all<'e, 'c: 'e, E, D, U>(self, executor: E, named_sql: &str, dto: Option<D>) -> Result<Vec<U>, DySqlError>
    where
        E: 'e + Executor<'c, Database = sqlx::Postgres> + SqlxExecutorAdatper<sqlx::Postgres>,
        D: Content + Send + Sync,
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
            for param_name in &param_names {
                let stpl = SimpleTemplate::new(param_name);
                
                let param_value = stpl.apply(dto);
                match param_value {
                    Ok(param_value) => {
                        query = impl_bind_sqlx_param_value!(query, param_value, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
                    },
                    Err(e) => Err(DySqlError(ErrorInner::new(Kind::BindParamterError, Some(e), None)))?,
                }
            }
        }

        let rst = query.fetch_all(executor).await;

        rst.map_err(|e| DySqlError(ErrorInner::new(Kind::QueryError, Some(Box::new(e)), None)))
    }

    pub async fn fetch_scalar<'e, 'c: 'e, E, D, U>(self, executor: E, named_sql: &str, dto: Option<D>) -> Result<U, DySqlError>
    where
        E: 'e + Executor<'c, Database = sqlx::Postgres> + SqlxExecutorAdatper<sqlx::Postgres>,
        D: Content + Send + Sync,
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
            for param_name in &param_names {
                let stpl = SimpleTemplate::new(param_name);
                
                let param_value = stpl.apply(dto);
                match param_value {
                    Ok(param_value) => {
                        query = impl_bind_sqlx_param_value!(query, param_value, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
                    },
                    Err(e) => Err(DySqlError(ErrorInner::new(Kind::BindParamterError, Some(e), None)))?,
                }
            }
        }

        let rst = query.fetch_one(executor).await;

        rst.map_err(|e| DySqlError(ErrorInner::new(Kind::QueryError, Some(Box::new(e)), None)))
    }

    pub async fn execute<'e, 'c: 'e, E, D>(self, executor: E, named_sql: &str, dto: Option<D>) -> Result<u64, DySqlError>
    where
        E: 'e + Executor<'c, Database = sqlx::Postgres> + SqlxExecutorAdatper<sqlx::Postgres>,
        D: Content + Send + Sync,
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
            for param_name in &param_names {
                let stpl = SimpleTemplate::new(param_name);
                
                let param_value = stpl.apply(dto);
                match param_value {
                    Ok(param_value) => {
                        query = impl_bind_sqlx_param_value!(query, param_value, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
                    },
                    Err(e) => Err(DySqlError(ErrorInner::new(Kind::BindParamterError, Some(e), None)))?,
                }
            }
        }

        let rst = query.execute(executor).await;
        let rst = rst.map_err(|e| DySqlError(ErrorInner::new(Kind::QueryError, Some(Box::new(e)), None)))?;

        let af_rows = rst.rows_affected();
        
        Ok(af_rows)
    }

    pub async fn insert<'e, 'c: 'e, E, D, U>(self, executor: E, named_sql: &str, dto: Option<D>) -> Result<Option<U>, DySqlError>
    where
        E: 'e + Executor<'c, Database = sqlx::Postgres> + SqlxExecutorAdatper<sqlx::Postgres> ,
        D: Content + Send + Sync,
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
            for param_name in &param_names {
                let stpl = SimpleTemplate::new(param_name);
                
                let param_value = stpl.apply(dto);
                match param_value {
                    Ok(param_value) => {
                        query = impl_bind_sqlx_param_value!(query, param_value, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
                    },
                    Err(e) => Err(DySqlError(ErrorInner::new(Kind::BindParamterError, Some(e), None)))?,
                }
            }
        }

        let insert_id = query.fetch_one(executor).await;
        

        let insert_id = insert_id.map_err(|e| DySqlError(ErrorInner::new(Kind::QueryError, Some(Box::new(e)), None)))?;
        Ok(Some(insert_id))
    }

    pub async fn fetch_insert_id<'e, 'c: 'e, E>(self, _executor: E) -> Result<i64, DySqlError>
    {
        Ok(-1)
    }

    pub async fn page<'e, 'c: 'e, E, D, U>(self, executor: E, named_sql: &str, page_dto: &PageDto<D>) -> Result<Pagination<U>, DySqlError>
    where
        E: 'e + Executor<'c, Database = sqlx::Postgres> + SqlxExecutorAdatper<sqlx::Postgres>,
        D: Content + Send + Sync,
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
        for param_name in &param_names {
            let stpl = SimpleTemplate::new(param_name);
            
            let param_value = stpl.apply(&page_dto);
            match param_value {
                Ok(param_value) => {
                    query = impl_bind_sqlx_param_value!(query, param_value, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
                },
                Err(e) => Err(DySqlError(ErrorInner::new(Kind::BindParamterError, Some(e), None)))?,
            }
        }

        let rst = query.fetch_all(executor).await;
        let rst = match rst {
            Ok(v) => v,
            Err(e) => Err(DySqlError(ErrorInner::new(Kind::QueryError, Some(Box::new(e)), None)))?,
        };

        let pg_data = Pagination::from_dto(&page_dto, rst);

        Ok(pg_data)
    }
}