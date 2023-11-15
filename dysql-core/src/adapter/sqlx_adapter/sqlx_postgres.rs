use std::marker::PhantomData;
use dysql_tpl::Content;
use sqlx::{Postgres, Database, Pool, Transaction};

use crate::{SqlxExecutorAdatper, SqlxQuery, SqlxTranAdatper};

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
}

impl<'q, D, DB> SqlxTranAdatper<'q, D, DB> for Transaction<'q, DB> 
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
}