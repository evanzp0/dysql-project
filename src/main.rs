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

#[derive(Content, Debug)]
struct PageDto <'a, T> {
    pub dto: &'a T,
    pub page_size: u64,
    pub page_no: u64,
    pub total_page: Option<u64>,
    pub start: Option<u64>,
    pub total: Option<u64>,
}

#[derive(Content, Debug)]
struct Pagenation <T> {
    pub data: T,
    pub page_size: u64,
    pub page_no: u64,
    pub total_page: u64,
    pub start: u64,
    pub total: u64,
}

impl<'a, T> PageDto<'a, T> 
where T: Content 
{
    pub fn new(page_size: u64, page_no: u64, dto: &'a T) -> Self {
        Self {
            page_size,
            page_no,
            dto,
            total_page: None,
            start: None,
            total: None,
        }
    }

    pub fn init(&mut self, total: u64) {
        let (start, page_no) = self.start(total);
        let total_page = self.total_page(total);

        self.start = Some(start);
        self.total_page = Some(total_page);
        self.total = Some(total);
        self.page_no = page_no;
    }

    fn total_page(&self, total: u64) -> u64 {
        let count: f64 = (total as f64) / self.page_size as f64;
        let count = count.ceil() as u64;
        
        count
    }

    fn start(&self, total: u64) -> (u64, u64) {
        let mut page_no = self.page_no;
        let mut start = self.start_of_page(page_no);

        if start as u64 > total {
            page_no = self.total_page(total);
            start = self.start_of_page(page_no);
            
        }

        (start, page_no)
    }

    fn start_of_page(&self, page_no: u64) -> u64 {
        self.page_size * (page_no - 1) 
    }
}

impl<'a, T> Pagenation<T> {
    pub fn from_dto<Dto>(dto: &PageDto<Dto>, data: T) -> Self {
        Self {
            data,
            page_size: dto.page_size,
            page_no: dto.page_no,
            total_page: dto.total_page.expect("Unexpected error"),
            start: dto.start.expect("Unexpected error"),
            total: dto.total.expect("Unexpected error")
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