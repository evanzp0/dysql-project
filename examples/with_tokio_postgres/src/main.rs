use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::{NoTls, connect};
use ramhorns::Content;

use dysql_macro::*;

sql!("select_sql", "select * from test_user ");

#[tokio::main]
async fn main() {
    let mut conn = connect_db().await;
    // fetch all
    let dto = UserDto{ id: None, name: None, age: Some(15) };
    let rst: Vec<User> = fetch_all!(|&dto, &conn| -> User {
        select_sql + 
        r#"where 1 = 1
            {{#name}}and name = :name{{/name}}
            {{#age}}and age > :age{{/age}}
        order by id"#
    }).unwrap();
    assert_eq!(
        vec![
            User { id: 2, name: Some("zhanglan".to_owned()), age: Some(21) }, 
            User { id: 3, name: Some("zhangsan".to_owned()), age: Some(35) }
        ], 
        rst
    );

    // fetch one
    let dto = UserDto{ id: Some(2), name: None, age: None };
    let rst = fetch_one!(|&dto, &conn| -> User {
        r#"select * from test_user 
        where 1 = 1
            and id = :id
        order by id"#
    }).unwrap();
    assert_eq!(User { id: 2, name: Some("zhanglan".to_owned()), age: Some(21) }, rst);

    // fetch scalar value
    let rst = fetch_scalar!(|_, &conn| -> i64 {
        r#"select count (*) from test_user"#
    }).unwrap();
    assert_eq!(3, rst);

    // execute with transaction
    let tran = conn.transaction().await.unwrap();

    let dto = UserDto{ id: Some(3), name: None, age: None };
    let affected_rows_num = execute!(|&dto, &tran| {
        r#"delete from test_user where id = :id"#
    }).unwrap();
    assert_eq!(1, affected_rows_num);

    tran.rollback().await.unwrap();

    // insert with transaction and get id back
    let tran = conn.transaction().await.unwrap();

    let dto = UserDto{ id: Some(4), name: Some("lisi".to_owned()), age: Some(50) };
    //// Here return type is omitted because default return type of insert_id is i64. 
    //// if the return type is others, you should give a specific type.
    let insert_id = insert!(|&dto, &mut tran| { 
        r#"insert into test_user (id, name, age) values (:id, :name, :age) returning id"#
    }).unwrap();
    assert!(insert_id > 3);
    
    tran.rollback().await.unwrap();

}


#[derive(Content)]
struct UserDto {
    id: Option<i64>,
    name: Option<String>,
    age: Option<i32>
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