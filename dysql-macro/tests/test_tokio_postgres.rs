#![feature(async_closure)]
#![cfg(feature = "tokio-postgres")]

use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::{NoTls, connect};
use ramhorns::Content;

use dysql_macro::*;

#[derive(Content)]
struct UserDto {
    id: Option<i64>,
    name: Option<String>,
    age: Option<i32>,
}

impl UserDto {
    fn new(id: Option<i64>, name: Option<String>, age: Option<i32>) -> Self {
        Self { id, name,  age }
    }
}

#[allow(dead_code)]
#[derive(PostgresMapper, Debug, PartialEq)]
#[pg_mapper(table="test_user")]
struct User {
    id: i64,
    name: Option<String>,
    age: Option<i32>
}

async fn connect_db() -> tokio_postgres::Client {
    let (client, connection) = connect("host=127.0.0.1 user=root password=111111 dbname=my_database", NoTls).await.unwrap();
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client
}

#[tokio::test]
async fn test_fetch_all() -> dysql::DySqlResult<()> {
    let conn = connect_db().await;
    let dto = UserDto::new(None, None,Some(13));

    let rst = fetch_all!(|dto, conn, User| {
        r#"select * from test_user 
        where 1 = 1
            {{#name}}and name = :name{{/name}}
            {{#age}}and age > :age{{/age}}
        order by id"#
    })?;

    assert_eq!(
        vec![
            User { id: 2, name: Some("zhanglan".to_owned()), age: Some(21) }, 
            User { id: 3, name: Some("zhangsan".to_owned()), age: Some(35) }
        ], 
        rst
    );

    Ok(())
}

#[tokio::test]
async fn test_fetch_one() -> dysql::DySqlResult<()>{
    let conn = connect_db().await;
    let dto = UserDto::new(Some(2), None, None);

    let rst = fetch_one!(|dto, conn, User| {
        r#"select * from test_user 
        where 1 = 1
            and id = :id
        order by id"#
    }).unwrap();

    assert_eq!(User { id: 2, name: Some("zhanglan".to_owned()), age: Some(21) }, rst);

    Ok(())
}

#[tokio::test]
async fn test_fetch_scalar() {
    let conn = connect_db().await;

    let rst = fetch_scalar!(|_, conn, i64| {
        r#"select count (*) from test_user"#
    })
    .unwrap();

    assert_eq!(3, rst);
}

#[tokio::test]
async fn test_execute() {
    let mut conn = connect_db().await;
    let tran = conn.transaction().await.unwrap();

    let dto = UserDto::new(Some(2), None, None);
    let rst = execute!(|dto, tran| {
        r#"delete from test_user where id = :id"#
    }).unwrap();
    assert_eq!(1, rst);

    tran.rollback().await.unwrap();
}

#[tokio::test]
async fn test_insert() {
    let mut conn = connect_db().await;
    let tran = conn.transaction().await.unwrap();

    let dto = UserDto{ id: Some(4), name: Some("lisi".to_owned()), age: Some(50) };
    let insert_id = insert!(|dto, &mut tran| {
        r#"insert into test_user (id, name, age) values (:id, :name, :age) returning id"#
    }).unwrap();
    
    assert!(insert_id > 3);

    tran.rollback().await.unwrap();
}
