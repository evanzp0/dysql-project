use std::any::TypeId;

use async_trait::async_trait;
use dysql_tpl::Content;
use sqlx::{Postgres, Executor, FromRow, Database, Pool, Transaction};

use crate::{SqlxExecutorAdatper, SqlxQuery, SqlxTranAdatper};

pub struct SqlxPgQuery<D> 
where 
    D: Content + 'static + Send + Sync
{
    pub sql: String,
    pub dto: Option<D>,
}

impl<D> SqlxPgQuery<D> 
where 
    D: Content + 'static + Send + Sync
{
    pub fn new(sql: String, dto: Option<D>) -> Self {
        Self {
            sql,
            dto,
        }
    }
}

#[async_trait]
impl<D> SqlxQuery for SqlxPgQuery<D> 
where 
    D: Content + 'static + Send + Sync
{
    type Database = Postgres;

    async fn fetch_one<'c, E, U>(self, cot: E) -> U 
    where
        E: Executor<'c, Database = Self::Database>,
        for<'r> U: FromRow<'r, <Self::Database as Database>::Row> + Send + Unpin
    {
        let query = sqlx::query_as::<Postgres, U>(&self.sql);
        let rst = query.fetch_one(cot).await.unwrap();

        rst
    }
}


impl<D> SqlxExecutorAdatper<D> for Pool<Postgres>
where 
    D: Content + 'static + Send + Sync
{
    type Query = SqlxPgQuery<D>;

    fn create_query(&self, sql: String, dto: Option<D>) -> Self::Query {
        SqlxPgQuery::new("select * from test_user where id = 1".to_owned(), dto)
    }
}

impl<'c, D, DB> SqlxTranAdatper<D, DB> for Transaction<'c, DB> 
where 
    DB: Database,
    D: Content + 'static + Send + Sync,
{
    fn create_query(&self, sql: String, dto: Option<D>) -> impl SqlxQuery<Database = Postgres> {
        
        if TypeId::of::<DB>() == TypeId::of::<Postgres>() {
            SqlxPgQuery::new("select * from test_user where id = 1".to_owned(), dto)
        } else {
            panic!("todo")
        }
        
    }

    // fn get_dialect(&self) -> SqlDialect {
    //     if TypeId::of::<DB>() == TypeId::of::<Postgres>() {
    //         SqlDialect::postgres
    //     } else if TypeId::of::<DB>() == TypeId::of::<MySql>() {
    //         SqlDialect::mysql
    //     } else if TypeId::of::<DB>() == TypeId::of::<Sqlite>() {
    //         SqlDialect::sqlite
    //     } else {
    //         panic!("unknow sql dialect")
    //     }
    // }
}