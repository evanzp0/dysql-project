use dysql::{sql, fetch_one};
use rbatis::RBatis;
use rbdc_sqlite::Driver;

use crate::common::{User, UserDto};

mod common;

async fn connect_db() -> RBatis {
    let rb = RBatis::new();
    rb.init(Driver{},"sqlite::memory:").unwrap();

    rb.exec(r#"
        CREATE TABLE test_user (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name VARCHAR(255) NULL,
            age INT NULL
        )"#,
        vec![]
    ).await.unwrap();

    rb.exec("INSERT INTO test_user (name, age) VALUES ('huanglan', 10)", vec![]).await.unwrap();
    rb.exec("INSERT INTO test_user (name, age) VALUES ('zhanglan', 21)", vec![]).await.unwrap();
    rb.exec("INSERT INTO test_user (name, age) VALUES ('zhangsan', 35)", vec![]).await.unwrap();
    rb.exec("INSERT INTO test_user (name, age) VALUES ('a4', 12)", vec![]).await.unwrap();
    rb.exec("INSERT INTO test_user (name, age) VALUES ('a5', 21)", vec![]).await.unwrap();
    rb.exec("INSERT INTO test_user (name, age) VALUES ('a6', 22)", vec![]).await.unwrap();
    rb.exec("INSERT INTO test_user (name, age) VALUES ('a7', 24)", vec![]).await.unwrap();
    rb.exec("INSERT INTO test_user (name, age) VALUES ('a8', 31)", vec![]).await.unwrap();
    rb.exec("INSERT INTO test_user (name, age) VALUES ('a9', 33)", vec![]).await.unwrap();

    rb
}

sql!("select_sql","select * from test_user ");
#[tokio::test]
async fn test_fetch_one() {
    let conn = connect_db().await;
    let _dto = UserDto::new(Some(2), None, None, None);
    let dto = dysql::Value::new(2_i64);

    let rst = fetch_one!(|&conn, dto| -> User {
        select_sql + "where id = :value order by id"
    }).unwrap();
    assert_eq!(common::User { id: 2, name: Some("zhanglan".to_owned()), age: Some(21) }, rst);

    let rst = fetch_one!(|&conn| -> User {
        select_sql + "where id = 2"
    }).unwrap();

    assert_eq!(User { id: 2, name: Some("zhanglan".to_owned()), age: Some(21) }, rst);
}