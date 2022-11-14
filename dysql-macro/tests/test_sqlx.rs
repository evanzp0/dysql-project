#![cfg(feature = "sqlx")]

use std::{str::FromStr, error::Error};

use dysql::PageDto;
use dysql_macro::*;
use ramhorns::Content;
use sqlx::{
    postgres::PgPoolOptions, FromRow, Pool, Postgres, mysql::MySqlPoolOptions, MySql, 
    sqlite::{SqliteConnectOptions, SqliteJournalMode}, 
    ConnectOptions, SqliteConnection, Acquire
};

#[derive(Content)]
struct UserDto {
    id: Option<i64>,
    name: Option<String>,
    age: Option<i32>
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
#[derive(FromRow)]
struct User {
    id: i64,
    name: Option<String>,
    age: Option<i32>
}

async fn connect_postgres_db() -> Pool<Postgres> {
    let conn = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://root:111111@127.0.0.1/my_database").await.unwrap();

    conn
}

async fn connect_mysql_db() -> Pool<MySql> {
    let conn = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:111111@127.0.0.1/my_database").await.unwrap();

    conn
}

async fn connect_sqlite_db() -> SqliteConnection {
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
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('huanglan', 10)").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('zhanglan', 21)").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('zhangsan', 35)").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('a4', 12)").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('a5', 21)").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('a6', 22)").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('a7', 24)").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('a8', 31)").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('a9', 33)").execute(&mut conn).await.unwrap();
    
    conn
}

#[tokio::test]
async fn test_fetch_all() {
    let conn = connect_postgres_db().await;

    let dto = UserDto{ id: None, name: None, age: Some(13) };
    let rst = fetch_all!(|&dto, &conn| -> User {
        r#"SELECT * FROM test_user 
        WHERE 1 = 1
          {{#name}}AND name = :name{{/name}}
          {{#age}}AND age > :age{{/age}}
        ORDER BY id"#
    }).unwrap();
    assert_eq!(7, rst.len());
}

sql!("select_sql","select * from test_user ");
#[tokio::test]
async fn test_fetch_one() {
    let conn = connect_postgres_db().await;

    let dto = UserDto{ id: Some(2), name: None, age: None };
    let rst = fetch_one!(|&dto, &conn| -> User {
        select_sql + "where 1 = 1 and id = :id order by id"
    }).unwrap();
    assert_eq!(User { id: 2, name: Some("zhanglan".to_owned()), age: Some(21) }, rst);
}

#[tokio::test]
async fn test_fetch_scalar() -> dysql::DySqlResult<()>{
    let conn = connect_postgres_db().await;

    let rst = fetch_scalar!(|_, &conn| -> i64 {
        r#"select count (*) from test_user"#
    })?;

    assert_eq!(9, rst);

    Ok(())
}


#[tokio::test]
async fn test_execute() -> Result<(), Box<dyn Error>> {
    let conn = connect_postgres_db().await;
    let mut tran = conn.begin().await?;

    let dto = UserDto{ id: Some(3), name: None, age: None };
    let affected_rows_num = execute!(|&dto, &mut tran| {
        r#"delete from test_user where id = :id"#
    })?;

    assert_eq!(1, affected_rows_num);
    tran.rollback().await?;

    Ok(())
}


#[tokio::test]
async fn test_insert() -> Result<(), Box<dyn Error>> {
    let conn = connect_postgres_db().await;
    let mut tran = conn.begin().await?;

    let dto = UserDto{ id: Some(10), name: Some("lisi".to_owned()), age: Some(50) };
    let insert_id = insert!(|&dto, &mut tran| {
        r#"insert into test_user (name, age) values (:name, :age) returning id"#
    })?;

    assert!(insert_id > 9);
    tran.rollback().await?;
    Ok(())
}

#[tokio::test]
async fn test_insert_mysql() -> Result<(), Box<dyn Error>> {
    let conn = connect_mysql_db().await;
    let mut tran = conn.begin().await?;

    let dto = UserDto{ id: Some(10), name: Some("lisi".to_owned()), age: Some(50) };
    let insert_id = insert!(|&dto, &mut tran| -> (_, mysql) {
        r#"insert into test_user (name, age) values ('aa', 1)"#
    })?;

    assert!(insert_id > 9);
    tran.rollback().await?;

    Ok(())
}

#[tokio::test]
async fn test_insert_sqlite() -> Result<(), Box<dyn Error>> {
    let mut conn = connect_sqlite_db().await;
    let mut tran = conn.begin().await?;

    let dto = UserDto{ id: Some(10), name: Some("lisi".to_owned()), age: Some(50) };

    let insert_id = insert!(|&dto, &mut tran| -> (_, sqlite) {
        r#"insert into test_user (name, age) values ('aa', 1)"#
    })?;

    assert!(insert_id > 9);
    tran.rollback().await?;

    Ok(())
}

#[tokio::test]
async fn test_page() {
    let conn = connect_postgres_db().await;

    let dto = UserDto{ id: None, name: None, age: Some(13) };
    let mut pg_dto = PageDto::new(3, 10, &dto);
    
    let rst = page!(|&mut pg_dto, &conn| -> User {
        "select * from test_user 
        where 1 = 1
        {{#data}}
            {{#name}}and name = :data.name{{/name}}
            {{#age}}and age > :data.age{{/age}}
        {{/data}}
        order by id"
    }).unwrap();

    assert_eq!(7, rst.total);
}

#[tokio::test]
async fn test_page_mysql() {
    let conn = connect_mysql_db().await;

    let dto = UserDto{ id: None, name: None, age: Some(13) };
    let mut pg_dto = PageDto::new(3, 10, &dto);
    
    let rst = page!(|&mut pg_dto, &conn| -> (User, mysql) {
        "select * from test_user 
        where 1 = 1
        {{#data}}
            {{#name}}and name = :data.name{{/name}}{{/data}}
            {{#data}}{{#age}}and age > :data.age{{/age}}
        {{/data}}
        order by id"
    }).unwrap();

    assert_eq!(7, rst.total);
}

#[tokio::test]
async fn test_page_sqlite() {
    let mut conn = connect_sqlite_db().await;

    let dto = UserDto{ id: None, name: None, age: Some(13) };
    let mut pg_dto = PageDto::new(3, 0, &dto);
    
    let rst = page!(|&mut pg_dto, &mut conn| -> (User, sqlite) {
        "select * from test_user 
        where 1 = 1
        {{#data}}
            {{#name}}and name = :data.name{{/name}}{{/data}}
            {{#data}}{{#age}}and age > :data.age{{/age}}
        {{/data}}
        order by id"
    }).unwrap();
    
    assert_eq!(7, rst.total);
}
