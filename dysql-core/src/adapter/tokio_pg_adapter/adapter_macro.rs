#[macro_export]
macro_rules! impl_tokio_pg_adapter_fetch_all {
    ([$($vtype:ty),+]) => 
    {
        async fn dy_fetch_all<D, U>(self, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>)
            -> Result<Vec<U>, crate::DySqlError>
        where 
            D: dysql_tpl::Content + Send + Sync,
            U: tokio_pg_mapper::FromTokioPostgresRow,
        {
            let mut named_sql_buf = Vec::<u8>::with_capacity(named_template.source().len());
            let named_sql_buf = crate::gen_named_sql_buf(named_template, named_sql_buf, &dto)?;
            let named_sql = unsafe{std::str::from_utf8_unchecked(&named_sql_buf)};

            let mut buf = Vec::<u8>::with_capacity(named_sql.len());
            let sql_and_params = crate::extract_params_buf(&named_sql, &mut buf, self.get_dialect());
            let sql = unsafe{std::str::from_utf8_unchecked(&buf)};
            let param_names = match sql_and_params {
                Ok(val) => val,
                Err(e) => Err(
                    crate::DySqlError(crate::ErrorInner::new(crate::Kind::ExtractSqlParamterError, Some(Box::new(e)), None))
                )?,
            };
            let stmt = self
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
                impl_bind_tokio_pg_param_value!(tosql_values, param_value, [$($vtype),+]);
            }

            let params = tosql_values.into_iter();
            let params = params.as_slice();


            let rows = self
                .query(&stmt, &params)
                .await
                .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(Box::new(e)), None)))?;

            let rst = rows
                .iter()
                .map(|row| <U>::from_row_ref(row).expect("query unexpected error"))
                .collect::<Vec<U>>();

            Ok(rst)
        }
    };
}

#[macro_export]
macro_rules! impl_tokio_pg_adapter_fetch_one {
    ([$($vtype:ty),+]) => 
    {
        async fn dy_fetch_one<D, U>(self, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>)
            -> Result<U, crate::DySqlError>
        where 
            D: dysql_tpl::Content + Send + Sync,
            U: tokio_pg_mapper::FromTokioPostgresRow,
        {
            let mut named_sql_buf = Vec::<u8>::with_capacity(named_template.source().len());
            let named_sql_buf = crate::gen_named_sql_buf(named_template, named_sql_buf, &dto)?;
            let named_sql = unsafe{std::str::from_utf8_unchecked(&named_sql_buf)};

            let mut buf = Vec::<u8>::with_capacity(named_sql.len());
            let sql_and_params = crate::extract_params_buf(&named_sql, &mut buf, self.get_dialect());
            let sql = unsafe{std::str::from_utf8_unchecked(&buf)};
            let param_names = match sql_and_params {
                Ok(val) => val,
                Err(e) => Err(
                    crate::DySqlError(crate::ErrorInner::new(crate::Kind::ExtractSqlParamterError, Some(Box::new(e)), None))
                )?,
            };
            let stmt = self
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
                impl_bind_tokio_pg_param_value!(tosql_values, param_value, [$($vtype),+]);
            }

            let params = tosql_values.into_iter();
            let params = params.as_slice();
            let row = self
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
    };
}

#[macro_export]
macro_rules! impl_tokio_pg_adapter_fetch_scalar {
    ([$($vtype:ty),+]) => 
    {
        async fn dy_fetch_scalar<D, U>(self, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>)
            -> Result<U, crate::DySqlError>
        where 
            D: dysql_tpl::Content + Send + Sync,
            for<'a> U: tokio_postgres::types::FromSql<'a>,
        {
            let mut named_sql_buf = Vec::<u8>::with_capacity(named_template.source().len());
            let named_sql_buf = crate::gen_named_sql_buf(named_template, named_sql_buf, &dto)?;
            let named_sql = unsafe{std::str::from_utf8_unchecked(&named_sql_buf)};
            
            let mut buf = Vec::<u8>::with_capacity(named_sql.len());
            let sql_and_params = crate::extract_params_buf(&named_sql, &mut buf, self.get_dialect());
            let sql = unsafe{std::str::from_utf8_unchecked(&buf)};
            let param_names = match sql_and_params {
                Ok(val) => val,
                Err(e) => Err(
                    crate::DySqlError(crate::ErrorInner::new(crate::Kind::ExtractSqlParamterError, Some(Box::new(e)), None))
                )?,
            };
            let stmt = self
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
                impl_bind_tokio_pg_param_value!(tosql_values, param_value, [$($vtype),+]);
            }

            let params = tosql_values.into_iter();
            let params = params.as_slice();

            let row = self
                .query_one(&stmt, &params)
                .await
                .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(Box::new(e)), None)))?;

            let rst: U = row.get(0);

            Ok(rst)
        }
    };
}

