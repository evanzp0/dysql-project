use dysql_macro::sql;
use ramhorns::Content;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://root:111111@127.0.0.1/my_database").await?;

    let rows = sqlx::query_as::<_, (i32, String, i32)>("SELECT id, name, age FROM test_user")
        .fetch_all(&pool).await?;

    rows.iter().for_each(|row| {
        println!("id: {}, name: {}, age: {}", row.0, row.1, row.2);
    });
    
    let dto = UserDto::new(None, None, Some(15));
    
    let (sql, params) = sql!(|dto| -> postgres {
        r#"SELECT * FROM test_user 
        WHERE 1 = 1
          {{#name}}AND name = :name{{/name}}
          {{#age}}AND age > :age{{/age}}
        ORDER BY id"#
    });
    
    // todo

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
