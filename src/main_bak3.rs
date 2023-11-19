use dysql::{Content, fetch_one, execute, fetch_all};
use sqlx::{FromRow, Postgres, Pool, postgres::PgPoolOptions};

#[tokio::main]
async fn main() {
    let conn = connect_postgres_db().await;
    let mut tran = conn.begin().await.unwrap();

    let dto1 = UserDto{ id: Some(2), name: Some("huanglan".to_owned()), age: Some(13) , id_rng: None };
    let dto2 = dto1.clone();
    let dto3 = dto1.clone();

    let rst = execute!(|&mut *tran, dto1| -> User {
        "update test_user set name = :name where id = :id"
    }).unwrap();
    println!("execute: {:?}", rst);

    let rst = fetch_one!(|&mut *tran, dto2| -> User {
        "select * from test_user where id = :id order by id"
    }).unwrap();
    println!("fetch_one with dto: {:?}", rst);

    let rst = fetch_one!(|&mut *tran| -> User {
        "select * from test_user where id = 1 order by id"
    }).unwrap();
    println!("fetch_one without dto: {:?}", rst);

    let rst = fetch_all!(|&mut *tran| -> User {
        "select * from test_user order by id"
    }).unwrap();
    println!("fetch_all without dto: {:?}", rst);
}

#[derive(Content, Clone)]
struct UserDto {
    id: Option<i64>,
    name: Option<String>,
    age: Option<i32>,
    id_rng: Option<Vec<i32>>,
}

#[derive(Debug, PartialEq)]
#[derive(FromRow)]
struct User {
    id: i64,
    name: Option<String>,
    age: Option<i32>,
}

async fn connect_postgres_db() -> Pool<Postgres> {
    dotenv::dotenv().ok();

    let conn = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://root:111111@127.0.0.1/my_database").await.unwrap();
    // let conn: Pool<Postgres> = PgPool::connect("postgres://root:111111@127.0.0.1/my_database").await.unwrap();

    conn
}