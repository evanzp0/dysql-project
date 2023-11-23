mod common;

use std::error::Error;

use dysql::{PageDto, SortModel, sql, fetch_one, insert, fetch_scalar, execute, page, fetch_all, Value};
use sqlx::{Pool, MySql, mysql::MySqlPoolOptions};

use crate::common::{UserDto, User};

pub async fn connect_mysql_db() -> Pool<MySql> {
    let conn = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:111111@127.0.0.1/my_database").await.unwrap();

    conn
}

#[tokio::test]
async fn test_fetch_all() {
    let conn = connect_mysql_db().await;

    let dto = UserDto{ id: None, name: None, age: Some(13) , id_rng: None };
    let rst = fetch_all!(|&conn, &dto| -> User {
        r#"SELECT * FROM test_user 
        WHERE 1 = 1
          {{#name}}AND name = :name{{/name}}
          {{#age}}AND age > :age{{/age}}
        ORDER BY id"#
    }).unwrap();
    assert_eq!(7, rst.len());

    let rst = fetch_all!(|&conn| -> User {
        r#"SELECT * FROM test_user"#
    }).unwrap();
    assert_eq!(9, rst.len());
}

sql!("select_sql","select * from test_user ");
#[tokio::test]
async fn test_fetch_one() {
    let conn = connect_mysql_db().await;
    // let dto = UserDto{ id: Some(2), name: None, age: None, id_rng: None };
    let dto = dysql::Value::new(2_i64);

    let rst = fetch_one!(|&conn, dto| -> User {
        select_sql + "where id = :value order by id"
    }).unwrap();
    assert_eq!(User { id: 2, name: Some("zhanglan".to_owned()), age: Some(21) }, rst);

    let rst = fetch_one!(|&conn| -> User {
        select_sql + "where id = 2"
    }).unwrap();
    assert_eq!(User { id: 2, name: Some("zhanglan".to_owned()), age: Some(21) }, rst);
}

#[tokio::test]
async fn test_fetch_scalar() -> dysql::DySqlResult<()>{
    let conn = connect_mysql_db().await;

    let value = Value::new(1);

    let rst = fetch_scalar!(|&conn, value| -> i64 {
        r#"select count (*) from test_user where id = :value"#
    })?;
    assert_eq!(1, rst);

    let rst = fetch_scalar!(|&conn| -> i64 {
        r#"select count (*) from test_user"#
    })?;
    assert_eq!(9, rst);

    Ok(())
}

#[tokio::test]
async fn test_execute() -> Result<(), Box<dyn Error>> {
    let conn = connect_mysql_db().await;
    let mut tran = conn.begin().await?;

    let dto = UserDto{ id: Some(3), name: None, age: None, id_rng: None };
    let affected_rows_num = execute!(|&mut *tran, dto| {
        r#"delete from test_user where id = :id"#
    })?;

    assert_eq!(1, affected_rows_num);
    tran.rollback().await?;

    Ok(())
}

#[tokio::test]
async fn test_insert() -> Result<(), Box<dyn Error>> {
    let conn = connect_mysql_db().await;
    let mut tran = conn.begin().await?;

    let dto = UserDto{ id: None, name: Some("lisi".to_owned()), age: Some(50), id_rng: None };
    let insert_id = insert!(|&mut *tran, dto| -> u64 {
        r#"insert into test_user (name, age) values (:name, :age)"#
    })?;

    assert!(insert_id > 9);
    tran.rollback().await?;
    Ok(())
}

#[tokio::test]
async fn test_page() {
    let conn = connect_mysql_db().await;

    let dto = UserDto{ id: None, name: Some("a".to_owned()), age: Some(13), id_rng: None };
    let sort_model = vec![
        SortModel {field: "id".to_owned(), sort: "desc".to_owned()}
    ];
    let mut pg_dto = PageDto::new_with_sort(3, 10, Some(&dto), sort_model.clone());
    let rst = page!(|&conn, pg_dto| -> User {
        "select * from test_user 
        where 1 = 1
        {{#data}}
            {{#name}}and name like '%' || :data.name || '%'{{/name}}
            {{#age}}and age > :data.age{{/age}}
        {{/data}}"
    }).unwrap();
    assert_eq!(7, rst.total);

    let mut pg_dto = PageDto::new_with_sort(3, 10, Option::<()>::None, sort_model);
    let rst = page!(|&conn, pg_dto| -> User {
        "select * from test_user"
    }).unwrap();
    assert_eq!(9, rst.total);
}

#[tokio::test]
async fn test_trim_sql() {
    let conn = connect_mysql_db().await;
    let dto = UserDto::new(None, Some("z".to_owned()), Some(13), Some(vec![1, 2, 3,]));
    let sort_model = vec![
        SortModel {field: "id".to_owned(), sort: "desc".to_owned()}
    ];
    let mut pg_dto = PageDto::new_with_sort(3, 10, Some(&dto), sort_model);

    let rst = page!(|&conn, pg_dto, "page_test_user"| -> User {
        "select * from test_user 
        where
        {{#data}}
            ![F_DEL(and)]
            {{#name}}and name like '%' || :data.name || '%'{{/name}}
            {{#age}}and age > :data.age{{/age}}
            {{?id_rng}}
                and id in (
                    {{#id_rng}} {{$value}}, {{/id_rng}} ![B_DEL(,)]
                )
            {{/id_rng}}
        {{/data}}"
    }).unwrap();
    // println!("{:?}", rst);

    assert_eq!(2, rst.total);
}