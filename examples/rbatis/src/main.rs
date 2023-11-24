use std::sync::Arc;

use async_trait::async_trait;
use rbatis::{RBatis, sql::PageRequest, dark_std::sync::SyncVec, intercept::{Intercept, ResultType}, executor::Executor, rbdc::db::ExecResult, Error};

use rbdc_mysql::Driver;
use rbs::Value;
use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() {
    let mut rb = RBatis::new();
    let queue = Arc::new(SyncVec::new());
    rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
    rb.init(Driver{},"mysql://root:111111@localhost:3306/my_database").unwrap();
    // let driver_type = rb.driver_type().unwrap();
    // println!("{}", driver_type);
    // let dto = UserDto{ id: Some(2), name: Some("ab".to_owned()), age: Some(13) , id_rng: None };
    // let sql = "select * from test_user where name = $1";

    // let rb_args = vec![rbs::to_value!(&dto.name)];

    // let rst: Option<User> = rb
    //     .query_decode(sql, rb_args.clone())
    //     .await
    //     .unwrap();
    // println!("{:#?}", rst);

    // let mut tran = rb.acquire_begin().await.unwrap();
    // let insert_sql = "insert into test_user (name, age) values ('ab', 1) returning id";

    // let r = tran.exec(&insert_sql, vec![]).await.unwrap();
    // println!("r = {:#?}", r);

    // let sql = "select * from test_user order by id";
    // let r = tran.query(&sql, vec![]).await.unwrap();
    // let rst = rbatis::decode::decode::<Option<Vec<User>>>(r);
    // println!("{:#?}", rst);

    // let sql = "select count(*) from test_user";
    // let r = tran.query(&sql, vec![]).await.unwrap();
    // let rst = rbatis::decode::decode::<i64>(r);
    // println!("{:#?}", rst);
    // tran.rollback().await.ok();
    
    let rst = pysql_select_page(&mut rb, &PageRequest::new(1, 10), "a4").await;
    let (sql, args) = queue.pop().unwrap();
    println!("{}", sql);

    println!("{:#?}", rst);
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct PySqlSelectPageArg {
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserDto {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub age: Option<i32>,
    pub id_rng: Option<Vec<i32>>,
}

rbatis::pysql_select_page!(pysql_select_page(name:&str) -> UserDto =>
    r#"`select `
      if do_count == true:
        ` count(1) as count `
      if do_count == false:
         ` * `
      `from test_user where 1 = 1`
        if name != '':
           ` and name=#{name}`
      ` limit ${page_no},${page_size}`
"#);

impl UserDto {
    pub fn new(id: Option<i64>, name: Option<String>, age: Option<i32>, id_rng: Option<Vec<i32>>) -> Self {
        Self { id, name, age, id_rng }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: Option<String>,
    pub age: Option<i32>,
}

#[derive(Debug)]
    pub struct MockIntercept {
        pub sql_args: Arc<SyncVec<(String, Vec<Value>)>>,
    }

    impl MockIntercept {
        fn new(inner: Arc<SyncVec<(String, Vec<Value>)>>) -> Self {
            Self { sql_args: inner }
        }
    }

    #[async_trait]
    impl Intercept for MockIntercept {
        async fn before(
            &self,
            task_id: i64,
            rb: &dyn Executor,
            sql: &mut String,
            args: &mut Vec<Value>,
            _result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
        ) -> Result<bool, Error> {
            self.sql_args.push((sql.to_string(), args.clone()));
            Ok(true)
        }
    }