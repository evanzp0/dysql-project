// use dysql::{Content, fetch_all};
// use sqlx::FromRow;
// use tokio_pg_mapper_derive::PostgresMapper;

// #[tokio::main]
// async fn main() {
//     dotenv::dotenv().unwrap();
//     env_logger::init();

//     let mut conn = connect_db().await;
//     let dto = UserDto{ id: None, name: None, age: Some(13) , id_rng: None };
//     let rst = fetch_all!(|&mut conn, &dto| -> User {
//         r#"SELECT * FROM test_user 
//         WHERE 1 = 1
//           {{#name}}AND name = :name{{/name}}
//           {{#age}}AND age > :age{{/age}}
//         ORDER BY id"#
//     }).unwrap();
//     assert_eq!(7, rst.len());

//     let dto = UserDto{ id: None, name: None, age: Some(15) , id_rng: None };
//     let rst = fetch_all!(|&mut conn, &dto| -> User {
//         r#"SELECT * FROM test_user 
//         WHERE 1 = 1
//           {{#name}}AND name = :name{{/name}}
//           {{#age}}AND age > :age{{/age}}
//         ORDER BY id"#
//     }).unwrap();
//     assert_eq!(7, rst.len());
// }

// async fn connect_db() -> sqlx::SqliteConnection {
//     use std::str::FromStr;
//     use sqlx::ConnectOptions;

//     let mut conn = sqlx::sqlite::SqliteConnectOptions::from_str("sqlite::memory:")
//         .unwrap()
//         .journal_mode( sqlx::sqlite::SqliteJournalMode::Wal)
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
// pub struct UserDto {
//     pub id: Option<i64>,
//     pub name: Option<String>,
//     pub age: Option<i32>,
//     pub id_rng: Option<Vec<i32>>,
// }

// impl UserDto {
//     pub fn new(id: Option<i64>, name: Option<String>, age: Option<i32>, id_rng: Option<Vec<i32>>) -> Self {
//         Self { id, name, age, id_rng }
//     }
// }

// #[allow(dead_code)]
// #[derive(PostgresMapper, Debug, PartialEq)]
// #[pg_mapper(table="test_user")]
// #[derive(FromRow)]
// pub struct User {
//     pub id: i64,
//     pub name: Option<String>,
//     pub age: Option<i32>
// } 

fn main() {}