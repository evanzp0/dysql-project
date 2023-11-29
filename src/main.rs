use dysql::{Content, fetch_all};
use tokio_pg_mapper_derive::PostgresMapper;

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();
    env_logger::init();

    let conn = connect_postgres_db().await;
    let dto = UserDto{ id: None, name: None, age: Some(13) , id_rng: None };
    let rst = fetch_all!(|&conn, &dto| -> User {
        r#"SELECT * FROM test_user 
        WHERE 1 = 1
          {{#name}}AND name = :name{{/name}}
          {{#age}}AND age > :age{{/age}}
        ORDER BY id"#
    }).unwrap();
    assert_eq!(7, rst.len());

    let dto = UserDto{ id: None, name: None, age: Some(15) , id_rng: None };
    let rst = fetch_all!(|&conn, &dto| -> User {
        r#"SELECT * FROM test_user 
        WHERE 1 = 1
          {{#name}}AND name = :name{{/name}}
          {{#age}}AND age > :age{{/age}}
        ORDER BY id"#
    }).unwrap();
    assert_eq!(7, rst.len());
}

async fn connect_postgres_db() -> tokio_postgres::Client {
    use tokio_postgres::{NoTls, connect};
    let (client, connection) = connect("host=127.0.0.1 user=root password=111111 dbname=my_database", NoTls).await.unwrap();
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client
}

#[derive(Content, Clone)]
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

#[allow(dead_code)]
#[derive(PostgresMapper, Debug, PartialEq)]
#[pg_mapper(table="test_user")]
pub struct User {
    pub id: i64,
    pub name: Option<String>,
    pub age: Option<i32>
} 

