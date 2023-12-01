mod common;

use std::error::Error;

// 支持 Uuid, Local, NaiveDateTime, Utc, FixedOffSet

use chrono::{Local, FixedOffset, DateTime, NaiveDateTime};
use dysql::{insert, fetch_one, Content};
use uuid::Uuid;


pub async fn connect_postgres_db() -> sqlx::Pool<sqlx::Postgres> {
    dotenv::dotenv().ok();
    let conn = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://root:111111@127.0.0.1/my_database").await.unwrap();
    conn
}

#[tokio::test]
async fn test_fetch_all() {
    let conn = connect_postgres_db().await;

    let dto = TestUser2Dto::new(None, None, None, None, None);
    let rst = fetch_one!(|&conn, &dto| -> TestUser2 {
        r#"SELECT * FROM test_user_2 WHERE name = 'a1' "#
    }).unwrap();

    assert_eq!("a1", &rst.name.unwrap());
}

#[tokio::test]
async fn test_insert() -> Result<(), Box<dyn Error>> {
    let conn = connect_postgres_db().await;
    let mut tran = conn.begin().await?;
    let update_at = Local::now().naive_local();
    let create_at: DateTime<FixedOffset> = DateTime::parse_from_str("2023-11-30 16:00:00.000+0000", "%Y-%m-%d %H:%M:%S%.3f %z").unwrap();

    let dto = TestUser2Dto::new(
        Some(Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap()),
        Some("a1".to_owned()),
        Some(50), 
        Some(create_at), 
        Some(update_at)
    );
    let insert_id = insert!(|&mut tran, dto| -> Uuid {
        r#"insert into test_user_2 (id, name, age, create_at, update_at) values (:id, :name, :age, :create_at, :update_at) returning id"#
    })?;

    assert_eq!("00000000-0000-0000-0000-000000000000", insert_id.to_string());

    tran.rollback().await?;
    Ok(())
}

#[derive(Content, Clone)]
pub struct TestUser2Dto {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub age: Option<i32>,
    pub create_at: Option<DateTime<FixedOffset>>,
    pub update_at: Option<NaiveDateTime>
}

#[allow(dead_code)]
impl TestUser2Dto {
    pub fn new(id: Option<Uuid>, name: Option<String>, age: Option<i32>, create_at: Option<DateTime<FixedOffset>>, update_at: Option<NaiveDateTime>) -> Self {
        Self { id, name, age, create_at, update_at }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
#[derive(sqlx::FromRow)]
pub struct TestUser2 {
    pub id: Uuid,
    pub name: Option<String>,
    pub age: Option<i32>,
    pub create_at: Option<DateTime<FixedOffset>>,
    pub update_at: Option<NaiveDateTime>,
}