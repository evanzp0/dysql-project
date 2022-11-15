#![cfg(not(feature = "sqlx"))]

use dysql::{Value, PageDto};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::{NoTls, connect};
use ramhorns::Content;

use dysql_macro::*;

#[derive(Content, Clone, Debug)]
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
async fn test_fetch_all() {
    let conn = connect_db().await;
    let dto = UserDto::new(None, None,Some(13));

    let rst = fetch_all!(|&dto, &conn| -> User {
        r#"select * from test_user 
        where 1 = 1
            {{#name}}and name = :name{{/name}}
            {{#age}}and age > :age{{/age}}
        order by id"#
    }).unwrap();

    assert_eq!(7, rst.len());
}

#[tokio::test]
async fn test_fetch_one() {
    let conn = connect_db().await;
    // let dto = UserDto::new(Some(2), None, None);
    let dto = Value::new(2_i64);

    let rst = fetch_one!(|&dto, &conn, "get_user_by_id"| -> User {
        r#"select * from test_user 
        where 1 = 1
            and id = :value
        order by id"#
    }).unwrap();

    assert_eq!(User { id: 2, name: Some("zhanglan".to_owned()), age: Some(21) }, rst);
}

sql!("count_sql","select count (*)");
#[tokio::test]
async fn test_fetch_scalar() -> dysql::DySqlResult<()>{
    let conn = connect_db().await;

    let rst = fetch_scalar!(|_, &conn| -> (i64, postgres) {
        count_sql + r#" from test_user"#
    }).unwrap();
    assert_eq!(9, rst);

    Ok(())
}

#[tokio::test]
async fn test_execute() {
    let mut conn = connect_db().await;
    let tran = conn.transaction().await.unwrap();

    let dto = UserDto::new(Some(2), None, None);
    let rst = execute!(|&dto, &tran| {
        r#"delete from test_user where id = :id"#
    }).unwrap();
    assert_eq!(1, rst);

    tran.rollback().await.unwrap();
}

#[tokio::test]
async fn test_insert() {
    let mut conn = connect_db().await;
    let tran = conn.transaction().await.unwrap();

    let dto = UserDto{ id: Some(10), name: Some("lisi".to_owned()), age: Some(50) };
    let insert_id = insert!(|&dto, &mut tran| -> (_, _) {
        r#"insert into test_user (id, name, age) values (:id, :name, :age) returning id"#
    }).unwrap();
    
    assert!(insert_id > 9);

    tran.rollback().await.unwrap();
}

#[tokio::test]
async fn test_page() {
    let conn = connect_db().await;
    let dto = UserDto::new(None, Some("a".to_owned()), Some(13));
    let pg_dto = PageDto::new(3, 10, dto);
    let pg_dto = &pg_dto;
    
    let rst = page!(|pg_dto, &conn| -> User {
        "select * from test_user 
        where 1 = 1
        {{#data}}
            {{#name}}and name like '%' || :data.name || '%'{{/name}}
            {{#age}}and age > :data.age{{/age}}
        {{/data}}
        order by id"
    }).unwrap();

    println!("{:?}", rst);

    assert_eq!(7, rst.total);
}
