
use std::{marker::PhantomData, any::TypeId};

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
        // todo! 以下分支需要用条件宏进行编译
        if TypeId::of::<DB>() == TypeId::of::<sqlx::Postgres>() {
            return SqlDialect::postgres
        }
        
        if TypeId::of::<DB>() == TypeId::of::<sqlx::MySql>() {
            return SqlDialect::mysql
        } 
        
        if TypeId::of::<DB>() == TypeId::of::<sqlx::Sqlite>() {
            return SqlDialect::sqlite
        }

        panic!("only support 'postgres', 'mysql', 'sqlite' sql dialect")
    }
}

// todo! 以下实现需要用tiaojian宏进行编译
impl<'q> SqlxExecutorAdatper<sqlx::Postgres> for sqlx::Transaction<'q, sqlx::Postgres> {}
impl SqlxExecutorAdatper<sqlx::Postgres> for sqlx::Pool<sqlx::Postgres> {}
impl SqlxExecutorAdatper<sqlx::Postgres> for &sqlx::Pool<sqlx::Postgres> {}
impl SqlxExecutorAdatper<sqlx::Postgres> for sqlx::PgConnection {}
impl SqlxExecutorAdatper<sqlx::Postgres> for &mut sqlx::PgConnection {}

impl<'q> SqlxExecutorAdatper<sqlx::MySql> for sqlx::Transaction<'q, sqlx::MySql> {}
impl SqlxExecutorAdatper<sqlx::MySql> for sqlx::Pool<sqlx::MySql> {}
impl SqlxExecutorAdatper<sqlx::MySql> for &sqlx::Pool<sqlx::MySql> {}
impl SqlxExecutorAdatper<sqlx::MySql> for sqlx::MySqlConnection {}
impl SqlxExecutorAdatper<sqlx::MySql> for &mut sqlx::MySqlConnection {}

impl<'q> SqlxExecutorAdatper<sqlx::Sqlite> for sqlx::Transaction<'q, sqlx::Sqlite> {}
impl SqlxExecutorAdatper<sqlx::Sqlite> for sqlx::Pool<sqlx::Sqlite> {}
impl SqlxExecutorAdatper<sqlx::Sqlite> for &sqlx::Pool<sqlx::Sqlite> {}
impl SqlxExecutorAdatper<sqlx::Sqlite> for sqlx::SqliteConnection {}
impl SqlxExecutorAdatper<sqlx::Sqlite> for &mut sqlx::SqliteConnection {}

pub struct SqlxQuery <DB>
{
    pub(crate) temp_db: PhantomData<DB>,
}
