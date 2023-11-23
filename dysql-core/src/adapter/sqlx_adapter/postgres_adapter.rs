use std::sync::Arc;
use std::io::Write;

use dysql_tpl::{Content, SimpleTemplate, Template};
use sqlx::{Executor, FromRow};

use crate::{SqlxQuery, SqlxExecutorAdatper, gen_named_sql, SqlNodeLinkList};
use crate::{DySqlError, ErrorInner, Kind, Pagination, PageDto, extract_params};

impl<'q> SqlxExecutorAdatper<sqlx::Postgres> for sqlx::Transaction<'q, sqlx::Postgres> {}
impl SqlxExecutorAdatper<sqlx::Postgres> for sqlx::Pool<sqlx::Postgres> {}
impl SqlxExecutorAdatper<sqlx::Postgres> for &sqlx::Pool<sqlx::Postgres> {}
impl SqlxExecutorAdatper<sqlx::Postgres> for sqlx::PgConnection {}
impl SqlxExecutorAdatper<sqlx::Postgres> for &mut sqlx::PgConnection {}

impl SqlxQuery <sqlx::Postgres>
{
    /// named_sql: 是已经代入 dto 进行模版 render 后的 named sql 
    pub async fn fetch_one<'e, 'c: 'e, E, D, U>(self, executor: E, named_template: Arc<Template>, dto: Option<D>) -> Result<U, DySqlError>
    where
        E: 'e + Executor<'c, Database = sqlx::Postgres> + SqlxExecutorAdatper<sqlx::Postgres>,
        D: Content + Send + Sync,
        for<'r> U: FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    {
        let named_sql = gen_named_sql(named_template, &dto);
        
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

    pub async fn fetch_all<'e, 'c: 'e, E, D, U>(self, executor: E, named_template: Arc<Template>, dto: Option<D>) -> Result<Vec<U>, DySqlError>
    where
        E: 'e + Executor<'c, Database = sqlx::Postgres> + SqlxExecutorAdatper<sqlx::Postgres>,
        D: Content + Send + Sync,
        for<'r> U: FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    {
        let named_sql = gen_named_sql(named_template, &dto);
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

    pub async fn fetch_scalar<'e, 'c: 'e, E, D, U>(self, executor: E, named_template: Arc<Template>, dto: Option<D>) -> Result<U, DySqlError>
    where
        E: 'e + Executor<'c, Database = sqlx::Postgres> + SqlxExecutorAdatper<sqlx::Postgres>,
        D: Content + Send + Sync,
        for<'r> U: sqlx::Decode<'r, sqlx::Postgres> + sqlx::Type<sqlx::Postgres> + Send + Unpin,
    {
        let named_sql = gen_named_sql(named_template, &dto);
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

    pub async fn execute<'e, 'c: 'e, E, D>(self, executor: E, named_template: Arc<Template>, dto: Option<D>) -> Result<u64, DySqlError>
    where
        E: 'e + Executor<'c, Database = sqlx::Postgres> + SqlxExecutorAdatper<sqlx::Postgres>,
        D: Content + Send + Sync,
    {
        let named_sql = gen_named_sql(named_template, &dto);
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

    pub async fn insert<'e, 'c: 'e, E, D, U>(self, executor: E, named_template: Arc<Template>, dto: Option<D>) -> Result<Option<U>, DySqlError>
    where
        E: 'e + Executor<'c, Database = sqlx::Postgres> + SqlxExecutorAdatper<sqlx::Postgres> ,
        D: Content + Send + Sync,
        for<'r> U: sqlx::Decode<'r, sqlx::Postgres> + sqlx::Type<sqlx::Postgres> + Send + Unpin,
    {
        let named_sql = gen_named_sql(named_template, &dto);
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

    pub async fn page_count<'e, 'c: 'e, E, D, U>(self, executor: E, named_template: Arc<Template>, dto: Option<D>) -> Result<U, DySqlError>
    where
        E: 'e + Executor<'c, Database = sqlx::Postgres> + SqlxExecutorAdatper<sqlx::Postgres>,
        D: Content + Send + Sync,
        for<'r> U: sqlx::Decode<'r, sqlx::Postgres> + sqlx::Type<sqlx::Postgres> + Send + Unpin,
    {
        let named_sql = gen_named_sql(named_template, &dto);
        
        let sql_and_params = extract_params(&named_sql, executor.get_dialect());
        let (sql, param_names) = match sql_and_params {
            Ok(val) => val,
            Err(e) => Err(
                DySqlError(ErrorInner::new(Kind::ExtractSqlParamterError, Some(Box::new(e)), None))
            )?,
        };

        // count sql
        let buffer_size = sql.len() + 200;
        let mut sql_buf = Vec::<u8>::with_capacity(buffer_size);
        let count_sql = {
            write!(sql_buf, "SELECT count(*) FROM ({}) as __dy_tmp", sql).unwrap();
            std::str::from_utf8(&sql_buf).unwrap()
        };

        let mut query = sqlx::query_scalar::<_, U>(&count_sql);
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

    pub async fn page_all<'e, 'c: 'e, E, D, U>(self, executor: E, named_template: Arc<Template>, page_dto: &PageDto<D>) -> Result<Pagination<U>, DySqlError>
    where
        E: 'e + Executor<'c, Database = sqlx::Postgres> + SqlxExecutorAdatper<sqlx::Postgres>,
        D: Content + Send + Sync,
        for<'r> U: FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    {
        let named_sql = {
            let named_sql= named_template.render(page_dto);
            // 格式化 sql 并解析 BDEL 和 FDEL 指令
            SqlNodeLinkList::new(&named_sql).trim().to_string()
        };
    
        let sql_and_params = extract_params(&named_sql, executor.get_dialect());
        let (sql, param_names) = match sql_and_params {
            Ok(val) => val,
            Err(e) => Err(
                DySqlError(ErrorInner::new(Kind::ExtractSqlParamterError, Some(Box::new(e)), None))
            )?,
        };

        let buffer_size = sql.len() + 200;
        let mut sql_buf = Vec::<u8>::with_capacity(buffer_size);
        let page_sql = {
            let sort_fragment = "{{#is_sort}} ORDER BY {{#sort_model}} {{field}} {{sort}}, {{/sort_model}} ![B_DEL(,)] {{/is_sort}} LIMIT {{page_size}} OFFSET {{start}}";
            let template = Template::new(sort_fragment)
                .map_err(|e| 
                    DySqlError(ErrorInner::new(Kind::TemplateParseError, Some(e.into()), None))
                )?;
            let sort_fragment = template.render(page_dto);
            let sort_fragment = SqlNodeLinkList::new(&sort_fragment).trim().to_string();
            
            write!(sql_buf, "{} {} ", sql, sort_fragment).unwrap();
            std::str::from_utf8(&sql_buf)
                .map_err(|e| 
                    DySqlError(ErrorInner::new(Kind::TemplateParseError, Some(e.into()), None))
                )?
        };

        let mut query = sqlx::query_as::<_, U>(&page_sql);
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