use dysql::{Content, fetch_all};
use sqlx::FromRow;

#[tokio::main]
async fn main() {
    let dto = UserDto{ id: None, name: None, age: Some(13) , id_rng: None };
    let _rst = fetch_all!(|dto| -> User {
        r#"SELECT * FROM test_user 
        WHERE 1 = 1
          {{#name}}AND name = :name{{/name}}
          {{#age}}AND age > :age{{/age}}
        ORDER BY id"#
    });
}

#[derive(Content, Clone)]
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