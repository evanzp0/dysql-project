use dysql_macro::fetch_all;
use ramhorns::Content;
use sqlx::{postgres::PgPoolOptions, FromRow};

#[tokio::main]
async fn main() -> dysql::DySqlResult<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://root:111111@127.0.0.1/my_database").await?;

    let rows = sqlx::query_as::<_, (i32, String, i32)>("SELECT id, name, age FROM test_user")
        .fetch_all(&pool).await?;

    rows.iter().for_each(|row| {
        println!("id: {}, name: {}, age: {}", row.0, row.1, row.2);
    });
    
    let dto = UserDto::new(None, None, Some(15));
    
    let rst = fetch_all!(|dto, pool, User| {
        r#"SELECT * FROM test_user 
        WHERE 1 = 1
          {{#name}}AND name = :name{{/name}}
          {{#age}}AND age > :age{{/age}}
        ORDER BY id"#
    });
    assert_eq!(
        vec![
            User { id: 2, name: Some("zhanglan".to_owned()), age: Some(21) }, 
            User { id: 3, name: Some("zhangsan".to_owned()), age: Some(35) }
        ], 
        rst
    );

    Ok(())
}

#[derive(Content)]
struct UserDto {
    name: Option<String>,
    id: Option<i32>,
    age: Option<i32>
}

impl UserDto {
    fn new(name: Option<String>, id: Option<i32>, age: Option<i32>) -> Self {
        Self { name, id, age }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
#[derive(FromRow)]
struct User {
    id: i32,
    name: Option<String>,
    age: Option<i32>
}
