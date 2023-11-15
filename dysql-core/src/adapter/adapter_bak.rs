mod sqlx_postgres;

use async_trait::async_trait;
use dysql_tpl::Content;
use sqlx::{Executor, FromRow, Database, Postgres};

pub use sqlx_postgres::*;

#[async_trait]
pub trait SqlxQuery {
    type Database: Database;
    
    async fn fetch_one<'c, E, U>(self, cot: E) -> U 
    where
        E: Executor<'c, Database = Self::Database>,
        for<'r> U: FromRow<'r, <Self::Database as Database>::Row> + Send + Unpin;
}

pub trait SqlxExecutorAdatper<D>
where 
    D: Content + 'static + Send + Sync
{
    type Query: SqlxQuery;

    fn create_query(&self, sql: String, dto: Option<D>) -> Self::Query;
}

pub trait SqlxTranAdatper<D, DB>
where 
    DB: Database,
    D: Content + 'static + Send + Sync,
{
    fn create_query(&self, sql: String, dto: Option<D>) -> impl SqlxQuery<Database = Postgres>;
}