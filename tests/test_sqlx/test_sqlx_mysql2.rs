mod common;

use std::error::Error;

// 不支持 Uuid, FixedOffSet
// 支持 Local, NaiveDateTime, Utc

use chrono::{Local, DateTime, NaiveDateTime, Utc}; 
use dysql::{insert, fetch_one};
use sqlx::{Pool, MySql, mysql::MySqlPoolOptions};

pub async fn connect_mysql_db() -> Pool<MySql> {
    let conn = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:111111@127.0.0.1/my_database").await.unwrap();

    conn
}

#[tokio::test]
async fn test_fetch_all() {
    let conn = connect_mysql_db().await;

    let dto = TestUser2Dto::new(None, None, None, None, None, None);
    let rst = fetch_one!(|&conn, &dto| -> TestUser2 {
        r#"SELECT * FROM test_user_2 WHERE name = 'a1' "#
    }).unwrap();

    assert_eq!("a1", &rst.name.unwrap());
}

#[tokio::test]
async fn test_insert() -> Result<(), Box<dyn Error>> {
    let conn = connect_mysql_db().await;
    let mut tran = conn.begin().await?;
    let update_at = Local::now().naive_local();
    let update_at2 = Local::now();
    let create_at: DateTime<Utc> = Utc::now();

    let dto = TestUser2Dto::new(
        None,
        Some("a1".to_owned()),
        Some(50), 
        Some(create_at), 
        Some(update_at),
        Some(update_at2)
    );
    let insert_id = insert!(|&mut tran, dto| -> u64 {
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
    pub update_at2: Option<DateTime<Local>>
}

#[allow(dead_code)]
impl TestUser2Dto {
    pub fn new(id: Option<i64>, name: Option<String>, age: Option<i32>, create_at: Option<DateTime<Utc>>, update_at: Option<NaiveDateTime>, update_at2: Option<DateTime<Local>>) -> Self {
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
    pub update_at2: Option<DateTime<Local>>
}