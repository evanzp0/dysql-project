mod common_nh;

use common_nh::*;

use dysql::{fetch_all, Value};
use rbatis::executor::Executor;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, dysql::Content)]
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[derive(sqlx::FromRow)]
#[derive(tokio_pg_mapper_derive::PostgresMapper, PartialEq)]
#[pg_mapper(table="test_user")]
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

pub async fn sqlx_pg_db() -> sqlx::PgConnection {
    use sqlx::ConnectOptions;
    let options = sqlx::postgres::PgConnectOptions::new()
        .host("127.0.0.1")
        .port(5432)
        .username("root")
        .password("111111")
        .database("my_database");
    
    let conn = options.connect().await.unwrap();
    conn
}

async fn rbatis_pg_db() -> rbatis::executor::RBatisConnExecutor {
    let rb = rbatis::RBatis::new();
    rb.init(rbdc_pg::Driver{},"postgres://root:111111@localhost:5432/my_database").unwrap();
    rb.acquire().await.unwrap()
}

rbatis::pysql!(pysql_select(rb: &dyn Executor, name:&str)  -> Result<rbs::Value, rbatis::Error> =>
    r#"`select `
         ` * `
      `from test_user where name=#{name}`
"#);

//cargo test --release --bench bench_nh_pg --no-fail-fast -- --exact -Z unstable-options --show-output

// ---- bench_raw_sqlx stdout ----
// use Time: 31.643106854s ,each:316431 ns/op
// use QPS: 3160 QPS/s
#[test]
fn bench_raw_sqlx() {
    let f = async move {
        let mut conn = sqlx_pg_db().await;
        let name = "a5";
        let sql ="select * from test_user where name = $1 ";
        rbench!(100000, {
            let query = sqlx::query_as::<sqlx::Postgres, User>(&sql);
            let query = query.bind(&name);
            let _rst = query.fetch_all(&mut conn).await.unwrap();
        });
    };
    block_on(f);
}

// ---- bench_dysql_sqlx stdout ----
// use Time: 31.754854259s ,each:317548 ns/op
// use QPS: 3149 QPS/s
#[test]
fn bench_dysql_sqlx() {
    let f = async move {
        let mut conn = sqlx_pg_db().await;
        let dto = Value::new("a5");
        rbench!(100000, {
            fetch_all!(|&mut conn, &dto| -> User {
                "select * from test_user where 1 = 1 and name = :value"
            }).unwrap();
        });
    };
    block_on(f);
}

// ---- bench_raw_rbatis stdout ----
// use Time: 32.099826871s ,each:320998 ns/op
// use QPS: 3115 QPS/s
#[test]
fn bench_raw_rbatis() {
    let f = async move {
        let conn = rbatis_pg_db().await;
        let name = "a5".to_owned();
        rbench!(100000, {
            pysql_select(&conn, &name).await.unwrap();
        });
    };
    block_on(f);
}

// ---- bench_dysql_rbatis stdout ----
// use Time: 32.353856944s ,each:323538 ns/op
// use QPS: 3090 QPS/s
#[test]
fn bench_dysql_rbatis() {
    let f = async move {
        let conn = rbatis_pg_db().await;
        let dto = Value::new("a5");
        rbench!(100000, {
            dysql::fetch_all!(|&conn, &dto| -> User {
                "select * from test_user where name = :value"
            }).unwrap();
        });
    };
    block_on(f);
}

// ---- bench_raw_tokio_pg stdout ----
// use Time: 55.855347915s ,each:558553 ns/op
// use QPS: 1790 QPS/s
#[test]
fn bench_raw_tokio_pg() {
    use tokio_pg_mapper::FromTokioPostgresRow;

    let f = async move {
        let conn = tokio_pg_db().await;
        let sql ="select * from test_user where name = $1 ";
        let name = "a5";
        rbench!(100000, {
            let stmt = conn.prepare(&sql).await.unwrap();
            let rows = conn.query(&stmt, &[&name]).await.unwrap();

            let _ = rows
                .iter()
                .map(|row| <User>::from_row_ref(row).expect("query unexpected error"))
                .collect::<Vec<User>>();
        });
    };
    block_on(f);
}

// ---- bench_dysql_tokio_pg stdout ----
// use Time: 56.263362344s ,each:562633 ns/op
// use QPS: 1777 QPS/s
#[test]
fn bench_dysql_tokio_pg() {
    let f = async move {
        let conn = tokio_pg_db().await;
        let dto = Value::new("a5");
        rbench!(100000, {
            dysql::fetch_all!(|&conn, &dto| -> User {
                "select * from test_user where name = :value"
            }).unwrap();
        });
    };
    block_on(f);
}
