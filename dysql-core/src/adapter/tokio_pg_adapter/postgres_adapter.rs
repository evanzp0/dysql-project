use crate::TokioPgExecutorAdatper;


impl TokioPgExecutorAdatper for &tokio_postgres::Client {
    crate::impl_tokio_pg_adapter_fetch_all!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc, DateTime_Local, DateTime_FixedOffset]);
    crate::impl_tokio_pg_adapter_fetch_one!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc, DateTime_Local, DateTime_FixedOffset]);
    crate::impl_tokio_pg_adapter_fetch_scalar!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc, DateTime_Local, DateTime_FixedOffset]);
    crate::impl_tokio_pg_adapter_execute!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc, DateTime_Local, DateTime_FixedOffset]);
    crate::impl_tokio_pg_adapter_insert!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc, DateTime_Local, DateTime_FixedOffset]);
    crate::impl_tokio_pg_adapter_fetch_insert_id!();
    crate::impl_tokio_pg_adapter_page_count!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc, DateTime_Local, DateTime_FixedOffset]);
    crate::impl_tokio_pg_adapter_page_all!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc, DateTime_Local, DateTime_FixedOffset]);
}

impl TokioPgExecutorAdatper for &tokio_postgres::Transaction<'_> {
    crate::impl_tokio_pg_adapter_fetch_all!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc, DateTime_Local, DateTime_FixedOffset]);
    crate::impl_tokio_pg_adapter_fetch_one!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc, DateTime_Local, DateTime_FixedOffset]);
    crate::impl_tokio_pg_adapter_fetch_scalar!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc, DateTime_Local, DateTime_FixedOffset]);
    crate::impl_tokio_pg_adapter_execute!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc, DateTime_Local, DateTime_FixedOffset]);
    crate::impl_tokio_pg_adapter_insert!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc, DateTime_Local, DateTime_FixedOffset]);
    crate::impl_tokio_pg_adapter_fetch_insert_id!();
    crate::impl_tokio_pg_adapter_page_count!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc, DateTime_Local, DateTime_FixedOffset]);
    crate::impl_tokio_pg_adapter_page_all!([i64, i32, i16, i8, f32, f64, bool, Uuid, NaiveDateTime, Utc, DateTime_Local, DateTime_FixedOffset]);
}
