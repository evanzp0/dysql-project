mod sqlx_postgres;

use std::marker::PhantomData;

use dysql_tpl::Content;
use sqlx::{Executor, FromRow, Database, IntoArguments, database::HasArguments};

use crate::{DySqlError, ErrorInner, Kind};

pub struct SqlxQuery <'q, D, DB> 
where 
    D: Content + 'static + Send + Sync,
    DB: Database,
{
    pub sql: &'q str,
    pub dto: Option<D>,
    pub(crate) temp_db: PhantomData<DB>,
}

impl<'q, D, DB> SqlxQuery <'q, D, DB> 
where 
    D: Content + 'static + Send + Sync,
    DB: Database,
{
    pub async fn fetch_one<'e, 'c: 'e, E, U>(self, executor: E) -> Result<U, DySqlError>
    where
        E: 'e + Executor<'c, Database = DB>,
        for<'r> U: FromRow<'r, <DB as Database>::Row> + Send + Unpin,
        <DB as HasArguments<'q>>::Arguments: IntoArguments<'q, DB>
    {
        let query = sqlx::query_as::<DB, U>(self.sql);
        let rst = query.fetch_one(executor).await;

        rst.map_err(|e| DySqlError(ErrorInner::new(Kind::QueryError, Some(Box::new(e)), None)))
    }
}

pub trait SqlxExecutorAdatper<'q, D, DB> 
where 
    D: Content + 'static + Send + Sync,
    DB: Database,
{
    fn create_query(&self, sql: &'q str, dto: Option<D>) -> SqlxQuery<'q, D, DB>;
}

pub trait SqlxTranAdatper<'q, D, DB>
where 
    D: Content + 'static + Send + Sync,
    DB: Database,
{
    fn create_query(&self, sql: &'q str, dto: Option<D>) -> SqlxQuery<'q, D, DB>;
}