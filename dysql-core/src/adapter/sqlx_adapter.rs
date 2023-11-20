
use std::marker::PhantomData;

use crate::SqlDialect;

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
        #[cfg(feature = "sqlx-postges")]
        if any::TypeId::of::<DB>() == any::TypeId::of::<sqlx::Postgres>() {
            return SqlDialect::postgres
        }
        
        #[cfg(feature = "sqlx-mysql")]
        if any::TypeId::of::<DB>() == any::TypeId::of::<sqlx::MySql>() {
            return SqlDialect::mysql
        } 
        
        #[cfg(feature = "sqlx-sqlite")]
        if any::TypeId::of::<DB>() == any::TypeId::of::<sqlx::Sqlite>() {
            return SqlDialect::sqlite
        }

        panic!("only support 'postgres', 'mysql', 'sqlite' sql dialect")
    }
}

pub struct SqlxQuery <DB>
{
    pub(crate) temp_db: PhantomData<DB>,
}