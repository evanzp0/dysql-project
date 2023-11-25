
#[macro_export]
macro_rules! impl_rbatis_adapter_fetch_one {
    () => {
        pub async fn fetch_one<E, D, U>(self, executor: &E, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>)
            -> Result<U, crate::DySqlError>
        where 
            E: rbatis::executor::Executor,
            D: dysql_tpl::Content + Send + Sync,
            U: serde::de::DeserializeOwned,
        {
            let mut named_sql_buf = Vec::<u8>::with_capacity(named_template.source().len());
            let named_sql_buf = crate::gen_named_sql_buf(named_template, named_sql_buf, &dto)?;
            let named_sql = unsafe{std::str::from_utf8_unchecked(&named_sql_buf)};
                
            let mut buf = Vec::<u8>::with_capacity(named_sql.len());
            let sql_and_params = crate::extract_params_buf(&named_sql, &mut buf, self.dialect);
            let sql = unsafe{std::str::from_utf8_unchecked(&buf)};
            let param_names = match sql_and_params {
                Ok(val) => val,
                Err(e) => Err(
                    crate::DySqlError(crate::ErrorInner::new(crate::Kind::ExtractSqlParamterError, Some(Box::new(e)), None))
                )?,
            };

            let mut param_values : Vec<rbs::Value> = Vec::with_capacity(param_names.len());
            if let Some(dto) = &dto {
                for param_name in &param_names {
                    let stpl = dysql_tpl::SimpleTemplate::new(param_name);
                    
                    let param_value = stpl
                        .apply(dto)
                        .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::BindParamterError, Some(e), None)))?;
                    param_values.push(crate::simple_2_value(param_value));
                }
            }

            let rst = executor
                .query(&sql, param_values)
                .await
                .map_err(|e| 
                    crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(e.into()), None))
                )?;

            let rst = rbatis::decode(rst)
                .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::ObjectMappingError, Some(e.into()), None)))?;

            Ok(rst)
        }
    };
}

#[macro_export]
macro_rules! impl_rbatis_adapter_fetch_all {
    () => {
        pub async fn fetch_all<E, D, U>(self, executor: &E, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>)
            -> Result<Vec<U>, crate::DySqlError>
        where 
            E: rbatis::executor::Executor,
            D: dysql_tpl::Content + Send + Sync,
            U: serde::de::DeserializeOwned,
        {
            let mut named_sql_buf = Vec::<u8>::with_capacity(named_template.source().len());
            let named_sql_buf = crate::gen_named_sql_buf(named_template, named_sql_buf, &dto)?;
            let named_sql = unsafe{std::str::from_utf8_unchecked(&named_sql_buf)};
                    
            let mut buf = Vec::<u8>::with_capacity(named_sql.len());
            let sql_and_params = crate::extract_params_buf(&named_sql, &mut buf, self.dialect);
            let sql = unsafe{std::str::from_utf8_unchecked(&buf)};
            let param_names = match sql_and_params {
                Ok(val) => val,
                Err(e) => Err(
                    crate::DySqlError(crate::ErrorInner::new(crate::Kind::ExtractSqlParamterError, Some(Box::new(e)), None))
                )?,
            };

            let mut param_values : Vec<rbs::Value> = Vec::with_capacity(param_names.len());
            if let Some(dto) = &dto {
                for param_name in &param_names {
                    let stpl = dysql_tpl::SimpleTemplate::new(param_name);
                    
                    let param_value = stpl
                        .apply(dto)
                        .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::BindParamterError, Some(e), None)))?;
                    param_values.push(crate::simple_2_value(param_value));
                }
            }

            let rst = executor
                .query(&sql, param_values)
                .await
                .map_err(|e| 
                    crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(e.into()), None))
                )?;

            let rst = rbatis::decode(rst)
                .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::ObjectMappingError, Some(e.into()), None)))?;

            Ok(rst)
        }
    };
}

#[macro_export]
macro_rules! impl_rbatis_adapter_fetch_scalar {
    () => {
        pub async fn fetch_scalar<E, D, U>(self, executor: &E, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>)
            -> Result<U, crate::DySqlError>
        where 
            E: rbatis::executor::Executor,
            D: dysql_tpl::Content + Send + Sync,
            U: serde::de::DeserializeOwned,
        {
            let mut named_sql_buf = Vec::<u8>::with_capacity(named_template.source().len());
            let named_sql_buf = crate::gen_named_sql_buf(named_template, named_sql_buf, &dto)?;
            let named_sql = unsafe{std::str::from_utf8_unchecked(&named_sql_buf)};
                
            let mut buf = Vec::<u8>::with_capacity(named_sql.len());
            let sql_and_params = crate::extract_params_buf(&named_sql, &mut buf, self.dialect);
            let sql = unsafe{std::str::from_utf8_unchecked(&buf)};
            let param_names = match sql_and_params {
                Ok(val) => val,
                Err(e) => Err(
                    crate::DySqlError(crate::ErrorInner::new(crate::Kind::ExtractSqlParamterError, Some(Box::new(e)), None))
                )?,
            };

            let mut param_values : Vec<rbs::Value> = Vec::with_capacity(param_names.len());
            if let Some(dto) = &dto {
                for param_name in &param_names {
                    let stpl = dysql_tpl::SimpleTemplate::new(param_name);
                    
                    let param_value = stpl
                        .apply(dto)
                        .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::BindParamterError, Some(e), None)))?;
                    param_values.push(crate::simple_2_value(param_value));
                }
            }

            let rst = executor
                .query(&sql, param_values)
                .await
                .map_err(|e| 
                    crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(e.into()), None))
                )?;

            let rst = rbatis::decode(rst)
                .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::ObjectMappingError, Some(e.into()), None)))?;

            Ok(rst)
        }
    }
}

