
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
            let named_sql = crate::gen_named_sql(named_template, &dto);
                
            let sql_and_params = crate::extract_params(&named_sql, self.dialect);
            let (sql, param_names) = match sql_and_params {
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

            Ok(
                rbatis::decode(rst)
                    .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::ObjectMappingError, Some(e.into()), None)))?
            )
        }
    };
}