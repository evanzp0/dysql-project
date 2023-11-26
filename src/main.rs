#![feature(async_fn_in_trait)]

use dysql::{Content, fetch_all};


#[tokio::main]
async fn main() {
    let mut conn = connect_db().await;
    let dto = UserDto{ id: None, name: Some("a5".to_owned()), age: Some(1), id_rng: None };
    let rst = fetch_all!(|&mut conn, &dto| -> User {
        r#"SELECT * FROM test_user 
        WHERE 1 = 1
          {{#name}}AND name = :name{{/name}}
          {{#age}}AND age > :age{{/age}}
        ORDER BY id"#
    }).unwrap();

    println!("{:#?}", rst);

    // let man = conn.page_a().await;
    // println!("{:#?}", man);

    // let mut tran = conn.begin().await.unwrap();



    // let query = sqlx::query_as::<sqlx::Sqlite, User>("select * from test_user where id = 1");
    
    // let rst = query.fetch_one(&mut conn).await;

    // let rst = conn.page_a().await;
    

    // println!("{:#?}", rst);
    // tran.rollback().await.unwrap();
}

#[derive(Content, Clone, Debug)]
pub struct UserDto {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub age: Option<i32>,
    pub id_rng: Option<Vec<i32>>,
}

impl UserDto {
    pub fn new(id: Option<i64>, name: Option<String>, age: Option<i32>, id_rng: Option<Vec<i32>>) -> Self {
        Self { id, name, age, id_rng }
    }
}

pub trait SqlxTestQuery {
    type DB: sqlx::Database;
    type Row: sqlx::Row<Database = Self::DB>;

    async fn page_a(&mut self)-> User;
}


#[derive(Clone, Debug, PartialEq)]
#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub name: Option<String>,
    pub age: Option<i32>,
}

impl SqlxTestQuery for sqlx::SqliteConnection {
    type DB = sqlx::Sqlite;

    type Row = sqlx::sqlite::SqliteRow;

    async fn page_a(&mut self) -> User
    {
        hello().await;
        // let mut conn = connect_db().await;
        let query = sqlx::query_as::<sqlx::Sqlite, User>("select * from test_user where id = 1");
        let rst = query.fetch_one(self).await.unwrap();

        rst
    }

}

pub async fn hello() {
    println!("hello");
}

pub async fn connect_db() -> sqlx::SqliteConnection {
    use std::str::FromStr;
    use sqlx::ConnectOptions;

    let mut conn = sqlx::sqlite::SqliteConnectOptions::from_str("sqlite::memory:")
        .unwrap()
        .journal_mode( sqlx::sqlite::SqliteJournalMode::Wal)
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


// fn main() {}