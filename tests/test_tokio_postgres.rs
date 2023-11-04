// #![cfg(feature = "tokio-postgres")]

use dysql::{PageDto, SortModel, fetch_all, insert, sql, fetch_one, fetch_scalar, execute, page, Content};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::{NoTls, connect};

#[derive(Content)]
struct UserDto {
    id: Option<i64>,
    name: Option<String>,
    age: Option<i32>,
    id_rng: Option<Vec<i32>>,
}

impl UserDto {
    fn new(id: Option<i64>, name: Option<String>, age: Option<i32>, id_rng: Option<Vec<i32>>) -> Self {
        Self { id, name, age, id_rng }
    }
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
    dotenv::dotenv().ok();
    
    let (client, connection) = connect("host=127.0.0.1 user=root password=111111 dbname=my_database", NoTls).await.unwrap();
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client
}

#[tokio::test]
async fn test_fetch_all() {
    let conn = connect_db().await;
    let dto = UserDto::new(None, None, Some(13), None);

    let rst = fetch_all!(|&dto, &conn| -> User {
        r#"select * from test_user 
        where 1 = 1
            {{#name}}and name = :name{{/name}}
            {{#age}}and age > :age{{/age}}
        order by id"#
    }).unwrap();

    assert_eq!(7, rst.len());
}

#[tokio::test]
async fn test_fetch_one() {
    let conn = connect_db().await;
    // let dto = UserDto::new(Some(2), None, None, None);
    let dto = dysql::Value::new(2_i64);

    let rst = fetch_one!(|&dto, &conn, "get_user_by_id"| -> User {
        r#"select * from test_user 
        where 1 = 1
            and id = :value
        order by id"#
    }).unwrap();

    assert_eq!(User { id: 2, name: Some("zhanglan".to_owned()), age: Some(21) }, rst);
}

sql!("count_sql","select count (*)");
#[tokio::test]
async fn test_fetch_scalar() -> dysql::DySqlResult<()>{
    let conn = connect_db().await;

    let rst = fetch_scalar!(|_, &conn| -> (i64, postgres) {
        count_sql + r#" from test_user"#
    }).unwrap();
    assert_eq!(9, rst);

    Ok(())
}

#[tokio::test]
async fn test_execute() {
    let mut conn = connect_db().await;
    let tran = conn.transaction().await.unwrap();
    
    let dto = UserDto::new(Some(2), None, None, None);
    let rst = execute!(|&dto, &tran| {
        r#"delete from test_user where id = :id"#
    }).unwrap();
    assert_eq!(1, rst);

    tran.rollback().await.unwrap();
}

#[tokio::test]
async fn test_insert() {
    let mut conn = connect_db().await;
    let tran = conn.transaction().await.unwrap();

    let dto = UserDto{ id: None, name: Some("lisi".to_owned()), age: Some(50), id_rng: None};
    let insert_id = insert!(|&dto, &mut tran| {
        r#"insert into test_user (name, age) values (:name, :age) returning id"#
    }).unwrap();
    
    assert!(insert_id > 9);

    tran.rollback().await.unwrap();
}

#[tokio::test]
async fn test_page() {
    let conn = connect_db().await;
    let dto = UserDto{ id: None, name: Some("a".to_owned()), age: Some(13), id_rng: None };
    let sort_model = vec![
        SortModel {field: "id".to_owned(), sort: "desc".to_owned()}
    ];
    let mut pg_dto = PageDto::new_with_sort(3, 10, dto, sort_model);
    
    let rst = page!(|&mut pg_dto, &conn| -> User {
        "select * from test_user 
        where 1 = 1
        {{#data}}
            {{#name}}and name like '%' || :data.name || '%'{{/name}}
            {{#age}}and age > :data.age{{/age}}
        {{/data}}"
    }).unwrap();

    assert_eq!(7, rst.total);
}

#[tokio::test]
async fn test_trim_sql() {
    let conn = connect_db().await;
    let dto = UserDto::new(None, Some("z".to_owned()), Some(13), Some(vec![1, 2, 3]));
    let sort_model = vec![
        SortModel {field: "id".to_owned(), sort: "desc".to_owned()}
    ];
    let mut pg_dto = PageDto::new_with_sort(3, 10, dto, sort_model);
    let pg_dto = &mut pg_dto;
    
    let rst = page!(|pg_dto, &conn| -> User {
        "select * from test_user 
        where
        {{#data}}
            ![F_DEL(and)]
            {{#name}}and name like '%' || :data.name || '%'{{/name}}
            {{#age}}and age > :data.age{{/age}}
            {{?id_rng}}
                and id in (
                    {{#id_rng}} {{$value}}, {{/id_rng}} ![B_DEL(,)]
                )
            {{/id_rng}}
        {{/data}}"
    }).unwrap();
    // println!("{:?}", rst);

    assert_eq!(2, rst.total);
}
