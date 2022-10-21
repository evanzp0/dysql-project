use dysql_macro::sql;
use ramhorns::Content;
use tokio_postgres::{NoTls, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Connect to the database.
    let (client, connection) =
        tokio_postgres::connect("host=127.0.0.1 user=root password=111111 dbname=my_database", NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    println!("Database connected!\n");

    println!("test_user's data:\n");
    let rows = client.query("SELECT * FROM test_user", &[]).await?;
    rows.iter().for_each(|row| {
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        let age: i32 = row.get(2);
        println!("id: {}, name: {}, age: {}", id, name, age);
    });
    
    // use dysql macro
    println!("\ntest_user's data with dysql:\n");
    let dto = UserDto::new(Some("zhanglan".to_owned()), None, None);

    let (sql, params) = sql!(|dto| -> postgres {
        r#"SELECT * FROM test_user 
        WHERE 1 = 1
          {{#name}}AND name = :name{{/name}}
          {{#age}}AND age > :age{{/age}}
        ORDER BY id"#
    });

    let rows = client.query(&sql, &params).await?;
    rows.iter().for_each(|row| {
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        let age: i32 = row.get(2);
        println!("id: {}, name: {}, age: {}", id, name, age);
    });
    println!("----------------------------------------");
    println!("sql:{}\ndto:{:?}\n", sql, params);

    let dto = UserDto::new(None, None, Some(15));
    
    let (sql, params) = sql!(|dto| -> postgres {
        r#"SELECT * FROM test_user 
        WHERE 1 = 1
          {{#name}}AND name = :name{{/name}}
          {{#age}}AND age > :age{{/age}}
        ORDER BY id"#
    });
    
    let rows = client.query(&sql, &params).await?;
    rows.iter().for_each(|row| {
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        let age: i32 = row.get(2);
        println!("id: {}, name: {}, age: {}", id, name, age);
    });
    println!("----------------------------------------");
    println!("sql:{}\ndto:{:?}", sql, params);

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
