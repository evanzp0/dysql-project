use crate::SqlxExecutorAdatper;

macro_rules! impl_sqlx_adapter_pg_insert {
    ([$($vtype:ty),+]) => 
    {
        async fn insert<D, U>(self, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>) 
            -> Result<Option<U>, crate::DySqlError>
        where
            D: dysql_tpl::Content + Send + Sync,
            for<'r> U: sqlx::Decode<'r, Self::DB> + sqlx::Type<Self::DB> + Send + Unpin
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
            let mut query = sqlx::query_scalar::<_, U>(&sql);
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

            let insert_id = query.fetch_one(self).await;
            
            let insert_id = insert_id.map_err(|e| 
                crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(Box::new(e)), None))
            )?;
            Ok(Some(insert_id))
        }
    };
}

macro_rules! impl_sqlx_adapter_pg_fetch_insert_id {
    ([$($vtype:ty),+]) => 
    {
        async fn fetch_insert_id<U>(self)
            -> Result<Option<U>, crate::DySqlError>
        where
            for<'r> U: sqlx::Decode<'r, Self::DB> + sqlx::Type<Self::DB> + Send + Unpin
        {
            Ok(None)
        }
    };
}

impl SqlxExecutorAdatper for &sqlx::Pool<sqlx::Postgres> {

    type DB = sqlx::Postgres;

    type Row = sqlx::postgres::PgRow;

    crate::impl_sqlx_adapter_fetch_all!(sqlx::postgres::PgRow, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
    crate::impl_sqlx_adapter_fetch_one!(sqlx::postgres::PgRow, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
    crate::impl_sqlx_adapter_fetch_scalar!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
    crate::impl_sqlx_adapter_execute!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
    crate::impl_sqlx_adapter_page_count!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
    crate::impl_sqlx_adapter_page_all!(sqlx::postgres::PgRow, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
    impl_sqlx_adapter_pg_insert!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
    impl_sqlx_adapter_pg_fetch_insert_id!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
}

impl SqlxExecutorAdatper for &mut sqlx::PgConnection {

    type DB = sqlx::Postgres;

    type Row = sqlx::postgres::PgRow;

    crate::impl_sqlx_adapter_fetch_all!(sqlx::postgres::PgRow, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
    crate::impl_sqlx_adapter_fetch_one!(sqlx::postgres::PgRow, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
    crate::impl_sqlx_adapter_fetch_scalar!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
    crate::impl_sqlx_adapter_execute!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
    crate::impl_sqlx_adapter_page_count!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
    crate::impl_sqlx_adapter_page_all!(sqlx::postgres::PgRow, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
    impl_sqlx_adapter_pg_insert!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
    impl_sqlx_adapter_pg_fetch_insert_id!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
}