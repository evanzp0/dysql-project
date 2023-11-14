use std::any::TypeId;

use sqlx::{PgConnection, Postgres, Pool, Transaction, MySql, Sqlite};

use crate::{SqlDialect, InstanceOf};

pub enum QueryCmd {
    Execute(String),
    FetchAll(String),
    FetchOne(String),
    FetchScalar(String),
    Insert(String),
    Page {count_sql: String, page_sql: String},
}

pub trait ExecutorAdatper {
    fn get_dialect(&self) -> SqlDialect;
}

impl ExecutorAdatper for PgConnection {
    fn get_dialect(&self) -> SqlDialect {
        SqlDialect::postgres
    }
}

impl ExecutorAdatper for Pool<Postgres> {
    fn get_dialect(&self) -> SqlDialect {
        SqlDialect::postgres
    }
}

impl<DB> ExecutorAdatper for Transaction<'_, DB> 
where 
    DB: sqlx::Database
{
    fn get_dialect(&self) -> SqlDialect {
        if TypeId::of::<DB>() == TypeId::of::<Postgres>() {
            SqlDialect::postgres
        } else if TypeId::of::<DB>() == TypeId::of::<MySql>() {
            SqlDialect::mysql
        } else if TypeId::of::<DB>() == TypeId::of::<Sqlite>() {
            SqlDialect::sqlite
        } else {
            panic!("unknow sql dialect")
        }
    }
}

// pub struct Query<D> {
//     pub query_type: QueryCmd,
//     pub dto: Option<D>,
// }

// impl<D> Query<D> 
// where 
//     D: Content + 'static
// {
//     pub fn new(query_type: QueryCmd,dto: Option<D>) -> Self {
//         Self {
//             query_type,
//             dto,
//         }
//     }

//     pub fn execute<C>(&mut self, cot: &C)
//     where 
//         C: 'static + Sized,
//     {
//         // self.dto.take()
//         // self.instance_of_mut::<bool>();
        
//         let rst = cot.instance_of::<sqlx::Pool<Postgres>>();
//         println!("sqlx::Pool<Postgres> = {}", rst);


//         // todo!()
//     }

//     pub async fn fetch_one<'c, E, U>(&mut self, cot: E) -> Result<U, DySqlError>
//     where 
//         E: sqlx::Executor<'c, Database = Postgres>,
//         for<'r> U: sqlx::FromRow<'r, <Postgres as sqlx::Database>::Row>,
//         U: Send + Sized + Unpin + Debug,
//     {
//         // let rst = cot.instance_of::<sqlx::Pool<Postgres>>();
//         // println!("sqlx::Pool<Postgres> = {}", rst);

//         let rst = sqlx::query_as::<Postgres, U>("select * from test_user where id = 1").fetch_one(cot).await.unwrap();
//         println!("rst = {:?}", rst);

//         Ok(rst)
//     }

//     pub fn execute_mut<N>(&mut self, cot: &mut N)
//     where 
//         N: 'static + Sized,
//     {
//         self.instance_of_mut::<bool>();


//         todo!()
//     }

//     pub fn gen_page_count_sql(raw_sql: &str) -> String {
//         format!("SELECT count(*) FROM ({}) as _tmp", raw_sql)
//     }
    
//     pub fn gen_page_sql(raw_sql: &str) -> String {
//         let mut page_sql = raw_sql.to_owned();
//         page_sql.push_str(
//             " {{#is_sort}} ORDER BY {{#sort_model}} {{field}} {{sort}}, {{/sort_model}} ![B_DEL(,)] {{/is_sort}} LIMIT {{page_size}} OFFSET {{start}} "
//         );
    
//         page_sql
//     }
// }
