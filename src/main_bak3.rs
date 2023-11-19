use dysql::{Content, fetch_one};
use sqlx::{FromRow, Postgres, Pool, postgres::PgPoolOptions};

#[tokio::main]
async fn main() {
    let conn = connect_postgres_db().await;
    let mut tran = conn.begin().await.unwrap();

    let dto = UserDto{ id: Some(2), name: None, age: Some(13) , id_rng: None };

    let rst = fetch_one!(|&mut *tran| -> User {
        "select * from test_user where id = 1 order by id"
    }).unwrap();

    println!("{:?}", rst);

    let rst = fetch_one!(|&mut *tran, dto| -> User {
        "select * from test_user where id = :id order by id"
    }).unwrap();

    println!("{:?}", rst);
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