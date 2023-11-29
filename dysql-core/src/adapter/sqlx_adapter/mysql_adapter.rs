use crate::SqlxExecutorAdatper;

macro_rules! impl_sqlx_adapter_mysql_insert {
    ([$($vtype:ty),+]) => 
    {
        async fn dy_insert<D, U>(self, template_id: u64, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>) 
            -> Result<Option<U>, crate::DySqlError>
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
            let mut query = sqlx::query(&sql);
            if let Some(dto) = &dto {
                for param_name in &param_names {
                    let stpl = dysql_tpl::SimpleTemplate::new(param_name);
                    
                    let param_value = stpl.apply(dto);
                    match param_value {
                        Ok(param_value) => {
                            query = impl_bind_sqlx_param_value!(query, param_value, [$($vtype),+]);
                        },
                        Err(e) => Err( crate::DySqlError( crate::ErrorInner::new( crate::Kind::BindParamterError, Some(e), None)))?,
                    }
                }
            }

            let rst = query.execute(self).await;
            match rst {
                // 返回 None 让外层继续调用 fetch_insert_id()
                Ok(_) => Ok(None),
                Err(e) => Err(crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(Box::new(e)), None))),
            }
        }
    };
}

macro_rules! impl_sqlx_adapter_mysql_fetch_insert_id {
    ([$($vtype:ty),+]) => 
    {
        async fn dy_fetch_insert_id<U>(self)
            -> Result<Option<U>, crate::DySqlError>
        where
            for<'r> U: sqlx::Decode<'r, Self::DB> + sqlx::Type<Self::DB> + Send + Unpin
        {
            let insert_id = sqlx::query_as::<_, (U,)>("SELECT LAST_INSERT_ID();")
                .fetch_one(self)
                .await;

            match insert_id {
                Ok(insert_id) => Ok(Some(insert_id.0)),
                Err(e) => Err(crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(Box::new(e)), None))),
            }
        }
    };
}

impl SqlxExecutorAdatper for &sqlx::Pool<sqlx::MySql> {

    type DB = sqlx::MySql;

    type Row = sqlx::mysql::MySqlRow;

    crate::impl_sqlx_adapter_fetch_all!(sqlx::mysql::MySqlRow, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
    crate::impl_sqlx_adapter_fetch_one!(sqlx::mysql::MySqlRow, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
    crate::impl_sqlx_adapter_fetch_scalar!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
    crate::impl_sqlx_adapter_execute!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
    crate::impl_sqlx_adapter_page_count!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
    crate::impl_sqlx_adapter_page_all!(sqlx::mysql::MySqlRow, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
    impl_sqlx_adapter_mysql_insert!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
    impl_sqlx_adapter_mysql_fetch_insert_id!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
}

impl SqlxExecutorAdatper for &mut sqlx::MySqlConnection {

    type DB = sqlx::MySql;

    type Row = sqlx::mysql::MySqlRow;

    crate::impl_sqlx_adapter_fetch_all!(sqlx::mysql::MySqlRow, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
    crate::impl_sqlx_adapter_fetch_one!(sqlx::mysql::MySqlRow, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
    crate::impl_sqlx_adapter_fetch_scalar!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
    crate::impl_sqlx_adapter_execute!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
    crate::impl_sqlx_adapter_page_count!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
    crate::impl_sqlx_adapter_page_all!(sqlx::mysql::MySqlRow, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
    impl_sqlx_adapter_mysql_insert!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
    impl_sqlx_adapter_mysql_fetch_insert_id!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
}
