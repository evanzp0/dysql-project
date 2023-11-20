// use std::str::FromStr;

// use dysql::{insert, Content};
// use sqlx::{SqliteConnection, sqlite::{SqliteConnectOptions, SqliteJournalMode}, ConnectOptions, Acquire, FromRow};

// #[tokio::main]
// async fn main() {
//     let mut conn = connect_sqlite_db().await;
//     let mut tran = conn.begin().await.unwrap();

//     let dto = UserDto{ id: None, name: Some("lisi".to_owned()), age: Some(50), id_rng: None };
//     let insert_id = insert!(|&mut *tran, dto| -> u64 {
//         r#"insert into test_user (name, age) values (:name, :age)"#
//     }).unwrap();

//     assert!(insert_id > 9);
//     tran.rollback().await.unwrap();
// }

// async fn connect_sqlite_db() -> SqliteConnection {
//     let mut conn = SqliteConnectOptions::from_str("sqlite::memory:")
//         .unwrap()
//         .journal_mode(SqliteJournalMode::Wal)
//         .read_only(false)
//         .connect()
//         .await.unwrap();

//     // prepare test data
//     sqlx::query("DROP TABLE IF EXISTS test_user").execute(&mut conn).await.unwrap();
//     sqlx::query(r#"
//         CREATE TABLE test_user (
//             id INTEGER PRIMARY KEY AUTOINCREMENT,
//             name VARCHAR(255) NULL,
//             age INT NULL
//         )"#
//     ).execute(&mut conn).await.unwrap();
//     sqlx::query("INSERT INTO test_user (name, age) VALUES ('huanglan', 10)").execute(&mut conn).await.unwrap();
//     sqlx::query("INSERT INTO test_user (name, age) VALUES ('zhanglan', 21)").execute(&mut conn).await.unwrap();
//     sqlx::query("INSERT INTO test_user (name, age) VALUES ('zhangsan', 35)").execute(&mut conn).await.unwrap();
//     sqlx::query("INSERT INTO test_user (name, age) VALUES ('a4', 12)").execute(&mut conn).await.unwrap();
//     sqlx::query("INSERT INTO test_user (name, age) VALUES ('a5', 21)").execute(&mut conn).await.unwrap();
//     sqlx::query("INSERT INTO test_user (name, age) VALUES ('a6', 22)").execute(&mut conn).await.unwrap();
//     sqlx::query("INSERT INTO test_user (name, age) VALUES ('a7', 24)").execute(&mut conn).await.unwrap();
//     sqlx::query("INSERT INTO test_user (name, age) VALUES ('a8', 31)").execute(&mut conn).await.unwrap();
//     sqlx::query("INSERT INTO test_user (name, age) VALUES ('a9', 33)").execute(&mut conn).await.unwrap();
    
//     conn
// }


// #[derive(Content, Clone)]
// struct UserDto {
//     id: Option<i64>,
//     name: Option<String>,
//     age: Option<i32>,
//     id_rng: Option<Vec<i32>>,
// }

// impl UserDto {

// }

// #[allow(dead_code)]
// #[derive(Debug, PartialEq)]
// #[derive(FromRow)]
// struct User {
//     id: i64,
//     name: Option<String>,
//     age: Option<i32>,
// }
tiaojian_macro::tiaojian!(
    |"sqlx1"| {
        use std;
    }
);

fn main() {

}