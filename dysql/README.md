# About Dysql

**Dysql** is a rust crate that adds the ability of dynamic sql query to **tokio-postgres** through proc macro. 
It uses [**Ramhorns**](https://github.com/maciejhirsz/ramhorns) the high performance template engine implementation of [**Mustache**](https://mustache.github.io/) 

### Cargo.toml:
```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
tokio-postgres = { version = "0.7", features = ["with-chrono-0_4"] }
dysql = "0.2"
dysql-macro = "0.2"
ramhorns = "0.14"
tokio-pg-mapper = "0.2"
tokio-pg-mapper-derive = "0.2"
```

### Example
```rust
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::{NoTls, connect};
use ramhorns::Content;

use dysql_macro::*;

#[tokio::main]
async fn main() -> dysql::DySqlResult<()> {
    let mut conn = connect_db().await;


    // fetch all
    let dto = UserDto::new(None, None,Some(13));
    let rst: Vec<User> = fetch_all!(|dto, conn, User| {
        r#"select * from test_user 
        where 1 = 1
            {{#name}}and name = :name{{/name}}
            {{#age}}and age > :age{{/age}}
        order by id"#
    });
    assert_eq!(
        vec![
            User { id: 2, name: Some("zhanglan".to_owned()), age: Some(21) }, 
            User { id: 3, name: Some("zhangsan".to_owned()), age: Some(35) }
        ], 
        rst
    );

    // fetch one
    let dto = UserDto::new(Some(2), None, None);
    let rst = fetch_one!(|dto, conn, User| {
        r#"select * from test_user 
        where 1 = 1
            and id = :id
        order by id"#
    });
    assert_eq!(User { id: 2, name: Some("zhanglan".to_owned()), age: Some(21) }, rst);

    // fetch scalar value
    let rst = fetch_scalar!(|_, conn, i64| {
        r#"select count (*) from test_user"#
    });
    assert_eq!(3, rst);

    // execute with transaction
    let tran = conn.transaction().await?;
    let dto = UserDto::new(Some(2), None, None);
    let rst = execute!(|dto, tran| {
        r#"delete from test_user where id = :id"#
    });
    assert_eq!(1, rst);
    tran.rollback().await?;


    Ok(())
}


#[derive(Content)]
struct UserDto {
    id: Option<i32>,
    name: Option<String>,
    age: Option<i32>
}

impl UserDto {
    fn new(id: Option<i32>, name: Option<String>, age: Option<i32>) -> Self {
        Self { id, name, age }
    }
}

#[allow(dead_code)]
#[derive(PostgresMapper, Debug, PartialEq)]
#[pg_mapper(table="test_user")]
struct User {
    id: i32,
    name: Option<String>,
    age: Option<i32>
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
```

### License

Dysql is free software, and is released under the terms of the Apache License version 2. See [LICENSE](LICENSE).