#[macro_export]
macro_rules! impl_rbatis_adapter_execute {
    () => {
        pub async fn execute<E, D>(self, executor: &E, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>)
            -> Result<u64, crate::DySqlError>
        where 
            E: rbatis::executor::Executor,
            D: dysql_tpl::Content + Send + Sync,
        {
            let mut named_sql_buf = Vec::<u8>::with_capacity(named_template.source().len());
            let named_sql_buf = crate::gen_named_sql_buf(named_template, named_sql_buf, &dto)?;
            let named_sql = unsafe{std::str::from_utf8_unchecked(&named_sql_buf)};
                
            let mut buf = Vec::<u8>::with_capacity(named_sql.len());
            let sql_and_params = crate::extract_params_buf(&named_sql, &mut buf, self.dialect);
            let sql = unsafe{std::str::from_utf8_unchecked(&buf)};
            let param_names = match sql_and_params {
                Ok(val) => val,
                Err(e) => Err(
                    crate::DySqlError(crate::ErrorInner::new(crate::Kind::ExtractSqlParamterError, Some(Box::new(e)), None))
                )?,
            };

            let mut param_values : Vec<rbs::Value> = Vec::with_capacity(param_names.len());
            if let Some(dto) = &dto {
                for param_name in &param_names {
                    let stpl = dysql_tpl::SimpleTemplate::new(param_name);
                    
                    let param_value = stpl
                        .apply(dto)
                        .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::BindParamterError, Some(e), None)))?;
                    param_values.push(crate::simple_2_value(param_value));
                }
            }

            let rst = executor
                .exec(&sql, param_values)
                .await
                .map_err(|e| 
                    crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(e.into()), None))
                )?;

            Ok(rst.rows_affected)
        }
    }
}

#[macro_export]
macro_rules! impl_rbatis_adapter_page_count {
    () => {
        pub async fn page_count<E, D, U>(self, executor: &E, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>)
            -> Result<U, crate::DySqlError>
        where 
            E: rbatis::executor::Executor,
            D: dysql_tpl::Content + Send + Sync,
            U: serde::de::DeserializeOwned,
        {
            use std::io::Write;

            let mut named_sql_buf = Vec::<u8>::with_capacity(named_template.source().len());
            let named_sql_buf = crate::gen_named_sql_buf(named_template, named_sql_buf, &dto)?;
            let named_sql = unsafe{std::str::from_utf8_unchecked(&named_sql_buf)};
                
            let mut buf = Vec::<u8>::with_capacity(named_sql.len());
            let sql_and_params = crate::extract_params_buf(&named_sql, &mut buf, self.dialect);
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

            let mut param_values : Vec<rbs::Value> = Vec::with_capacity(param_names.len());
            if let Some(dto) = &dto {
                for param_name in &param_names {
                    let stpl = dysql_tpl::SimpleTemplate::new(param_name);
                    
                    let param_value = stpl
                        .apply(dto)
                        .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::BindParamterError, Some(e), None)))?;
                    param_values.push(crate::simple_2_value(param_value));
                }
            }

            let rst = executor
                .query(&count_sql, param_values)
                .await
                .map_err(|e| 
                    crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(e.into()), None))
                )?;

            let rst = rbatis::decode(rst)
                .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::ObjectMappingError, Some(e.into()), None)))?;

            Ok(rst)
        }
    };
}

#[macro_export]
macro_rules! impl_rbatis_adapter_page_all {
    () => {
        pub async fn page_all<E, D, U>(self, executor: &E, named_template: std::sync::Arc<dysql_tpl::Template>, page_dto: &crate::PageDto<D>) 
            -> Result<crate::Pagination<U>, crate::DySqlError>
        where 
            E: rbatis::executor::Executor,
            D: dysql_tpl::Content + Send + Sync,
            U: serde::de::DeserializeOwned,
        {
            use std::io::Write;

            let named_sql= named_template.render(page_dto);
            let sql_buf: Vec<u8> = Vec::with_capacity(named_sql.len());
            let sql_buf = crate::trim_sql(&named_sql, sql_buf).unwrap();
            let named_sql = unsafe { std::str::from_utf8_unchecked(&sql_buf) };
            
            let mut buf = Vec::<u8>::with_capacity(named_sql.len());
            let sql_and_params = crate::extract_params_buf(&named_sql, &mut buf, self.dialect);
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

            let mut param_values : Vec<rbs::Value> = Vec::with_capacity(param_names.len());
            for param_name in &param_names {
                let stpl = dysql_tpl::SimpleTemplate::new(param_name);
                
                let param_value = stpl
                    .apply(page_dto)
                    .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::BindParamterError, Some(e), None)))?;
                param_values.push(crate::simple_2_value(param_value));
            }

            let rst = executor
                .query(&sql, param_values)
                .await
                .map_err(|e| 
                    crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(e.into()), None))
                )?;

            let rst = rbatis::decode(rst)
                .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::ObjectMappingError, Some(e.into()), None)))?;

            let pg_data = crate::Pagination::from_dto(&page_dto, rst);

            Ok(pg_data)
        }
    };
}