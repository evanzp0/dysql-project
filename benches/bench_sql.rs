use std::{future::Future, str::FromStr};

use dysql::{Value, PageDto, page, Content, Pagination};
use rbatis::{RBatis, sql::PageRequest};
use rbdc_sqlite::Driver;
use serde::{Serialize, Deserialize};
use sqlx::{SqliteConnection, sqlite::{SqliteConnectOptions, SqliteJournalMode}, ConnectOptions, FromRow};

pub fn block_on<T>(task: T) -> T::Output
where
    T: Future + Send + 'static,
    T::Output: Send + 'static,
{
    tokio::task::block_in_place(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("tokio block_on fail")
            .block_on(task)
    })
}

pub trait QPS {
    fn qps(&self, total: u64);
    fn time(&self, total: u64);
    fn cost(&self);
}

impl QPS for std::time::Instant {
    fn qps(&self, total: u64) {
        let time = self.elapsed();
        println!(
            "use QPS: {} QPS/s",
            (total as u128 * 1000000000 as u128 / time.as_nanos() as u128)
        );
    }

    fn time(&self, total: u64) {
        let time = self.elapsed();
        println!(
            "use Time: {:?} ,each:{} ns/op",
            &time,
            time.as_nanos() / (total as u128)
        );
    }

    fn cost(&self) {
        let time = self.elapsed();
        println!("cost:{:?}", time);
    }
}

#[macro_export]
macro_rules! rbench {
    ($total:expr,$body:block) => {{
        let now = std::time::Instant::now();
        for _ in 0..$total {
            $body;
        }
        now.time($total);
        now.qps($total);
    }};
}

//cargo test --release --package dysql --bench bench_sql_raw --no-fail-fast -- --exact -Z unstable-options --show-output
//cargo test --release --bench bench_sql_raw  --no-fail-fast -- --exact -Z unstable-options --show-output
// ---- bench_raw stdout ----(windows)
// use Time: 17.794592926s ,each:177945 ns/op
// use QPS: 5619 QPS/s
#[test]
fn bench_raw_rbatis() {
    let rbatis = block_on(init_rbatis_connection());
    let name = "a".to_owned();
    let f = async move {
        rbench!(100000, {
            pysql_select_page(&rbatis.acquire().await.unwrap(), &PageRequest::new(1, 10), &name).await.unwrap();
        });
    };
    block_on(f);
}

// ---- bench_raw_dysql_rbatis stdout ----
// use Time: 23.54819283s ,each:235481 ns/op
// use QPS: 4246 QPS/s
#[test]
fn bench_raw_dysql_rbatis() {
    let rbatis = block_on(init_rbatis_connection());
    let dto = Value::new("a");
    let sort_model = vec![];
    let mut pg_dto = PageDto::new_with_sort(10, 1, Some(dto), sort_model);
    let f = async move {
        rbench!(100000, {
            page!(|&rbatis, pg_dto| -> User {
                "select * from test_user 
                where 1 = 1
                    {{#data.value}}and name = :data.value {{/data.value}}
                "
            }).unwrap();
        });
    };
    block_on(f);
}

// ---- bench_raw_dysql_sqlx stdout ----
// use Time: 6.835074214s ,each:68350 ns/op
// use QPS: 14630 QPS/s
#[test]
fn bench_raw_dysql_sqlx() {
    let mut conn = block_on(init_sqlx_db());
    let dto = Value::new("a");
    let sort_model = vec![];
    let mut pg_dto = PageDto::new_with_sort(10, 1, Some(dto), sort_model);
    let f = async move {
        rbench!(100000, {
            page!(|&mut conn, pg_dto| -> User {
                "select * from test_user 
                where 1 = 1
                    {{#data.value}}and name = :data.value {{/data.value}}
                "
            }).unwrap();
        });
    };
    block_on(f);
}

use std::io::Write;

// ---- bench_raw_sqlx stdout ----
// use Time: 6.005570082s ,each:60055 ns/op
// use QPS: 16651 QPS/s
#[test]
fn bench_raw_sqlx() {
    let mut conn = block_on(init_sqlx_db());
    let dto = UserDto{ id: None, name: Some("a".to_owned()), age: Some(13), id_rng: None };
    let name = "a";
    let mut pg_dto = PageDto::new_with_sort(3, 10, Some(dto), vec![]);
    let f = async move {
        rbench!(100000, {
            let count_sql = {
                "select * from test_user where 1 = 1 ".to_owned() + if name.len() > 0 {
                    " and name = ? "
                } else {
                    ""
                }
            };
            let buffer_size = count_sql.len() + 200;
            let mut sql_buf = Vec::<u8>::with_capacity(buffer_size);
            let count_sql = {
                write!(sql_buf, "SELECT count(*) FROM ({}) as __dy_tmp", count_sql).unwrap();
                std::str::from_utf8(&sql_buf).unwrap()
            };
            let query = sqlx::query_scalar(&count_sql);
            let query = query.bind(&name);
            let rst: i32 = query.fetch_one(&mut conn).await.unwrap();
            pg_dto.init(rst as u64);

            let page_sql = {
                "select * from test_user where 1 = 1 ".to_owned() + if name.len() > 0 {
                    " and name = ? "
                } else {
                    ""
                } + " limit 10 offset 1"
            };

            let query = sqlx::query_as::<_, User>(&page_sql);
            let query = query.bind(&name);
            let rst: Vec<User> = query.fetch_all(&mut conn).await.unwrap();

            let _pg_data = Pagination::from_dto(&pg_dto, rst);
        });
    };
    block_on(f);
}


async fn init_rbatis_connection() -> RBatis {
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

    rb
}

async fn init_sqlx_db() -> SqliteConnection {
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

#[derive(Clone, Debug, Serialize, Deserialize, Content)]
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
#[derive(FromRow)]
pub struct User {
    pub id: i64,
    pub name: Option<String>,
    pub age: Option<i32>,
}