use std::{marker::PhantomData, any::TypeId};
use dysql_tpl::Content;
use sqlx::{Postgres, Database, Pool, Transaction, MySql, Sqlite};

use crate::{SqlxExecutorAdatper, SqlxQuery, SqlDialect};

impl<'q, D, DB> SqlxExecutorAdatper<'q, D, DB> for Pool<Postgres>
where 
    D: Content + 'static + Send + Sync,
    DB: Database
{
    fn create_query(&self, sql: &'q str, dto: Option<D>) -> SqlxQuery<'q, D, DB>
    {
        SqlxQuery {
            sql,
            dto,
            temp_db: PhantomData,
        }
    }

    fn get_dialect(&self) -> SqlDialect {
        SqlDialect::postgres
    }
}

impl<'q, D, DB> SqlxExecutorAdatper<'q, D, DB> for Transaction<'q, DB> 
where 
    DB: Database,
    D: Content + 'static + Send + Sync,
{
    fn create_query(&self, sql: &'q str, dto: Option<D>) -> SqlxQuery<'q, D, DB> 
    where 
        DB: Database
    {
        SqlxQuery {
            sql,
            dto,
            temp_db: PhantomData,
        }
    }

    fn get_dialect(&self) -> crate::SqlDialect {
        // todo! 以下分支需要用条件宏进行编译
        if TypeId::of::<DB>() == TypeId::of::<Postgres>() {
            return SqlDialect::postgres
        }
        
        if TypeId::of::<DB>() == TypeId::of::<MySql>() {
            return SqlDialect::mysql
        } 
        
        if TypeId::of::<DB>() == TypeId::of::<Sqlite>() {
            return SqlDialect::sqlite
        }

        panic!("only support 'postgres', 'mysql', 'sqlite' sql dialect")
    }
}