#[macro_export]
macro_rules! impl_tokio_pg_adapter_execute {
    ([$($vtype:ty),+]) => 
    {
        async fn dy_execute<D>(self, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>)
            -> Result<u64, crate::DySqlError>
        where 
            D: dysql_tpl::Content + Send + Sync,
        {
            let mut named_sql_buf = Vec::<u8>::with_capacity(named_template.source().len());
            let named_sql_buf = crate::gen_named_sql_buf(named_template, named_sql_buf, &dto)?;
            let named_sql = unsafe{std::str::from_utf8_unchecked(&named_sql_buf)};

            let mut buf = Vec::<u8>::with_capacity(named_sql.len());
            let sql_and_params = crate::extract_params_buf(&named_sql, &mut buf, self.get_dialect());
            let sql = unsafe{std::str::from_utf8_unchecked(&buf)};
            let param_names = match sql_and_params {
                Ok(val) => val,
                Err(e) => Err(
                    crate::DySqlError(crate::ErrorInner::new(crate::Kind::ExtractSqlParamterError, Some(Box::new(e)), None))
                )?,
            };
            let stmt = self
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
                impl_bind_tokio_pg_param_value!(tosql_values, param_value, [$($vtype),+]);
            }

            let params = tosql_values.into_iter();
            let params = params.as_slice();

            let affect_count = self
                .execute(&stmt, &params)
                .await
                .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(Box::new(e)), None)))?;

            Ok(affect_count)
        }
    };
}

#[macro_export]
macro_rules! impl_tokio_pg_adapter_insert {
    ([$($vtype:ty),+]) => 
    {
        async fn dy_insert<D, U>(self, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>)
            -> Result<Option<U>, crate::DySqlError>
        where 
            D: dysql_tpl::Content + Send + Sync,
            for<'a> U: tokio_postgres::types::FromSql<'a>,
        {
            let mut named_sql_buf = Vec::<u8>::with_capacity(named_template.source().len());
            let named_sql_buf = crate::gen_named_sql_buf(named_template, named_sql_buf, &dto)?;
            let named_sql = unsafe{std::str::from_utf8_unchecked(&named_sql_buf)};

            let mut buf = Vec::<u8>::with_capacity(named_sql.len());
            let sql_and_params = crate::extract_params_buf(&named_sql, &mut buf, self.get_dialect());
            let sql = unsafe{std::str::from_utf8_unchecked(&buf)};
            let param_names = match sql_and_params {
                Ok(val) => val,
                Err(e) => Err(
                    crate::DySqlError(crate::ErrorInner::new(crate::Kind::ExtractSqlParamterError, Some(Box::new(e)), None))
                )?,
            };
            let stmt = self
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
                impl_bind_tokio_pg_param_value!(tosql_values, param_value, [$($vtype),+]);
            }
            let params = tosql_values.into_iter();
            let params = params.as_slice();
            
            let row = self
                .query_one(&stmt, &params)
                .await;
            let row = row.unwrap();
            let rst: U = row.get(0);

            Ok(Some(rst))
        }
    }
}

#[macro_export]
macro_rules! impl_tokio_pg_adapter_fetch_insert_id {
    () => 
    {
        async fn dy_fetch_insert_id<U>(self)
            -> Result<Option<U>, crate::DySqlError>
        where 
            for<'a> U: tokio_postgres::types::FromSql<'a>,
        {
            Ok(None)
        }
    }
}

#[macro_export]
macro_rules! impl_tokio_pg_adapter_page_count {
    ([$($vtype:ty),+]) => 
    {
        async fn dy_page_count<D, U>(self, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>)
            -> Result<U, crate::DySqlError>
        where 
            D: dysql_tpl::Content + Send + Sync,
            for<'a> U: tokio_postgres::types::FromSql<'a>
        {
            use std::io::Write;

            let mut named_sql_buf = Vec::<u8>::with_capacity(named_template.source().len());
            let named_sql_buf = crate::gen_named_sql_buf(named_template, named_sql_buf, &dto)?;
            let named_sql = unsafe{std::str::from_utf8_unchecked(&named_sql_buf)};
            
            let mut buf = Vec::<u8>::with_capacity(named_sql.len());
            let sql_and_params = crate::extract_params_buf(&named_sql, &mut buf, self.get_dialect());
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

            let stmt = self
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
                impl_bind_tokio_pg_param_value!(tosql_values, param_value, [$($vtype),+]);
            }

            let params = tosql_values.into_iter();
            let params = params.as_slice();

            let row = self
                .query_one(&stmt, &params)
                .await
                .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(Box::new(e)), None)))?;

            let rst: U = row.get(0);

            Ok(rst)
        }
    }
}

#[macro_export]
macro_rules! impl_tokio_pg_adapter_page_all {
    ([$($vtype:ty),+]) => 
    {
        async fn dy_page_all<D, U>(self, named_template: std::sync::Arc<dysql_tpl::Template>, page_dto: &crate::PageDto<D>)
            -> Result<crate::Pagination<U>, crate::DySqlError>
        where 
            D: dysql_tpl::Content + Send + Sync,
            U: tokio_pg_mapper::FromTokioPostgresRow
        {   
            use std::io::Write;

            let named_sql= named_template.render(page_dto);
            let sql_buf: Vec<u8> = Vec::with_capacity(named_sql.len());
            let sql_buf = crate::trim_sql(&named_sql, sql_buf).unwrap();
            let named_sql = unsafe { std::str::from_utf8_unchecked(&sql_buf) };
            
            let mut buf = Vec::<u8>::with_capacity(named_sql.len());
            let sql_and_params = crate::extract_params_buf(&named_sql, &mut buf, self.get_dialect());
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
            let stmt = self
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
                impl_bind_tokio_pg_param_value!(tosql_values, param_value, [$($vtype),+]);
            }

            let params = tosql_values.into_iter();
            let params = params.as_slice();


            let rows = self
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
}