
#[macro_export]
macro_rules! impl_sql_adapter {
    ($db:path, $conn:path) => {
        impl<'q> crate::SqlxExecutorAdatper<$db> for sqlx::Transaction<'q, $db> {}
        impl crate::SqlxExecutorAdatper<$db> for sqlx::Pool<$db> {}
        impl crate::SqlxExecutorAdatper<$db> for &sqlx::Pool<$db> {}
        impl crate::SqlxExecutorAdatper<$db> for $conn {}
        impl crate::SqlxExecutorAdatper<$db> for &mut $conn {}
        // impl crate::SqlxExecutorAdatper<$db> for &std::cell::RefCell<$conn> {}
    };
}

#[macro_export]
macro_rules! impl_sqlx_adapter_page_all {
    ($db:path, $row:path, [$($vtype:ty),+]) => {
        pub async fn page_all<'e, 'c: 'e, E, D, U>(self, executor: E, named_template: std::sync::Arc<dysql_tpl::Template>, page_dto: &crate::PageDto<D>) 
            -> Result<crate::Pagination<U>, crate::DySqlError>
        where
            E: 'e + sqlx::Executor<'c, Database = $db> + crate::SqlxExecutorAdatper<$db>,
            D: dysql_tpl::Content + Send + Sync,
            for<'r> U: sqlx::FromRow<'r, $row> + Send + Unpin,
        {
            use std::io::Write;

            let named_sql = {
                let named_sql= named_template.render(page_dto);
                // 格式化 sql 并解析 BDEL 和 FDEL 指令
                crate::SqlNodeLinkList::new(&named_sql).trim().to_string()
            };
        
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
    
            let mut query = sqlx::query_as::<_, U>(&page_sql);
            for param_name in &param_names {
                let stpl = dysql_tpl::SimpleTemplate::new(param_name);
                
                let param_value = stpl.apply(&page_dto);
                match param_value {
                    Ok(param_value) => {
                        query = impl_bind_sqlx_param_value!(query, param_value, [$($vtype),+]);
                    },
                    Err(e) => Err(crate::DySqlError(crate::ErrorInner::new(crate::Kind::BindParamterError, Some(e), None)))?,
                }
            }
    
            let rst = query.fetch_all(executor).await;
            let rst = match rst {
                Ok(v) => v,
                Err(e) => Err(crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(Box::new(e)), None)))?,
            };
    
            let pg_data = crate::Pagination::from_dto(&page_dto, rst);
    
            Ok(pg_data)
        }
    };
}

#[macro_export]
macro_rules! impl_sqlx_adapter_page_count {
    ($db:path, $row:path, [$($vtype:ty),+])  => {
        pub async fn page_count<'e, 'c: 'e, E, D, U>(self, executor: E, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>) 
            -> Result<U, crate::DySqlError>
        where
            E: 'e + sqlx::Executor<'c, Database = $db> + crate::SqlxExecutorAdatper<$db>,
            D: dysql_tpl::Content + Send + Sync,
            for<'r> U: sqlx::Decode<'r, $db> + sqlx::Type<$db> + Send + Unpin,
        {
            use std::io::Write;

            let named_sql = crate::gen_named_sql(named_template, &dto);
            
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
    
            let mut query = sqlx::query_scalar::<_, U>(&count_sql);
            if let Some(dto) = &dto {
                for param_name in &param_names {
                    let stpl = dysql_tpl::SimpleTemplate::new(param_name);
                    
                    let param_value = stpl.apply(dto);
                    match param_value {
                        Ok(param_value) => {
                            query = impl_bind_sqlx_param_value!(query, param_value, [$($vtype),+]);
                        },
                        Err(e) => Err(crate::DySqlError(crate::ErrorInner::new(crate::Kind::BindParamterError, Some(e), None)))?,
                    }
                }
            }
    
            let rst = query.fetch_one(executor).await;
    
            rst.map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(Box::new(e)), None)))
        }        
    };
}

#[macro_export]
macro_rules! impl_sqlx_adapter_execute {
    ($db:path, $row:path, [$($vtype:ty),+]) => {
        pub async fn execute<'e, 'c: 'e, E, D>(self, executor: E, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>) 
            -> Result<u64, crate::DySqlError>
        where
            E: 'e + sqlx::Executor<'c, Database = $db> + crate::SqlxExecutorAdatper<$db>,
            D: dysql_tpl::Content + Send + Sync,
        {
            let named_sql = crate::gen_named_sql(named_template, &dto);
            let mut buf = Vec::<u8>::with_capacity(named_sql.len());
            let sql_and_params = crate::extract_params_buf(&named_sql, &mut buf, executor.get_dialect());
            let sql = unsafe{std::str::from_utf8_unchecked(&buf)};
            let param_names = match sql_and_params {
                Ok(val) => val,
                Err(e) => Err(
                    crate::DySqlError(crate::ErrorInner::new(crate::Kind::ExtractSqlParamterError, Some(Box::new(e)), None))
                )?,
            };
    
            let mut query = sqlx::query::<_>(&sql);
            if let Some(dto) = &dto {
                for param_name in &param_names {
                    let stpl = dysql_tpl::SimpleTemplate::new(param_name);
                    
                    let param_value = stpl.apply(dto);
                    match param_value {
                        Ok(param_value) => {
                            query = impl_bind_sqlx_param_value!(query, param_value, [$($vtype),+]);
                        },
                        Err(e) => Err(crate::DySqlError(crate::ErrorInner::new(crate::Kind::BindParamterError, Some(e), None)))?,
                    }
                }
            }
    
            let rst = query.execute(executor).await;
            let rst = rst.map_err(|e| 
                crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(Box::new(e)), None))
            )?;
    
            let af_rows = rst.rows_affected();
            
            Ok(af_rows)
        }
    };
}

