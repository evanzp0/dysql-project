use dysql::{PageDto, Pagenation};
use dysql_macro::{fetch_scalar, fetch_all};
use ramhorns::Content;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::{connect, NoTls};

#[allow(dead_code)]
#[derive(PostgresMapper, Debug, PartialEq)]
#[pg_mapper(table="test_user")]
pub struct User {
    pub id: i64,
    pub name: Option<String>,
    pub age: Option<i32>
}

#[derive(Content, Debug, Default)]
pub struct UserDto {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub age: Option<i32>,
}

#[allow(dead_code)]
impl UserDto {
    fn new(id: Option<i64>, name: Option<String>, age: Option<i32>) -> Self {
        Self { id, name,  age }
    }
}

#[tokio::main]
async fn main() {
    let conn = connect_db().await;
    let dto = UserDto {
        age: Some(1),
        ..Default::default()
    };

    let mut pg_dto = PageDto::new(3, 10, &dto);

    let count = fetch_scalar!(|&pg_dto, &conn| -> (i64, postgres) {
        "select count (*) from test_user"
    }).unwrap();

    pg_dto.init(count as u64);

    let rst = fetch_all!(|&pg_dto, &conn| -> User {
        "select * from test_user limit {{page_size}} offset {{start}}"
    }).unwrap();

    let pg_data = Pagenation::from_dto(&pg_dto, rst);

    println!("{:?}", pg_dto);
    println!("{:#?}", pg_data);
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