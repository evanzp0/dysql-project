
use std::sync::Arc;

use async_trait::async_trait;
use dysql_tpl::{Content, SimpleTemplate, SimpleValue, Template};
use tokio_postgres::{Statement, Error, types::{ToSql, FromSql}, Row, ToStatement};
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{TokioPgExecutorAdatper, TokioPgQuery, DySqlError, extract_params, ErrorInner, Kind, PageDto, Pagination};

impl TokioPgQuery
{
    /// named_sql: 是已经代入 dto 进行模版 render 后的 named sql 
    pub async fn fetch_one<E, D, U>(self, executor: &E, named_template: Arc<Template>, dto: Option<D>)
        -> Result<U, DySqlError>
    where 
        E: TokioPgExecutorAdatper,
        D: Content + Send + Sync,
        U: FromTokioPostgresRow,
    {
        let named_sql = crate::gen_named_sql(named_template, &dto);
        let sql_and_params = extract_params(&named_sql, executor.get_dialect());
        let (sql, param_names) = match sql_and_params {
            Ok(val) => val,
            Err(e) => Err(
                DySqlError(ErrorInner::new(Kind::ExtractSqlParamterError, Some(Box::new(e)), None))
            )?,
        };
        let stmt = (*executor)
            .prepare(&sql)
            .await
            .map_err(|e| DySqlError(ErrorInner::new(Kind::PrepareStamentError, Some(Box::new(e)), None)))?;

        let mut param_values : Vec<SimpleValue> = Vec::with_capacity(param_names.len());
        if let Some(dto) = &dto {
            for param_name in &param_names {
                let stpl = SimpleTemplate::new(param_name);
                
                let param_value = stpl
                    .apply(dto)
                    .map_err(|e| DySqlError(ErrorInner::new(Kind::BindParamterError, Some(e), None)))?;
                param_values.push(param_value);
            }
        }
        let mut tosql_values : Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::with_capacity(param_names.len());
        for param_value in &param_values {
            impl_bind_tokio_pg_param_value!(tosql_values, param_value, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
        }

        let params = tosql_values.into_iter();
        let params = params.as_slice();
        let row = (*executor)
            .query_one(&stmt, &params)
            .await
            .map_err(|e| {
                if e.to_string().contains("number of rows") {
                    DySqlError(ErrorInner::new(Kind::RecordNotFound, Some(Box::new(e)), None))
                } else {
                    DySqlError(ErrorInner::new(Kind::QueryError, Some(Box::new(e)), None))
                }
            })?;
        let rst = <U>::from_row(row)
            .map_err(|e| DySqlError(ErrorInner::new(Kind::ObjectMappingError, Some(Box::new(e)), None)))?;

        Ok(rst)
    }

    pub async fn fetch_all<E, D, U>(self, executor: &E, named_template: Arc<Template>, dto: Option<D>)
        -> Result<Vec<U>, DySqlError>
    where 
        E: TokioPgExecutorAdatper,
        D: Content + Send + Sync,
        U: FromTokioPostgresRow,
    {
        let named_sql = crate::gen_named_sql(named_template, &dto);
        let sql_and_params = extract_params(&named_sql, executor.get_dialect());
        let (sql, param_names) = match sql_and_params {
            Ok(val) => val,
            Err(e) => Err(
                DySqlError(ErrorInner::new(Kind::ExtractSqlParamterError, Some(Box::new(e)), None))
            )?,
        };
        let stmt = (*executor)
            .prepare(&sql)
            .await
            .map_err(|e| DySqlError(ErrorInner::new(Kind::PrepareStamentError, Some(Box::new(e)), None)))?;

        let mut param_values : Vec<SimpleValue> = Vec::with_capacity(param_names.len());
        if let Some(dto) = &dto {
            for param_name in &param_names {
                let stpl = SimpleTemplate::new(param_name);
                
                let param_value = stpl
                    .apply(dto)
                    .map_err(|e| DySqlError(ErrorInner::new(Kind::BindParamterError, Some(e), None)))?;
                param_values.push(param_value);
            }
        }
        let mut tosql_values : Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::with_capacity(param_names.len());
        for param_value in &param_values {
            impl_bind_tokio_pg_param_value!(tosql_values, param_value, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
        }

        let params = tosql_values.into_iter();
        let params = params.as_slice();


        let rows = (*executor)
            .query(&stmt, &params)
            .await
            .map_err(|e| DySqlError(ErrorInner::new(Kind::QueryError, Some(Box::new(e)), None)))?;

        let rst = rows
            .iter()
            .map(|row| <U>::from_row_ref(row).expect("query unexpected error"))
            .collect::<Vec<U>>();

        Ok(rst)
    }

    pub async fn fetch_scalar<E, D, U>(self, executor: &E, named_template: Arc<Template>, dto: Option<D>)
        -> Result<U, DySqlError>
    where 
        E: TokioPgExecutorAdatper,
        D: Content + Send + Sync,
        for<'a> U: FromSql<'a>,
    {
        let named_sql = crate::gen_named_sql(named_template, &dto);
        let sql_and_params = extract_params(&named_sql, executor.get_dialect());
        let (sql, param_names) = match sql_and_params {
            Ok(val) => val,
            Err(e) => Err(
                DySqlError(ErrorInner::new(Kind::ExtractSqlParamterError, Some(Box::new(e)), None))
            )?,
        };
        let stmt = (*executor)
            .prepare(&sql)
            .await
            .map_err(|e| DySqlError(ErrorInner::new(Kind::PrepareStamentError, Some(Box::new(e)), None)))?;

        let mut param_values : Vec<SimpleValue> = Vec::with_capacity(param_names.len());
        if let Some(dto) = &dto {
            for param_name in &param_names {
                let stpl = SimpleTemplate::new(param_name);
                
                let param_value = stpl
                    .apply(dto)
                    .map_err(|e| DySqlError(ErrorInner::new(Kind::BindParamterError, Some(e), None)))?;
                param_values.push(param_value);
            }
        }
        let mut tosql_values : Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::with_capacity(param_names.len());
        for param_value in &param_values {
            impl_bind_tokio_pg_param_value!(tosql_values, param_value, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
        }

        let params = tosql_values.into_iter();
        let params = params.as_slice();

        let row = (*executor)
            .query_one(&stmt, &params)
            .await
            .map_err(|e| DySqlError(ErrorInner::new(Kind::QueryError, Some(Box::new(e)), None)))?;

        let rst: U = row.get(0);

        Ok(rst)
    }
    
    pub async fn execute<E, D>(self, executor: &E, named_template: Arc<Template>, dto: Option<D>)
        -> Result<u64, DySqlError>
    where 
        E: TokioPgExecutorAdatper,
        D: Content + Send + Sync,
    {
        let named_sql = crate::gen_named_sql(named_template, &dto);
        let sql_and_params = extract_params(&named_sql, executor.get_dialect());
        let (sql, param_names) = match sql_and_params {
            Ok(val) => val,
            Err(e) => Err(
                DySqlError(ErrorInner::new(Kind::ExtractSqlParamterError, Some(Box::new(e)), None))
            )?,
        };
        let stmt = (*executor)
            .prepare(&sql)
            .await
            .map_err(|e| DySqlError(ErrorInner::new(Kind::PrepareStamentError, Some(Box::new(e)), None)))?;

        let mut param_values : Vec<SimpleValue> = Vec::with_capacity(param_names.len());
        if let Some(dto) = &dto {
            for param_name in &param_names {
                let stpl = SimpleTemplate::new(param_name);
                
                let param_value = stpl
                    .apply(dto)
                    .map_err(|e| DySqlError(ErrorInner::new(Kind::BindParamterError, Some(e), None)))?;
                param_values.push(param_value);
            }
        }
        let mut tosql_values : Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::with_capacity(param_names.len());
        for param_value in &param_values {
            impl_bind_tokio_pg_param_value!(tosql_values, param_value, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
        }

        let params = tosql_values.into_iter();
        let params = params.as_slice();

        let affect_count = (*executor)
            .execute(&stmt, &params)
            .await
            .map_err(|e| DySqlError(ErrorInner::new(Kind::QueryError, Some(Box::new(e)), None)))?;

        Ok(affect_count)
    }

    pub async fn insert<E, D, U>(self, executor: &E, named_template: Arc<Template>, dto: Option<D>)
        -> Result<Option<U>, DySqlError>
    where 
        E: TokioPgExecutorAdatper,
        D: Content + Send + Sync,
        for<'a> U: FromSql<'a>,
    {
        let named_sql = crate::gen_named_sql(named_template, &dto);
        let sql_and_params = extract_params(&named_sql, executor.get_dialect());
        let (sql, param_names) = match sql_and_params {
            Ok(val) => val,
            Err(e) => Err(
                DySqlError(ErrorInner::new(Kind::ExtractSqlParamterError, Some(Box::new(e)), None))
            )?,
        };
        let stmt = (*executor)
            .prepare(&sql)
            .await
            .map_err(|e| DySqlError(ErrorInner::new(Kind::PrepareStamentError, Some(Box::new(e)), None)))?;

        let mut param_values : Vec<SimpleValue> = Vec::with_capacity(param_names.len());
        if let Some(dto) = &dto {
            for param_name in &param_names {
                let stpl = SimpleTemplate::new(param_name);
                
                let param_value = stpl
                    .apply(dto)
                    .map_err(|e| DySqlError(ErrorInner::new(Kind::BindParamterError, Some(e), None)))?;
                param_values.push(param_value);
            }
        }
        let mut tosql_values : Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::with_capacity(param_names.len());
        for param_value in &param_values {
            impl_bind_tokio_pg_param_value!(tosql_values, param_value, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
        }
        let params = tosql_values.into_iter();
        let params = params.as_slice();
        
        let row = (*executor)
            .query_one(&stmt, &params)
            .await;
        let row = row.unwrap();
        let rst: U = row.get(0);

        Ok(Some(rst))
    }

    pub async fn fetch_insert_id<'e, 'c: 'e, E>(self, _executor: E) -> Result<i64, DySqlError>
    {
        Ok(-1)
    }

    pub async fn page_count<E, D, U>(self, executor: &E, named_template: Arc<Template>, dto: Option<D>)
        -> Result<U, DySqlError>
    where 
        E: TokioPgExecutorAdatper,
        D: Content + Send + Sync,
        for<'a> U: FromSql<'a>,
    {
        use std::io::Write;

        let named_sql = crate::gen_named_sql(named_template, &dto);
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

        let stmt = (*executor)
            .prepare(count_sql)
            .await
            .map_err(|e| DySqlError(ErrorInner::new(Kind::PrepareStamentError, Some(Box::new(e)), None)))?;

        let mut param_values : Vec<SimpleValue> = Vec::with_capacity(param_names.len());
        if let Some(dto) = &dto {
            for param_name in &param_names {
                let stpl = SimpleTemplate::new(param_name);
                
                let param_value = stpl
                    .apply(dto)
                    .map_err(|e| DySqlError(ErrorInner::new(Kind::BindParamterError, Some(e), None)))?;
                param_values.push(param_value);
            }
        }
        let mut tosql_values : Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::with_capacity(param_names.len());
        for param_value in &param_values {
            impl_bind_tokio_pg_param_value!(tosql_values, param_value, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
        }

        let params = tosql_values.into_iter();
        let params = params.as_slice();

        let row = (*executor)
            .query_one(&stmt, &params)
            .await
            .map_err(|e| DySqlError(ErrorInner::new(Kind::QueryError, Some(Box::new(e)), None)))?;

        let rst: U = row.get(0);

        Ok(rst)
    }

    pub async fn page_all<E, D, U>(self, executor: &E, named_template: Arc<Template>, page_dto: &PageDto<D>)
        -> Result<Pagination<U>, DySqlError>
    where 
        E: TokioPgExecutorAdatper,
        D: Content + Send + Sync,
        U: FromTokioPostgresRow,
    {   
        use std::io::Write;

        let named_sql = {
            let named_sql= named_template.render(page_dto);
            // 格式化 sql 并解析 BDEL 和 FDEL 指令
            crate::SqlNodeLinkList::new(&named_sql).trim().to_string()
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
            let template = dysql_tpl::Template::new(sort_fragment)
                .map_err(|e| 
                    crate::DySqlError(crate::ErrorInner::new(crate::Kind::TemplateParseError, Some(e.into()), None))
                )?;
            let sort_fragment = template.render(page_dto);
            let sort_fragment = crate::SqlNodeLinkList::new(&sort_fragment).trim().to_string();
            
            write!(sql_buf, "{} {} ", sql, sort_fragment).unwrap();
            std::str::from_utf8(&sql_buf)
                .map_err(|e| 
                    crate::DySqlError(crate::ErrorInner::new(crate::Kind::TemplateParseError, Some(e.into()), None))
                )?
        };
        let stmt = (*executor)
            .prepare(page_sql)
            .await
            .map_err(|e| DySqlError(ErrorInner::new(Kind::PrepareStamentError, Some(Box::new(e)), None)))?;

        let mut param_values : Vec<SimpleValue> = Vec::with_capacity(param_names.len());
        for param_name in &param_names {
            let stpl = SimpleTemplate::new(param_name);
            
            let param_value = stpl
                .apply(page_dto)
                .map_err(|e| DySqlError(ErrorInner::new(Kind::BindParamterError, Some(e), None)))?;
            param_values.push(param_value);
        }

        let mut tosql_values : Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new(); 
        for param_value in &param_values {
            impl_bind_tokio_pg_param_value!(tosql_values, param_value, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
        }

        let params = tosql_values.into_iter();
        let params = params.as_slice();


        let rows = (*executor)
            .query(&stmt, &params)
            .await
            .map_err(|e| DySqlError(ErrorInner::new(Kind::QueryError, Some(Box::new(e)), None)))?;

        let rst = rows
            .iter()
            .map(|row| <U>::from_row_ref(row).expect("query unexpected error"))
            .collect::<Vec<U>>();

        let pg_data = Pagination::from_dto(&page_dto, rst);

        Ok(pg_data)
    }
}

macro_rules! impl_tokio_pg_executor_adapter {
    ( $executor:ty) => {
        #[async_trait]
        impl TokioPgExecutorAdatper for $executor {
            async fn prepare(&self, query: &str) -> Result<Statement, Error> {
                self.prepare(query).await
            }
        
            async fn query<T>(
                &self, 
                statement: &T, 
                params: &[&(dyn ToSql + Sync)]
            ) -> Result<Vec<Row>, Error> 
            where
                T: ?Sized + ToStatement + Sync,
            {
                self.query(statement, params).await
            }
        
            async fn query_one<T>(
                &self,
                statement: &T,
                params: &[&(dyn ToSql + Sync)],
            ) -> Result<Row, Error>
            where
                T: ?Sized + ToStatement + Sync
            {
                self.query_one(statement, params).await
            }
        
            async fn execute<T>(
                &self,
                statement: &T,
                params: &[&(dyn ToSql + Sync)],
            ) -> Result<u64, Error>
            where
                T: ?Sized + ToStatement + Sync
            {
                self.execute(statement, params).await
            }
        }
    }
}

impl_tokio_pg_executor_adapter!(::tokio_postgres::Client);
impl_tokio_pg_executor_adapter!(&::tokio_postgres::Client);
impl_tokio_pg_executor_adapter!(&mut::tokio_postgres::Client);
impl_tokio_pg_executor_adapter!(::tokio_postgres::Transaction<'_>);
impl_tokio_pg_executor_adapter!(&::tokio_postgres::Transaction<'_>);
impl_tokio_pg_executor_adapter!(&mut::tokio_postgres::Transaction<'_>);