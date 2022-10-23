use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::{NoTls, connect};
use ramhorns::Content;

use dysql_macro::*;

#[tokio::main]
async fn main() -> dysql::DySqlResult<()> {
    let mut conn = connect_db().await;

    let rows = conn.query("select * from test_user", &[]).await?;
    rows.iter().for_each(|row| {
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        let age: i32 = row.get(2);
        println!("id: {}, name: {}, age: {}", id, name, age);
    });

    // fetch all
    let dto = UserDto{ id: None, name: None, age: Some(15) };
    let rst: Vec<User> = fetch_all!(|dto, conn, User| {
        r#"select * from test_user 
        where 1 = 1
            {{#name}}and name = :name{{/name}}
            {{#age}}and age > :age{{/age}}
        order by id"#
    });
    assert_eq!(
        vec![
            User { id: 2, name: Some("zhanglan".to_owned()), age: Some(21) }, 
            User { id: 3, name: Some("zhangsan".to_owned()), age: Some(35) }
        ], 
        rst
    );

    // fetch one
    let dto = UserDto{ id: Some(2), name: None, age: None };
    let rst = fetch_one!(|dto, conn, User| {
        r#"select * from test_user 
        where 1 = 1
            and id = :id
        order by id"#
    });
    assert_eq!(User { id: 2, name: Some("zhanglan".to_owned()), age: Some(21) }, rst);

    // fetch scalar value
    let rst = fetch_scalar!(|_, conn, i64| {
        r#"select count (*) from test_user"#
    });
    assert_eq!(3, rst);

    // execute with transaction
    let tran = conn.transaction().await?;

    let dto = UserDto{ id: Some(3), name: None, age: None };
    let affected_rows_num = execute!(|dto, tran| {
        r#"delete from test_user where id = :id"#
    });
    assert_eq!(1, affected_rows_num);

    tran.rollback().await?;

    // insert with transaction and get id back
    let tran = conn.transaction().await?;

    let dto = UserDto{ id: Some(4), name: Some("lisi".to_owned()), age: Some(50) };
    let insert_id = fetch_scalar!(|dto, &mut tran, i32| {
        r#"insert into test_user (id, name, age) values (:id, :name, :age) returning id"#
    });
    assert_eq!(4, insert_id);
    
    tran.rollback().await?;

    Ok(())
}


#[derive(Content)]
struct UserDto {
    id: Option<i32>,
    name: Option<String>,
    age: Option<i32>
}

#[allow(dead_code)]
#[derive(PostgresMapper, Debug, PartialEq)]
#[pg_mapper(table="test_user")]
struct User {
    id: i32,
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