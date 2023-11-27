
pub struct RbatisSqliteAdapter{
    dialect: crate::SqlDialect
}

impl RbatisSqliteAdapter {
    pub fn new(dialect: crate::SqlDialect) -> Self {
        Self { dialect }
    }
}

impl RbatisSqliteAdapter {
    crate::impl_rbatis_adapter_fetch_one!();
    crate::impl_rbatis_adapter_fetch_all!();
    crate::impl_rbatis_adapter_fetch_scalar!();
    crate::impl_rbatis_adapter_execute!();
    crate::impl_rbatis_adapter_page_count!();
    crate::impl_rbatis_adapter_page_all!();

    pub async fn dy_insert<E, D, U>(self, executor: &E, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>)
        -> Result<Option<U>, crate::DySqlError>
    where 
        E: rbatis::executor::Executor,
        D: dysql_tpl::Content + Send + Sync,
        U: serde::de::DeserializeOwned,
    {
        let named_sql = crate::gen_named_sql(named_template, &dto)?;
            
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

        Ok(None)
    }

    pub async fn dy_fetch_insert_id<E, U>(self, executor: &E) 
        -> Result<Option<U>, crate::DySqlError>
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

        Ok(Some(insert_id))
    }
}