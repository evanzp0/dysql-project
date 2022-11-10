use dysql_macro::{fetch_scalar};
use ramhorns::Content;
use tokio_postgres::{connect, NoTls};

#[derive(Content, Default)]
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

#[derive(Content)]
struct PageDto <'a, T> {
    pub page_size: i32,
    pub page_no: i32,
    pub dto: &'a T
}
impl<'a, T> PageDto<'a, T> {
    pub fn new(page_size: i32, page_no: i32, dto: &'a T) -> Self {
        Self {
            page_size,
            page_no,
            dto
        }
    }
}

#[tokio::main]
async fn main() {
    let conn = connect_db().await;
    let dto = UserDto {
        age: Some(1),
        ..Default::default()
    };

    let pg_dto = PageDto::new(2, 1, &dto);

    let rst = fetch_scalar!(|&pg_dto, &conn| -> (i64, postgres) {
        "select count (*) 
        from test_user
        where 1 = 1
            {{#dto.age}}and age > :dto.age{{/dto.age}}"
    }).unwrap();

    // let rst = extract_params("select count (*) 
    //     from test_user
    //     where 1 = 1 and age > :dto.age", SqlDialect::postgres);

    println!("{:#?}", rst);
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