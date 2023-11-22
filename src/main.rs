
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::{NoTls, connect};
use dysql::*;

#[tokio::main]
async fn main() {
    let mut conn = connect_db().await;
    let tran = conn.transaction().await.unwrap();

    let dto = UserDto::new(Some(1), Some("a100".to_owned()), Some(10));

    let rst = insert!(|&tran, dto| -> i64 {
        "insert into test_user (name, age) values (:name, :age) returning id"
    });
    println!("{:#?}", rst);

    let rst = fetch_all!(|&tran| -> User {
        "select * from test_user order by id"
    });
    println!("{:#?}", rst);
    tran.rollback().await.unwrap();
}

#[derive(Content)]
struct UserDto {
    id: Option<i64>,
    name: Option<String>,
    age: Option<i32>,
}

#[allow(dead_code)]
#[derive(PostgresMapper, Debug, PartialEq)]
#[pg_mapper(table="test_user")]
struct User {
    id: i64,
    name: Option<String>,
    age: Option<i32>
}

impl UserDto {
    pub fn new(
        id: Option<i64>,
        name: Option<String>,
        age: Option<i32>,
    ) -> Self {
        Self {
            id,
            name,
            age,
        }
    }
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
