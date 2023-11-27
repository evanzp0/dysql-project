mod common_nh;
use std::{future::Future, pin::Pin};

use common_nh::*;

use dysql::{Content, fetch_all, Value};
use rbatis::executor::Executor;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, ConnectOptions, PgConnection};

#[derive(Clone, Debug, Serialize, Deserialize, Content)]
pub struct UserDto {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub age: Option<i32>,
    pub id_rng: Option<Vec<i32>>,
}

impl UserDto {
    pub fn new(id: Option<i64>, name: Option<String>, age: Option<i32>, id_rng: Option<Vec<i32>>) -> Self {
        Self { id, name, age, id_rng }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[derive(FromRow)]
pub struct User {
    pub id: i64,
    pub name: Option<String>,
    pub age: Option<i32>,
}

async fn tokio_pg_db() -> tokio_postgres::Client {
    use tokio_postgres::{NoTls, connect};
    let (client, connection) = connect("host=127.0.0.1 user=root password=111111 dbname=my_database", NoTls).await.unwrap();
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    client
}

pub async fn sqlx_pg_db() -> PgConnection {
    // let conn = sqlx::postgres::PgPoolOptions::new()
    //     .acquire_timeout(std::time::Duration::from_secs(60 * 60))
    //     .max_connections(5)
    //     .connect("postgres://root:111111@127.0.0.1/my_database").await.unwrap();

    let options = sqlx::postgres::PgConnectOptions::new()
        .host("127.0.0.1")
        .port(5432)
        .username("root")
        .password("111111")
        .database("my_database");
    
    let conn = options.connect().await.unwrap();
    conn
}

async fn rbatis_pg_db() -> rbatis::RBatis {
    let rb = rbatis::RBatis::new();
    rb.init(rbdc_pg::Driver{},"postgres://root:111111@localhost:5432/my_database").unwrap();
    rb
}

rbatis::pysql!(pysql_select(rb: &dyn Executor, name:&str)  -> Result<rbs::Value, rbatis::Error> =>
    r#"`select `
         ` * `
      `from test_user where name=#{name}`
"#);

//cargo test --release --bench bench_nh_pg --no-fail-fast -- --exact -Z unstable-options --show-output
// ---- bench_raw_rbatis stdout ----
// use Time: 32.028605071s ,each:320286 ns/op
// use QPS: 3122 QPS/s
#[tokio::test]
async fn bench_raw_rbatis() {
    let rbatis = rbatis_pg_db().await;
    let name = "a5".to_owned();
    let f = async move {
        rbench!(100000, {
            pysql_select(&rbatis, &name).await.unwrap();
        });
    };
    f.await;
}

// ---- bench_dysql_rbatis stdout ----
// use Time: 32.179844565s ,each:321798 ns/op
// use QPS: 3107 QPS/s
#[tokio::test]
async fn bench_dysql_rbatis() {
    let rbatis = rbatis_pg_db().await;
    let dto = Value::new("a5");
    let f = async move {
        rbench!(100000, {
            dysql::fetch_all!(|&rbatis, &dto| -> User {
                "select * from test_user where name = :value"
            }).unwrap();
        });
    };
    f.await;
}

// ---- bench_dysql_sqlx stdout ----
// use Time: 16.68448359s ,each:166844 ns/op
// use QPS: 5993 QPS/s
#[tokio::test]
async fn bench_dysql_sqlx() {
    let mut conn = sqlx_pg_db().await;
    let dto = Value::new("a5");
    let f = async move {
        rbench!(100000, {
            fetch_all!(|&mut conn, &dto| -> User {
                "select * from test_user where 1 = 1 and name = :value"
            }).unwrap();
        });
    };
    f.await;
}

// ---- bench_raw_sqlx stdout ----
// use Time: 16.53535847s ,each:165353 ns/op
// use QPS: 6047 QPS/s
#[tokio::test]
async fn bench_raw_sqlx() {
    let mut conn = sqlx_pg_db().await;
    let name = "a5";
    let f = async move {
        rbench!(100000, {
            let sql ="select * from test_user where name = $1 ";

            let query = sqlx::query_as::<sqlx::Postgres, User>(&sql);
            let query = query.bind(&name);
            let _rst = query.fetch_all(&mut conn).await.unwrap();
        });
    };
    f.await
}
