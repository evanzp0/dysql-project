use dysql::{Content, fetch_one, execute, fetch_all, fetch_scalar, insert, SortModel, PageDto, page};
use sqlx::{FromRow, Postgres, Pool, postgres::PgPoolOptions};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let conn = connect_postgres_db().await;

    let dto1 = UserDto{ id: Some(1), name: Some("huanglan".to_owned()), age: Some(13) , id_rng: None };
    let dto2 = dto1.clone();
    let dto3 = dto1.clone();
    let dto4 = dto1.clone();
    let dto5 = dto1.clone();
    
    let rst = fetch_one!(|&conn, &dto2| -> User {
        "select * from test_user 
        where 1 = 1
            {{#id}} and id = :id  {{/id}}
        order by id
        "}).unwrap();
    println!("fetch_one with dto: {:?}, {:?}", rst, dto2);

    let rst = fetch_one!(|&conn| -> User {
        "select * from test_user where id = 1 order by id"
    }).unwrap();
    println!("fetch_one without dto: {:?}", rst);

    let rst = fetch_all!(|&conn| -> User {
        "select * from test_user order by id"
    }).unwrap();
    println!("fetch_all without dto: {:?}", rst);

    let rst = fetch_scalar!(|&conn, dto4| -> i64 {
        "select count(*) from test_user where id = :id"
    }).unwrap();
    println!("fetch_scalar without dto: {:?}", rst);

    let mut tran = conn.begin().await.unwrap();
    let rst = execute!(|&mut *tran, dto2| -> User {
        "update test_user set name = :name where id = :id"
    }).unwrap();
    println!("execute: {:?}", rst);
    tran.rollback().await.unwrap();

    let mut tran = conn.begin().await.unwrap();
    let rst = insert!(|&mut *tran, dto3| -> i64 {
        "insert into test_user (name, age) values (:name, :age) returning id"
    }).unwrap();
    println!("insert: {:?}", rst);
    tran.rollback().await.unwrap();

    let sort_model = vec![
        SortModel {field: "id".to_owned(), sort: "desc".to_owned()}
    ];
    let mut pg_dto = PageDto::new_with_sort(3, 10, Some(&dto5), sort_model);
    let mut tran = conn.begin().await.unwrap();

    let rst = page!(|&mut *tran, &pg_dto| -> User {
        "select * from test_user 
        where 1 = 1
        {{#data}}
            {{#name}}and name like '%' || :data.name || '%'{{/name}}
            {{#age}}and age > :data.age{{/age}}
        {{/data}}"
    }).unwrap();
    println!("page: {:?} , \n {:?}", rst, pg_dto);
}

#[derive(Content, Clone, Debug)]
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
