use crate::SqlxExecutorAdatper;

crate::impl_sql_adapter!(sqlx::Sqlite, sqlx::SqliteConnection);


impl SqlxExecutorAdatper<sqlx::Sqlite> for &mut sqlx::SqliteConnection {
    crate::impl_sqlx_adapter_fetch_all!(sqlx::Sqlite, sqlx::sqlite::SqliteRow, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
}


impl crate::SqlxQuery <sqlx::Sqlite>
{
    // crate::impl_sqlx_adapter_fetch_all!(sqlx::Sqlite, sqlx::sqlite::SqliteRow, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);

    // crate::impl_sqlx_adapter_fetch_one!(sqlx::Sqlite, sqlx::sqlite::SqliteRow, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);

    // crate::impl_sqlx_adapter_fetch_scalar!(sqlx::Sqlite, sqlx::sqlite::SqliteRow, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
    // crate::impl_sqlx_adapter_execute!(sqlx::Sqlite, sqlx::sqlite::SqliteRow, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
    // crate::impl_sqlx_adapter_page_count!(sqlx::Sqlite, sqlx::sqlite::SqliteRow, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
    // crate::impl_sqlx_adapter_page_all!(sqlx::Sqlite, sqlx::sqlite::SqliteRow, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);

    pub async fn insert<'e, 'c: 'e, E, D, U>(self, executor: E, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>) 
        -> Result<Option<U>, crate::DySqlError>
        where
            E: 'e + sqlx::Executor<'c, Database = sqlx::Sqlite> + crate::SqlxExecutorAdatper<sqlx::Sqlite> ,
            D: dysql_tpl::Content + Send + Sync,
            for<'r> U: sqlx::Decode<'r, sqlx::Sqlite> + sqlx::Type<sqlx::Sqlite> + Send + Unpin,
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
        let mut query = sqlx::query(&sql);
        if let Some(dto) = &dto {
            for param_name in &param_names {
                let stpl = dysql_tpl::SimpleTemplate::new(param_name);
                
                let param_value = stpl.apply(dto);
                match param_value {
                    Ok(param_value) => {
                        query = impl_bind_sqlx_param_value!(query, param_value, [i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc]);
                    },
                    Err(e) => Err( crate::DySqlError( crate::ErrorInner::new( crate::Kind::BindParamterError, Some(e), None)))?,
                }
            }
        }
        let rst = query.execute(executor).await;
        match rst {
            // 返回 None 让外层继续调用 fetch_insert_id()
            Ok(_) => Ok(None),
            Err(e) => Err(crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(Box::new(e)), None))),
        }
    }

    pub async fn fetch_insert_id<'e, 'c: 'e, E, U>(self, executor: E) -> Result<U, crate::DySqlError>
    where
        E: 'e + sqlx::Executor<'c, Database = sqlx::Sqlite> + crate::SqlxExecutorAdatper<sqlx::Sqlite>,
        for<'r> U: sqlx::Decode<'r, sqlx::Sqlite> + sqlx::Type<sqlx::Sqlite> + Send + Unpin,
    {
        let insert_id = sqlx::query_as::<_, (U,)>("SELECT last_insert_rowid();")
            .fetch_one(executor)
            .await;

        match insert_id {
            Ok(insert_id) => Ok(insert_id.0),
            Err(e) => Err(crate::DySqlError(crate::ErrorInner::new(crate::Kind::QueryError, Some(Box::new(e)), None))),
        }
    }
}
