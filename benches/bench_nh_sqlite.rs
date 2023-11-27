use dysql::{Value, fetch_all};
use rbatis::{RBatis, executor::Executor};
use rbdc_sqlite::Driver;
use sqlx::{SqliteConnection, sqlite::{SqliteConnectOptions, SqliteJournalMode}};

mod common_nh;
use common_nh::*;


async fn init_rbatis_connection() -> rbatis::executor::RBatisConnExecutor {
    let rb = RBatis::new();
    rb.init(Driver{},"sqlite::memory:").unwrap();

    rb.exec(r#"
        CREATE TABLE test_user (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name VARCHAR(255) NULL,
            age INT NULL
        )"#,
        vec![]
    ).await.unwrap();

    rb.exec("INSERT INTO test_user (name, age) VALUES ('a', 10)", vec![]).await.unwrap();
    rb.exec("INSERT INTO test_user (name, age) VALUES ('a', 21)", vec![]).await.unwrap();
    rb.exec("INSERT INTO test_user (name, age) VALUES ('a', 35)", vec![]).await.unwrap();
    rb.exec("INSERT INTO test_user (name, age) VALUES ('a', 12)", vec![]).await.unwrap();
    rb.exec("INSERT INTO test_user (name, age) VALUES ('a', 21)", vec![]).await.unwrap();
    rb.exec("INSERT INTO test_user (name, age) VALUES ('b', 22)", vec![]).await.unwrap();
    rb.exec("INSERT INTO test_user (name, age) VALUES ('b', 24)", vec![]).await.unwrap();
    rb.exec("INSERT INTO test_user (name, age) VALUES ('b', 31)", vec![]).await.unwrap();
    rb.exec("INSERT INTO test_user (name, age) VALUES ('b', 33)", vec![]).await.unwrap();

    rb.acquire().await.unwrap()
}

async fn init_sqlx_db() -> SqliteConnection {
    use std::str::FromStr;
    use sqlx::ConnectOptions;
    let mut conn = SqliteConnectOptions::from_str("sqlite::memory:")
        .unwrap()
        .journal_mode(SqliteJournalMode::Wal)
        .read_only(false)
        .connect()
        .await.unwrap();

    // prepare test data
    sqlx::query("DROP TABLE IF EXISTS test_user").execute(&mut conn).await.unwrap();
    sqlx::query(r#"
        CREATE TABLE test_user (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name VARCHAR(255) NULL,
            age INT NULL
        )"#
    ).execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('a', 10)").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('a', 21)").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('a', 35)").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('a', 12)").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('a', 21)").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('b', 22)").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('b', 24)").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('b', 31)").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('b', 33)").execute(&mut conn).await.unwrap();
    
    conn
}

rbatis::pysql!(pysql_select(rb: &dyn Executor, name:&str)  -> Result<rbs::Value, rbatis::Error> =>
    r#"`select `
         ` * `
      `from test_user where name=#{name}`
"#);

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
pub struct User {
    pub id: i64,
    pub name: Option<String>,
    pub age: Option<i32>,
}

//cargo test --release --package dysql --bench bench_nh_sqlite--no-fail-fast -- --exact -Z unstable-options --show-output
//cargo test --release --bench bench_nh_sqlite --no-fail-fast -- --exact -Z unstable-options --show-output


// ---- bench_dysql_sqlx stdout ----
// use Time: 4.174755327s ,each:41747 ns/op
// use QPS: 23953 QPS/s
#[test]
fn bench_raw_sqlx() {
    let f = async move {
        let mut conn = init_sqlx_db().await;
            let name = "a";
            let sql ="select * from test_user where name = ? ";
        rbench!(100000, {
            let query = sqlx::query_as::<_, User>(&sql);
            let query = query.bind(&name);
            let _rst = query.fetch_all(&mut conn).await.unwrap();
        });
    };
    block_on(f);
}

// ---- bench_raw_sqlx stdout ----
// use Time: 4.903177835s ,each:49031 ns/op
// use QPS: 20394 QPS/s
#[test]
fn bench_dysql_sqlx() {
    let f = async move {
        let mut conn = init_sqlx_db().await;
        let dto = Value::new("a");
        rbench!(100000, {
            fetch_all!(|&mut conn, &dto| -> User {
                "select * from test_user where 1 = 1 and name = :value"
            }).unwrap();
        });
    };
    block_on(f);
}

// ---- bench_raw_rbatis stdout ----
// use Time: 7.235434223s ,each:72354 ns/op
// use QPS: 13820 QPS/s
#[test]
fn bench_raw_rbatis() {
    let f = async move {
        let rbatis = init_rbatis_connection().await;
        let name = "a".to_owned();
        rbench!(100000, {
            pysql_select(&rbatis, &name).await.unwrap();
        });
    };
    block_on(f);
}

// ---- bench_dysql_rbatis stdout ----
// use Time: 7.215524126s ,each:72155 ns/op
// use QPS: 13859 QPS/s
#[test]
fn bench_dysql_rbatis() {
    let f = async move {
        let rbatis = init_rbatis_connection().await;
        let dto = Value::new("a");
        rbench!(100000, {
            dysql::fetch_all!(|&rbatis, &dto| -> User {
                "select * from test_user where name = :value"
            }).unwrap();
        });
    };
    block_on(f);
}
