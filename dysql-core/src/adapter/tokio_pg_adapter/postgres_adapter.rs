use async_trait::async_trait;

impl crate::TokioPgQuery
{
    /// named_sql: 是已经代入 dto 进行模版 render 后的 named sql 
    pub async fn fetch_one<E, D, U>(self, executor: &E, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>)
        -> Result<U, crate::DySqlError>
    where 
        E: crate::TokioPgExecutorAdatper,
        D: dysql_tpl::Content + Send + Sync,
        U: tokio_pg_mapper::FromTokioPostgresRow,
    {
        let mut named_sql_buf = Vec::<u8>::with_capacity(named_template.source().len());
        let named_sql_buf = crate::gen_named_sql_buf(named_template, named_sql_buf, &dto)?;
        let named_sql = unsafe{std::str::from_utf8_unchecked(&named_sql_buf)};

        let mut buf = Vec::<u8>::with_capacity(named_sql.len());
        let sql_and_params = crate::extract_params_buf(&named_sql, &mut buf, executor.get_dialect());
        let sql = unsafe{std::str::from_utf8_unchecked(&buf)};
        let param_names = match sql_and_params {
            Ok(val) => val,
            Err(e) => Err(
                crate::DySqlError(crate::ErrorInner::new(crate::Kind::ExtractSqlParamterError, Some(Box::new(e)), None))
            )?,
        };
        let stmt = (*executor)
            .prepare(&sql)
            .await
            .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::PrepareStamentError, Some(Box::new(e)), None)))?;

        let mut param_values : Vec<dysql_tpl::SimpleValue> = Vec::with_capacity(param_names.len());
        if let Some(dto) = &dto {
            for param_name in &param_names {
                let stpl = dysql_tpl::SimpleTemplate::new(param_name);
                
                let param_value = stpl
                    .apply(dto)
                    .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::BindParamterError, Some(e), None)))?;
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
                    crate::DySqlError(crate::ErrorInner::new(crate::Kind::RecordNotFound, Some(Box::new(e)), None))
                } else {
                    crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(Box::new(e)), None))
                }
            })?;
        let rst = <U>::from_row(row)
            .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::ObjectMappingError, Some(Box::new(e)), None)))?;

        Ok(rst)
    }

    pub async fn fetch_all<E, D, U>(self, executor: &E, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>)
        -> Result<Vec<U>, crate::DySqlError>
    where 
        E: crate::TokioPgExecutorAdatper,
        D: dysql_tpl::Content + Send + Sync,
        U: tokio_pg_mapper::FromTokioPostgresRow,
    {
        let mut named_sql_buf = Vec::<u8>::with_capacity(named_template.source().len());
        let named_sql_buf = crate::gen_named_sql_buf(named_template, named_sql_buf, &dto)?;
        let named_sql = unsafe{std::str::from_utf8_unchecked(&named_sql_buf)};

        let mut buf = Vec::<u8>::with_capacity(named_sql.len());
        let sql_and_params = crate::extract_params_buf(&named_sql, &mut buf, executor.get_dialect());
        let sql = unsafe{std::str::from_utf8_unchecked(&buf)};
        let param_names = match sql_and_params {
            Ok(val) => val,
            Err(e) => Err(
                crate::DySqlError(crate::ErrorInner::new(crate::Kind::ExtractSqlParamterError, Some(Box::new(e)), None))
            )?,
        };
        let stmt = (*executor)
            .prepare(&sql)
            .await
            .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::PrepareStamentError, Some(Box::new(e)), None)))?;

        let mut param_values : Vec<dysql_tpl::SimpleValue> = Vec::with_capacity(param_names.len());
        if let Some(dto) = &dto {
            for param_name in &param_names {
                let stpl = dysql_tpl::SimpleTemplate::new(param_name);
                
                let param_value = stpl
                    .apply(dto)
                    .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::BindParamterError, Some(e), None)))?;
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
            .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(Box::new(e)), None)))?;

        let rst = rows
            .iter()
            .map(|row| <U>::from_row_ref(row).expect("query unexpected error"))
            .collect::<Vec<U>>();

        Ok(rst)
    }

    pub async fn fetch_scalar<E, D, U>(self, executor: &E, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>)
        -> Result<U, crate::DySqlError>
    where 
        E: crate::TokioPgExecutorAdatper,
        D: dysql_tpl::Content + Send + Sync,
        for<'a> U: tokio_postgres::types::FromSql<'a>,
    {
        let mut named_sql_buf = Vec::<u8>::with_capacity(named_template.source().len());
        let named_sql_buf = crate::gen_named_sql_buf(named_template, named_sql_buf, &dto)?;
        let named_sql = unsafe{std::str::from_utf8_unchecked(&named_sql_buf)};
        
        let mut buf = Vec::<u8>::with_capacity(named_sql.len());
        let sql_and_params = crate::extract_params_buf(&named_sql, &mut buf, executor.get_dialect());
        let sql = unsafe{std::str::from_utf8_unchecked(&buf)};
        let param_names = match sql_and_params {
            Ok(val) => val,
            Err(e) => Err(
                crate::DySqlError(crate::ErrorInner::new(crate::Kind::ExtractSqlParamterError, Some(Box::new(e)), None))
            )?,
        };
        let stmt = (*executor)
            .prepare(&sql)
            .await
            .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::PrepareStamentError, Some(Box::new(e)), None)))?;

        let mut param_values : Vec<dysql_tpl::SimpleValue> = Vec::with_capacity(param_names.len());
        if let Some(dto) = &dto {
            for param_name in &param_names {
                let stpl = dysql_tpl::SimpleTemplate::new(param_name);
                
                let param_value = stpl
                    .apply(dto)
                    .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::BindParamterError, Some(e), None)))?;
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
            .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(Box::new(e)), None)))?;

        let rst: U = row.get(0);

        Ok(rst)
    }
    
    pub async fn execute<E, D>(self, executor: &E, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>)
        -> Result<u64, crate::DySqlError>
    where 
        E: crate::TokioPgExecutorAdatper,
        D: dysql_tpl::Content + Send + Sync,
    {
        let mut named_sql_buf = Vec::<u8>::with_capacity(named_template.source().len());
        let named_sql_buf = crate::gen_named_sql_buf(named_template, named_sql_buf, &dto)?;
        let named_sql = unsafe{std::str::from_utf8_unchecked(&named_sql_buf)};

        let mut buf = Vec::<u8>::with_capacity(named_sql.len());
        let sql_and_params = crate::extract_params_buf(&named_sql, &mut buf, executor.get_dialect());
        let sql = unsafe{std::str::from_utf8_unchecked(&buf)};
        let param_names = match sql_and_params {
            Ok(val) => val,
            Err(e) => Err(
                crate::DySqlError(crate::ErrorInner::new(crate::Kind::ExtractSqlParamterError, Some(Box::new(e)), None))
            )?,
        };
        let stmt = (*executor)
            .prepare(&sql)
            .await
            .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::PrepareStamentError, Some(Box::new(e)), None)))?;

        let mut param_values : Vec<dysql_tpl::SimpleValue> = Vec::with_capacity(param_names.len());
        if let Some(dto) = &dto {
            for param_name in &param_names {
                let stpl = dysql_tpl::SimpleTemplate::new(param_name);
                
                let param_value = stpl
                    .apply(dto)
                    .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::BindParamterError, Some(e), None)))?;
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
            .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(Box::new(e)), None)))?;

        Ok(affect_count)
    }

    pub async fn insert<E, D, U>(self, executor: &E, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>)
        -> Result<Option<U>, crate::DySqlError>
    where 
        E: crate::TokioPgExecutorAdatper,
        D: dysql_tpl::Content + Send + Sync,
        for<'a> U: tokio_postgres::types::FromSql<'a>,
    {
        let mut named_sql_buf = Vec::<u8>::with_capacity(named_template.source().len());
        let named_sql_buf = crate::gen_named_sql_buf(named_template, named_sql_buf, &dto)?;
        let named_sql = unsafe{std::str::from_utf8_unchecked(&named_sql_buf)};

        let mut buf = Vec::<u8>::with_capacity(named_sql.len());
        let sql_and_params = crate::extract_params_buf(&named_sql, &mut buf, executor.get_dialect());
        let sql = unsafe{std::str::from_utf8_unchecked(&buf)};
        let param_names = match sql_and_params {
            Ok(val) => val,
            Err(e) => Err(
                crate::DySqlError(crate::ErrorInner::new(crate::Kind::ExtractSqlParamterError, Some(Box::new(e)), None))
            )?,
        };
        let stmt = (*executor)
            .prepare(&sql)
            .await
            .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::PrepareStamentError, Some(Box::new(e)), None)))?;

        let mut param_values : Vec<dysql_tpl::SimpleValue> = Vec::with_capacity(param_names.len());
        if let Some(dto) = &dto {
            for param_name in &param_names {
                let stpl = dysql_tpl::SimpleTemplate::new(param_name);
                
                let param_value = stpl
                    .apply(dto)
                    .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::BindParamterError, Some(e), None)))?;
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

    pub async fn fetch_insert_id<E>(self, _executor: E) -> Result<i64, crate::DySqlError>
    {
        Ok(-1)
    }

    pub async fn page_count<E, D, U>(self, executor: &E, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>)
        -> Result<U, crate::DySqlError>
    where 
        E: crate::TokioPgExecutorAdatper,
        D: dysql_tpl::Content + Send + Sync,
        for<'a> U: tokio_postgres::types::FromSql<'a>,
    {
        use std::io::Write;

        let mut named_sql_buf = Vec::<u8>::with_capacity(named_template.source().len());
        let named_sql_buf = crate::gen_named_sql_buf(named_template, named_sql_buf, &dto)?;
        let named_sql = unsafe{std::str::from_utf8_unchecked(&named_sql_buf)};
        
        let mut buf = Vec::<u8>::with_capacity(named_sql.len());
        let sql_and_params = crate::extract_params_buf(&named_sql, &mut buf, executor.get_dialect());
        let sql = unsafe{std::str::from_utf8_unchecked(&buf)};
        let param_names = match sql_and_params {
            Ok(val) => val,
            Err(e) => Err(
                crate::DySqlError(crate::ErrorInner::new(crate::Kind::ExtractSqlParamterError, Some(Box::new(e)), None))
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
            .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::PrepareStamentError, Some(Box::new(e)), None)))?;

        let mut param_values : Vec<dysql_tpl::SimpleValue> = Vec::with_capacity(param_names.len());
        if let Some(dto) = &dto {
            for param_name in &param_names {
                let stpl = dysql_tpl::SimpleTemplate::new(param_name);
                
                let param_value = stpl
                    .apply(dto)
                    .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::BindParamterError, Some(e), None)))?;
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
            .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(Box::new(e)), None)))?;

        let rst: U = row.get(0);

        Ok(rst)
    }

    pub async fn page_all<E, D, U>(self, executor: &E, named_template: std::sync::Arc<dysql_tpl::Template>, page_dto: &crate::PageDto<D>)
        -> Result<crate::Pagination<U>, crate::DySqlError>
    where 
        E: crate::TokioPgExecutorAdatper,
        D: dysql_tpl::Content + Send + Sync,
        U: tokio_pg_mapper::FromTokioPostgresRow,
    {   
        use std::io::Write;

        let named_sql= named_template.render(page_dto);
        let sql_buf: Vec<u8> = Vec::with_capacity(named_sql.len());
        let sql_buf = crate::trim_sql(&named_sql, sql_buf).unwrap();
        let named_sql = unsafe { std::str::from_utf8_unchecked(&sql_buf) };
        
        let mut buf = Vec::<u8>::with_capacity(named_sql.len());
        let sql_and_params = crate::extract_params_buf(&named_sql, &mut buf, executor.get_dialect());
        let sql = unsafe{std::str::from_utf8_unchecked(&buf)};
        let param_names = match sql_and_params {
            Ok(val) => val,
            Err(e) => Err(
                crate::DySqlError(crate::ErrorInner::new(crate::Kind::ExtractSqlParamterError, Some(Box::new(e)), None))
            )?,
        };

        let buffer_size = sql.len() + 200;
        let mut sql_buf = Vec::<u8>::with_capacity(buffer_size);
        let page_sql = {
            let sort_fragment = "{{#is_sort}} ORDER BY ![DEL(,)] {{#sort_model}} , {{field}} {{sort}} {{/sort_model}} {{/is_sort}} LIMIT {{page_size}} OFFSET {{start}}";
            let template = dysql_tpl::Template::new(sort_fragment)
                .map_err(|e| 
                    crate::DySqlError(crate::ErrorInner::new(crate::Kind::TemplateParseError, Some(e.into()), None))
                )?;
            let sort_fragment = template.render(page_dto);

            let sort_fragment_buf: Vec<u8> = Vec::with_capacity(sort_fragment.len());
            let sort_fragment_buf = crate::trim_sql(&sort_fragment, sort_fragment_buf).unwrap();
            let sort_fragment = unsafe { std::str::from_utf8_unchecked(&sort_fragment_buf) };
            
            write!(sql_buf, "{} {} ", sql, sort_fragment).unwrap();
            std::str::from_utf8(&sql_buf)
                .map_err(|e| 
                    crate::DySqlError(crate::ErrorInner::new(crate::Kind::TemplateParseError, Some(e.into()), None))
                )?
        };
        let stmt = (*executor)
            .prepare(page_sql)
            .await
            .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::PrepareStamentError, Some(Box::new(e)), None)))?;

        let mut param_values : Vec<dysql_tpl::SimpleValue> = Vec::with_capacity(param_names.len());
        for param_name in &param_names {
            let stpl = dysql_tpl::SimpleTemplate::new(param_name);
            
            let param_value = stpl
                .apply(page_dto)
                .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::BindParamterError, Some(e), None)))?;
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
            .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(Box::new(e)), None)))?;

        let rst = rows
            .iter()
            .map(|row| <U>::from_row_ref(row).expect("query unexpected error"))
            .collect::<Vec<U>>();

        let pg_data = crate::Pagination::from_dto(&page_dto, rst);

        Ok(pg_data)
    }
}

macro_rules! impl_tokio_pg_executor_adapter {
    ( $executor:ty) => {
        #[async_trait]
        impl crate::TokioPgExecutorAdatper for $executor {
            async fn prepare(&self, query: &str) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.prepare(query).await
            }
        
            async fn query<T>(
                &self, 
                statement: &T, 
                params: &[&(dyn tokio_postgres::types::ToSql + Sync)]
            ) -> Result<Vec<tokio_postgres::Row>, tokio_postgres::Error> 
            where
                T: ?Sized + tokio_postgres::ToStatement + Sync,
            {
                self.query(statement, params).await
            }
        
            async fn query_one<T>(
                &self,
                statement: &T,
                params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
            ) -> Result<tokio_postgres::Row, tokio_postgres::Error>
            where
                T: ?Sized + tokio_postgres::ToStatement + Sync
            {
                self.query_one(statement, params).await
            }
        
            async fn execute<T>(
                &self,
                statement: &T,
                params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
            ) -> Result<u64, tokio_postgres::Error>
            where
                T: ?Sized + tokio_postgres::ToStatement + Sync
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