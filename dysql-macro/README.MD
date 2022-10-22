# About Dysql

**Dysql** is a rust crate that adds the ability of dynamic sql to **tokio-postgres** through proc macro. it uses [**Ramhorns**](https://github.com/maciejhirsz/ramhorns) the high performance template engine implementation of [**Mustache**](https://mustache.github.io/) 

Dysql suport **postgres**，**mysql**，**sqlite**，sql dialect.

### Cargo.toml:
```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
tokio-postgres = { version = "0.7", features = ["with-chrono-0_4"] }
dysql = {path = "../../dysql"}
dysql-macro = {path = "../../dysql-macro"}
ramhorns = "0.14"
```

### Example
```rust
use dysql_macro::sql;
use ramhorns::Content;
use tokio_postgres::{NoTls, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Connect to the database.
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=your_username password=your_password dbname=your_database", NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
 
    println!("\ntest_user's data with dysql:\n");

    // use postgres sql dialect 
    let dto = UserDto::new(Some("zhanglan".to_owned()), None, None);
    let (sql, params) = sql!(|dto| { // default sql dialect is "postgres"
        r#"SELECT * FROM test_user 
        WHERE 1 = 1
          {{#name}}AND name = :name{{/name}}
          {{#age}}AND age > :age{{/age}}
        ORDER BY id"#
    });

    //// use mysql sql dialect 
    // let (sql, params) = sql!(|dto| -> mysql { // use "mysql" as sql dialect
    //     r#"SELECT * FROM test_user 
    //     WHERE 1 = 1
    //       {{#name}}AND name = :name{{/name}}
    //       {{#age}}AND age > :age{{/age}}
    //     ORDER BY id"#
    // });

    let rows = client.query(&sql, &params).await?;
    rows.iter().for_each(|row| {
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        let age: i32 = row.get(2);
        println!("id: {}, name: {}, age: {}", id, name, age);
    });
    println!("----------------------------------------");
    println!("sql:{}\ndto:{:?}\n", sql, params);

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
```

### License

Dysql is free software, and is released under the terms of the Apache License version 2. See [LICENSE](LICENSE).