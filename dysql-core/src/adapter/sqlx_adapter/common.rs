
use std::marker::PhantomData;

use sqlx::{Database, Sqlite};

use crate::SqlDialect;

#[macro_export]
macro_rules! impl_bind_sqlx_param_value {
    (
        $query:ident, $p_val:ident, [$($vtype:ty),+]
    ) => {
        paste::paste!{
            match $p_val {
                $(
                    dysql_tpl::SimpleValue::[<t_ $vtype>](val) => $query.bind(val),
                )*
                dysql_tpl::SimpleValue::t_str(val) => $query.bind(unsafe {&*val.0}),
                dysql_tpl::SimpleValue::t_String(val) => $query.bind(unsafe {&*val.0}),
                dysql_tpl::SimpleValue::None(val) => $query.bind(val),
                _ => Err(crate::DySqlError(crate::ErrorInner::new(crate::Kind::BindParamterError, None, Some(format!("the type of {:?} is not support", $p_val)))))?,
            }
        }
    };
}

pub struct SqlxQuery <DB>
{
    pub(crate) temp_db: PhantomData<DB>,
}

pub trait SqlxExecutorAdatper<DB> 
where 
    DB: sqlx::Database,
{
    fn create_query(&self) -> SqlxQuery<DB>
    where
        DB: sqlx::Database
    {
        SqlxQuery { temp_db: PhantomData}
    }

    fn get_dialect(&self) -> SqlDialect 
    {
        // 以下分支需要用条件宏进行编译
        #[cfg(feature = "sqlx-postgres")]
        if std::any::TypeId::of::<DB>() == std::any::TypeId::of::<sqlx::Postgres>() {

            return SqlDialect::postgres
        }
        
        #[cfg(feature = "sqlx-mysql")]
        if std::any::TypeId::of::<DB>() == std::any::TypeId::of::<sqlx::MySql>() {

            return SqlDialect::mysql
        } 
        
        #[cfg(feature = "sqlx-sqlite")]
        if std::any::TypeId::of::<DB>() == std::any::TypeId::of::<sqlx::Sqlite>() {

            return SqlDialect::sqlite
        }

        panic!("only support 'postgres', 'mysql', 'sqlite' sql dialect")
    }

    async fn dy_fetch_all<D, U>(self, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>) 
        -> Result<Vec<U>, crate::DySqlError>
    where
        D: dysql_tpl::Content + Send + Sync,
        for<'r> U: sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin;
}
