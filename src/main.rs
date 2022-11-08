use dysql_macro::{sql, fetch_scalar};
use ramhorns::Content;
use tokio_postgres::{connect, NoTls};

#[derive(Content)]
struct UserDto {
    id: Option<i64>,
    name: Option<String>,
    age: Option<i32>,
}

#[allow(dead_code)]
impl UserDto {
    fn new(id: Option<i64>, name: Option<String>, age: Option<i32>) -> Self {
        Self { id, name,  age }
    }
}

sql!("my_sql", "abcde"); 

#[tokio::main]
async fn main() {
    let conn = connect_db().await;

    let _rst = fetch_scalar!(|_, &conn| -> (i64, postgres) {
        my_sql + r#"select count (*) from test_user"#
    }).unwrap();
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