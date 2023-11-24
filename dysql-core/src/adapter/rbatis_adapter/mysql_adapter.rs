
pub struct RbatisMysqlQuery1 {
    dialect: crate::SqlDialect
}

impl RbatisMysqlQuery1 {
    pub fn new(dialect: crate::SqlDialect) -> Self {
        Self { dialect }
    }
}

impl RbatisMysqlQuery1 {
    crate::impl_rbatis_adapter_fetch_one!();
    crate::impl_rbatis_adapter_fetch_all!();
    crate::impl_rbatis_adapter_fetch_scalar!();
    crate::impl_rbatis_adapter_execute!();
    crate::impl_rbatis_adapter_page_count!();
    crate::impl_rbatis_adapter_page_all!();

    pub async fn insert<E, D, U>(self, executor: &E, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>)
        -> Result<Option<U>, crate::DySqlError>
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
            .exec(&sql, param_values)
            .await
            .map_err(|e| 
                crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(e.into()), None))
            )?;

        Ok(None)
    }

    pub async fn fetch_insert_id<E, U>(self, executor: &mut E) 
        -> Result<U, crate::DySqlError>
    where
        E: rbatis::executor::Executor,
        U: serde::de::DeserializeOwned,
    {
        let insert_id = executor
            .query("SELECT last_insert_rowid();", vec![])
            .await
            .map_err(|e| 
                crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(e.into()), None))
            )?;

        let insert_id = rbatis::decode(insert_id)
            .map_err(|e| crate::DySqlError(crate::ErrorInner::new(crate::Kind::ObjectMappingError, Some(e.into()), None)))?;

        Ok(insert_id)
    }
}