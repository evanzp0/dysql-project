mod common;

use std::error::Error;

// 不支持 Uuid
// 支持 Local, FixedOffSet, NaiveDateTime, Utc

use chrono::{Local, DateTime, NaiveDateTime, Utc, FixedOffset, TimeZone}; 
use dysql::{insert, fetch_one};
use sqlx::Acquire;

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
    sqlx::query("DROP TABLE IF EXISTS test_user_2").execute(&mut conn).await.unwrap();
    sqlx::query(r#"
        CREATE TABLE test_user_2 (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name VARCHAR(255) NULL,
            age INT NULL,
            create_at TIMESTAMP NULL,
            update_at TIMESTAMP NULL,
            update_at2 TIMESTAMP NULL
        )"#
    ).execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user_2 (name, age, create_at, update_at, update_at2) VALUES ('a1', 10, '2023-12-01 00:00:00', '2023-12-01 00:00:00', '2023-12-01 00:00:00')").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user_2 (name, age, create_at, update_at, update_at2) VALUES ('a2', 21, '2023-12-01 00:00:00', '2023-12-01 00:00:00', '2023-12-01 00:00:00')").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user_2 (name, age, create_at, update_at, update_at2) VALUES ('a3', 35, '2023-12-01 00:00:00', '2023-12-01 00:00:00', '2023-12-01 00:00:00')").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user_2 (name, age, create_at, update_at, update_at2) VALUES ('a4', 12, '2023-12-01 00:00:00', '2023-12-01 00:00:00', '2023-12-01 00:00:00')").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user_2 (name, age, create_at, update_at, update_at2) VALUES ('a5', 21, '2023-12-01 00:00:00', '2023-12-01 00:00:00', '2023-12-01 00:00:00')").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user_2 (name, age, create_at, update_at, update_at2) VALUES ('a6', 22, '2023-12-01 00:00:00', '2023-12-01 00:00:00', '2023-12-01 00:00:00')").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user_2 (name, age, create_at, update_at, update_at2) VALUES ('a7', 24, '2023-12-01 00:00:00', '2023-12-01 00:00:00', '2023-12-01 00:00:00')").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user_2 (name, age, create_at, update_at, update_at2) VALUES ('a8', 31, '2023-12-01 00:00:00', '2023-12-01 00:00:00', '2023-12-01 00:00:00')").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user_2 (name, age, create_at, update_at, update_at2) VALUES ('a9', 33, '2023-12-01 00:00:00', '2023-12-01 00:00:00', '2023-12-01 00:00:00')").execute(&mut conn).await.unwrap();
    
    conn
}

#[tokio::test]
async fn test_fetch_one() {
    let mut conn = connect_db().await;
    let dto = TestUser2Dto::new(None, None, None, None, None, None);

    let rst = fetch_one!(|&mut conn, dto| -> TestUser2 {
        "SELECT * FROM test_user_2 WHERE name = 'a1'"
    }).unwrap();

    assert_eq!("a1", &rst.name.unwrap());
}

#[tokio::test]
async fn test_insert() -> Result<(), Box<dyn Error>> {
    let mut conn = connect_db().await;
    let mut tran = conn.begin().await?;
    let update_at = Local::now().naive_local();
    let update_at2 = FixedOffset::east_opt(5 * 1)
        .unwrap()
        .with_ymd_and_hms(2016, 11, 08, 0, 0, 0)
        .unwrap();
    let create_at: DateTime<Utc> = Utc::now();

    let dto = TestUser2Dto::new(
        None,
        Some("a1".to_owned()),
        Some(50), 
        Some(create_at), 
        Some(update_at),
        Some(update_at2)
    );
    let insert_id = insert!(|&mut *tran, dto| -> i32 {
        r#"insert into test_user_2 (name, age, create_at, update_at) values (:name, :age, :create_at, :update_at)"#
    })?;

    assert!(insert_id > 1);
    tran.rollback().await?;
    Ok(())
}

#[derive(dysql::Content, Clone)]
pub struct TestUser2Dto {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub age: Option<i32>,
    pub create_at: Option<DateTime<Utc>>,
    pub update_at: Option<NaiveDateTime>,
    pub update_at2: Option<DateTime<FixedOffset>>
}

#[allow(dead_code)]
impl TestUser2Dto {
    pub fn new(id: Option<i64>, name: Option<String>, age: Option<i32>, create_at: Option<DateTime<Utc>>, update_at: Option<NaiveDateTime>, update_at2: Option<DateTime<FixedOffset>>) -> Self {
        Self { id, name, age, create_at, update_at, update_at2 }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
#[derive(sqlx::prelude::FromRow)]
pub struct TestUser2 {
    pub id: i64,
    pub name: Option<String>,
    pub age: Option<i32>,
    pub create_at: Option<DateTime<Utc>>,
    pub update_at: Option<NaiveDateTime>,
    pub update_at2: Option<DateTime<FixedOffset>>
}