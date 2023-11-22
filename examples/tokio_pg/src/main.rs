
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::{NoTls, connect};
use dysql::*;

#[tokio::main]
async fn main() {
    let mut conn = connect_db().await;
    let tran = conn.transaction().await.unwrap();

    let sql = "select * from test_user where id = $1";
    let mut param_values : Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new(); 

    param_values.push(&1_i64);
    let stmt = tran.prepare(&sql).await.unwrap();
    let params = param_values.into_iter();
    let params = params.as_slice();
    // let row = conn.query_one(&stmt, &[&1_i64]).await.expect("Unexpected error");
    let row = tran.query_one(&stmt, params).await.expect("Unexpected error");
    let rst = User::from_row(row).unwrap();

    println!("{:#?}", rst);
    // tran.rollback();
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

async fn connect_db() -> tokio_postgres::Client {
    let (client, connection) = connect("host=127.0.0.1 user=root password=111111 dbname=my_database", NoTls).await.unwrap();
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client
}
