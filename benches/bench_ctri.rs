
use dysql::{Content, fetch_all, Value};
use criterion::{Criterion, BenchmarkId, criterion_group, criterion_main};
use sqlx::FromRow;

#[derive(Content, Clone)]
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

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
#[derive(FromRow)]
pub struct User {
    pub id: i64,
    pub name: Option<String>,
    pub age: Option<i32>,
}

async fn connect_db() -> sqlx::SqliteConnection {
    use std::str::FromStr;
    use sqlx::ConnectOptions;

    let mut conn = sqlx::sqlite::SqliteConnectOptions::from_str("sqlite::memory:")
        .unwrap()
        .journal_mode( sqlx::sqlite::SqliteJournalMode::Wal)
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
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('a1', 10)").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('a2', 21)").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('a3', 35)").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('a4', 12)").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('a5', 21)").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('a6', 22)").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('a7', 24)").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('a8', 31)").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('a9', 33)").execute(&mut conn).await.unwrap();
    
    conn
}

// Here we have an async function to benchmark
async fn do_fetch_all_dysql_sqlx(db: &std::cell::RefCell<sqlx::SqliteConnection>, dto: &Value<i32>) {
    let mut db = db.borrow_mut();
    let _rst = fetch_all!(|db, &dto| -> User {
        r#"SELECT * FROM test_user WHERE 1 = 1 AND age > :value"#
    }).unwrap();
}

fn fetch_all_dysql_sqlx(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let db = std::cell::RefCell::new(runtime.block_on(connect_db()));
    let dto = Value::new(1);
    c.bench_with_input(
        BenchmarkId::new("sqlx-sqlite + dysql", "fetch_all"),
        &(&db, &dto),
        move |b, param_ref| {
            let &(db, dto) = param_ref;
            b.to_async(&runtime).iter(|| 
                do_fetch_all_dysql_sqlx(db, dto)
            );
        },
    );
}

async fn do_fetch_all_raw_sqlx(db: &std::cell::RefCell<sqlx::SqliteConnection>, dto: &Value<i32>) {
    let sql ="select * from test_user where name = ? ";
    let mut db = db.borrow_mut();
    let query = sqlx::query_as::<_, User>(&sql);
    let query = query.bind(&dto.value);
    let _rst = query.fetch_all(&mut *db).await.unwrap();
}
 
fn fetch_all_raw_sqlx(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let db = std::cell::RefCell::new(runtime.block_on(connect_db()));
    let dto = Value::new(1);
    c.bench_with_input(
        BenchmarkId::new("sqlx-sqlite (raw)", "fetch_all"),
        &(&db, &dto),
        move |b, param_ref| {
            let &(db, dto) = param_ref;
            b.to_async(&runtime).iter(|| 
                do_fetch_all_raw_sqlx(db, dto)
            );
        },
    );
}

criterion_group!(
    benches,
    fetch_all_dysql_sqlx,
    fetch_all_raw_sqlx
);
criterion_main!(benches);

