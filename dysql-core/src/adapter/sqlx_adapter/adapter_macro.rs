
#[macro_export]
macro_rules! impl_sqlx_adapter_fetch_all {
    ($row:path, [$($vtype:ty),+]) => 
    {
        async fn dy_fetch_all<D, U>(self, template_id: u64, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>) 
            -> Result<Vec<U>, crate::DySqlError>
        where
            D: dysql_tpl::Content + Send + Sync,
            for<'r> U: sqlx::FromRow<'r, $row> + Send + Unpin,
        {
            let dialect = self.get_dialect();

            // let sql_data = crate::get_sql_and_params(template_id, named_template.clone(), &dto, dialect)?;
            // let sql = &sql_data.sql;
            // let param_names = &sql_data.param_names;

            let named_sql = crate::gen_named_sql(named_template, &dto)?;
            let mut buf = Vec::<u8>::with_capacity(named_sql.len());
            let sql_and_params = crate::extract_params_buf(&named_sql, &mut buf, dialect);
            let sql = unsafe{std::str::from_utf8_unchecked(&buf)};
            let param_names = match sql_and_params {
                Ok(val) => val,
                Err(e) => Err(
                    crate::DySqlError(crate::ErrorInner::new(crate::Kind::ExtractSqlParamterError, Some(Box::new(e)), None))
                )?,
            };

            let mut query = sqlx::query_as::<_, U>(sql);
            if let Some(dto) = &dto {
                for param_name in param_names {
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

            let rst = query.fetch_all(self).await;

            rst.map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(Box::new(e)), None)))
        }
    };
}

#[macro_export]
macro_rules! impl_sqlx_adapter_fetch_one {
    ($row:path, [$($vtype:ty),+]) => {
        async fn dy_fetch_one<D, U>(self, template_id: u64, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>) 
            -> Result<U, crate::DySqlError>
        where
            D: dysql_tpl::Content + Send + Sync,
            for<'r> U: sqlx::FromRow<'r, $row> + Send + Unpin,
        {
            let named_sql = crate::gen_named_sql(named_template, &dto)?;
            
            let mut buf = Vec::<u8>::with_capacity(named_sql.len());
            let sql_and_params = crate::extract_params_buf(&named_sql, &mut buf, self.get_dialect());
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
    
            let rst = query.fetch_one(self).await;
    
            rst.map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(Box::new(e)), None)))
        }
    };
}

#[macro_export]
macro_rules! impl_sqlx_adapter_fetch_scalar {
    ([$($vtype:ty),+]) => {
        async fn dy_fetch_scalar<D, U>(self, template_id: u64, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>) 
            -> Result<U, crate::DySqlError>
        where
            D: dysql_tpl::Content + Send + Sync,
            for<'r> U: sqlx::Decode<'r, Self::DB> + sqlx::Type<Self::DB> + Send + Unpin
        {
            let named_sql = crate::gen_named_sql(named_template, &dto)?;

            let mut buf = Vec::<u8>::with_capacity(named_sql.len());
            let sql_and_params = crate::extract_params_buf(&named_sql, &mut buf, self.get_dialect());
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

            let rst = query.fetch_one(self).await;

            rst.map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(Box::new(e)), None)))
        }
    };
}


#[macro_export]
macro_rules! impl_sqlx_adapter_execute {
    ([$($vtype:ty),+]) => {
        async fn dy_execute<D>(self, template_id: u64, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>) 
            -> Result<u64, crate::DySqlError>
        where
            D: dysql_tpl::Content + Send + Sync
        {
            let named_sql = crate::gen_named_sql(named_template, &dto)?;

            let mut buf = Vec::<u8>::with_capacity(named_sql.len());
            let sql_and_params = crate::extract_params_buf(&named_sql, &mut buf, self.get_dialect());
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
    
            let rst = query.execute(self).await;
            let rst = rst.map_err(|e| 
                crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(Box::new(e)), None))
            )?;
    
            let af_rows = rst.rows_affected();
            
            Ok(af_rows)
        }
    };
}

#[macro_export]
macro_rules! impl_sqlx_adapter_page_count {
    ([$($vtype:ty),+])  => {
        async fn dy_page_count<D, U>(self, template_id: u64, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>) 
            -> Result<U, crate::DySqlError>
        where
            D: dysql_tpl::Content + Send + Sync,
            for<'r> U: sqlx::Decode<'r, Self::DB> + sqlx::Type<Self::DB> + Send + Unpin
        {
            use std::io::Write;
            let named_sql = crate::gen_named_sql(named_template, &dto)?;
            
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
    
            let rst = query.fetch_one(self).await;
    
            rst.map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(Box::new(e)), None)))
        }        
    };
}

#[macro_export]
macro_rules! impl_sqlx_adapter_page_all {
    ($row:path, [$($vtype:ty),+]) => {
        async fn dy_page_all<D, U>(self, template_id: u64, named_template: std::sync::Arc<dysql_tpl::Template>, page_dto: &crate::PageDto<D>) 
            -> Result<crate::Pagination<U>, crate::DySqlError>
        where
            D: dysql_tpl::Content + Send + Sync,
            for<'r> U: sqlx::FromRow<'r, Self::Row> + Send + Unpin
        {
            use std::io::Write;

            let named_sql= named_template.render_sql(page_dto);
        
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
                let sort_fragment = template.render_sql(page_dto);
                
                write!(sql_buf, "{} {} ", sql, sort_fragment).unwrap();

                let rst = std::str::from_utf8(&sql_buf)
                    .map_err(|e| 
                        crate::DySqlError(crate::ErrorInner::new(crate::Kind::TemplateParseError, Some(e.into()), None))
                    )?;
                rst
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
    
            let rst = query.fetch_all(self).await;
            let rst = match rst {
                Ok(v) => v,
                Err(e) => Err(crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(Box::new(e)), None)))?,
            };
    
            let pg_data = crate::Pagination::from_dto(&page_dto, rst);
    
            Ok(pg_data)
        }
    };
}