#[macro_export]
macro_rules! impl_sqlx_adapter_fetch_scalar {
    ($db:path, $row:path, [$($vtype:ty),+]) => {
        pub async fn fetch_scalar<'e, 'c: 'e, E, D, U>(self, executor: E, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>) 
            -> Result<U, crate::DySqlError>
        where
            E: 'e + sqlx::Executor<'c, Database = $db> + crate::SqlxExecutorAdatper<$db>,
            D: dysql_tpl::Content + Send + Sync,
            for<'r> U: sqlx::Decode<'r, $db> + sqlx::Type<$db> + Send + Unpin,
        {
            let named_sql = crate::gen_named_sql(named_template, &dto);
            let mut buf = Vec::<u8>::with_capacity(named_sql.len());
            let sql_and_params = crate::extract_params_buf(&named_sql, &mut buf, executor.get_dialect());
            let sql = unsafe{std::str::from_utf8_unchecked(&buf)};
            let param_names = match sql_and_params {
                Ok(val) => val,
                Err(e) => Err(
                    crate::DySqlError(crate::ErrorInner::new(crate::Kind::ExtractSqlParamterError, Some(Box::new(e)), None))
                )?,
            };

            let mut query = sqlx::query_scalar::<_, U>(&sql);
            if let Some(dto) = &dto {
                for param_name in &param_names {
                    let stpl = dysql_tpl::SimpleTemplate::new(param_name);
                    
                    let param_value = stpl.apply(dto);
                    match param_value {
                        Ok(param_value) => {
                            query = impl_bind_sqlx_param_value!(query, param_value, [$($vtype),+]);
                        },
                        Err(e) => Err(crate::DySqlError(crate::ErrorInner::new(crate::Kind::BindParamterError, Some(e), None)))?,
                    }
                }
            }

            let rst = query.fetch_one(executor).await;

            rst.map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(Box::new(e)), None)))
        }
    };
}

#[macro_export]
macro_rules! impl_sqlx_adapter_fetch_one {
    ($db:path, $row:path, [$($vtype:ty),+]) => {
        pub async fn fetch_one<'e, 'c: 'e, E, D, U>(self, executor: E, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>) 
            -> Result<U, crate::DySqlError>
        where
            E: 'e + sqlx::Executor<'c, Database = $db> + crate::SqlxExecutorAdatper<$db>,
            D: dysql_tpl::Content + Send + Sync,
            for<'r> U: sqlx::FromRow<'r, $row> + Send + Unpin,
        {
            let named_sql = crate::gen_named_sql(named_template, &dto);
            
            let mut buf = Vec::<u8>::with_capacity(named_sql.len());
            let sql_and_params = crate::extract_params_buf(&named_sql, &mut buf, executor.get_dialect());
            let sql = unsafe{std::str::from_utf8_unchecked(&buf)};
            let param_names = match sql_and_params {
                Ok(val) => val,
                Err(e) => Err(
                    crate::DySqlError(crate::ErrorInner::new(crate::Kind::ExtractSqlParamterError, Some(Box::new(e)), None))
                )?,
            };
    
            let mut query = sqlx::query_as::<_, U>(&sql);
            if let Some(dto) = &dto {
                for param_name in &param_names {
                    let stpl = dysql_tpl::SimpleTemplate::new(param_name);
                    
                    let param_value = stpl.apply(dto);
                    match param_value {
                        Ok(param_value) => {
                            
                            query = impl_bind_sqlx_param_value!(query, param_value, [$($vtype),+]);
                        },
                        Err(e) => Err(crate::DySqlError(crate::ErrorInner::new(crate::Kind::BindParamterError, Some(e), None)))?,
                    }
                }
            }
    
            let rst = query.fetch_one(executor).await;
    
            rst.map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(Box::new(e)), None)))
        }
    };
}

#[macro_export]
macro_rules! impl_sqlx_adapter_fetch_all {
    ($db:path, $row:path, [$($vtype:ty),+]) => {
        pub async fn fetch_all<'e, 'c: 'e, E, D, U>(self, executor: E, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>) 
            -> Result<Vec<U>, crate::DySqlError>
        where
            E: 'e + sqlx::Executor<'c, Database = $db> + crate::SqlxExecutorAdatper<$db>,
            D: dysql_tpl::Content + Send + Sync,
            for<'r> U: sqlx::FromRow<'r, $row> + Send + Unpin,
        {
            let named_sql = crate::gen_named_sql(named_template, &dto);
            let mut buf = Vec::<u8>::with_capacity(named_sql.len());
            let sql_and_params = crate::extract_params_buf(&named_sql, &mut buf, executor.get_dialect());
            let sql = unsafe{std::str::from_utf8_unchecked(&buf)};
            let param_names = match sql_and_params {
                Ok(val) => val,
                Err(e) => Err(
                    crate::DySqlError(crate::ErrorInner::new(crate::Kind::ExtractSqlParamterError, Some(Box::new(e)), None))
                )?,
            };

            let mut query = sqlx::query_as::<_, U>(sql);
            if let Some(dto) = &dto {
                for param_name in &param_names {
                    let stpl = dysql_tpl::SimpleTemplate::new(param_name);
                    
                    let param_value = stpl.apply(dto);
                    match param_value {
                        Ok(param_value) => {
                            query = impl_bind_sqlx_param_value!(query, param_value, [$($vtype),+]);
                        },
                        Err(e) => Err(crate::DySqlError(crate::ErrorInner::new(crate::Kind::BindParamterError, Some(e), None)))?,
                    }
                }
            }

            let rst = query.fetch_all(executor).await;

            rst.map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(Box::new(e)), None)))
        }
    };
}