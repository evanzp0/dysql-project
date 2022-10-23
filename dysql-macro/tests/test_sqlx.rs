#![cfg(feature = "sqlx")]

use dysql_macro::{fetch_all, fetch_one, fetch_scalar, execute};
use ramhorns::Content;
use sqlx::{postgres::PgPoolOptions, FromRow, Pool, Postgres};

#[derive(Content)]
struct UserDto {
    id: Option<i32>,
    name: Option<String>,
    age: Option<i32>
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
#[derive(FromRow)]
struct User {
    id: i32,
    name: Option<String>,
    age: Option<i32>
}

async fn connect_db() -> Pool<Postgres> {
    let conn = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://root:111111@127.0.0.1/my_database").await.unwrap();

    conn
}

#[tokio::test]
async fn test_fetch_all() -> dysql::DySqlResult<()>{
    let conn = connect_db().await;

    let dto = UserDto{ id: None, name: None, age: Some(15) };
    let rst = fetch_all!(|dto, conn, User| {
        r#"SELECT * FROM test_user 
        WHERE 1 = 1
          {{#name}}AND name = :name{{/name}}
          {{#age}}AND age > :age{{/age}}
        ORDER BY id"#
    });
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

    let dto = UserDto{ id: Some(2), name: None, age: None };
    let rst = fetch_one!(|dto, conn, User| {
        r#"select * from test_user 
        where 1 = 1
            and id = :id
        order by id"#
    });
    assert_eq!(User { id: 2, name: Some("zhanglan".to_owned()), age: Some(21) }, rst);


    Ok(())
}

#[tokio::test]
async fn test_fetch_scalar() -> dysql::DySqlResult<()>{
    let conn = connect_db().await;

    let rst = fetch_scalar!(|_, conn, i64| {
        r#"select count (*) from test_user"#
    });
    assert_eq!(3, rst);

    Ok(())
}


#[tokio::test]
async fn test_execute() -> dysql::DySqlResult<()>{
    let conn = connect_db().await;
    let mut tran = conn.begin().await?;

    let dto = UserDto{ id: Some(3), name: None, age: None };
    let affected_rows_num = execute!(|dto, &mut tran| {
        r#"delete from test_user where id = :id"#
    });

    assert_eq!(1, affected_rows_num);
    tran.rollback().await?;

    Ok(())
}


#[tokio::test]
async fn test_insert() -> dysql::DySqlResult<()>{
    let conn = connect_db().await;
    let mut tran = conn.begin().await?;

    let dto = UserDto{ id: Some(4), name: Some("lisi".to_owned()), age: Some(50) };
    let insert_id = fetch_scalar!(|dto, &mut tran, i32| {
        r#"insert into test_user (id, name, age) values (:id, :name, :age) returning id"#
    });
    assert_eq!(4, insert_id);
    tran.rollback().await?;

    Ok(())
}

// #[tokio::test]
// async fn test_insert_mysql() -> dysql::DySqlResult<()>{
//     let conn = connect_db().await;
//     let mut tran = conn.begin().await?;

//     let dto = UserDto{ id: Some(4), name: Some("lisi".to_owned()), age: Some(50) };
//     let last_insert_id = insert!(|dto, tran| -> mysql {
//         r#"insert into test_user (id, name, age) values (4, 'aa', 1)"#
//     });
//     assert_eq!(4, last_insert_id);
//     tran.rollback().await?;

//     Ok(())
// }

