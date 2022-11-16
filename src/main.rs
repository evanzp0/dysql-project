use dysql::PageDto;
use dysql_macro::page;
use ramhorns::Content;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::{connect, NoTls};

#[tokio::main]
async fn main() {
    let conn = connect_db().await;
    let dto = UserDto{ id: None, name: None, age: Some(13) };
    let mut pg_dto = PageDto::new(3, 10, &dto);
    
    let rst = page!(|&mut pg_dto, &conn| -> User {
        "select * from test_user 
        where 1 = 1
        {{#data}}
            {{#name}}and name = :data.name{{/name}}
            {{#age}}and age > :data.age{{/age}}
        {{/data}}
        order by id"
    }).unwrap();
    assert_eq!(7, rst.total);

}

#[derive(Content)]
struct UserDto {
    id: Option<i64>,
    name: Option<String>,
    age: Option<i32>
}

#[allow(dead_code)]
#[derive(PostgresMapper, Debug, PartialEq)]
#[pg_mapper(table="test_user")]
struct User {
    id: i64,
    name: Option<String>,
    age: Option<i32>
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