# About Dysql

**Dysql** is a rust crate that do dynamic-sql query through proc-macro, it bases on [**tokio-postgres**](https://github.com/sfackler/rust-postgres) (default feature) and [**sqlx**](https://github.com/launchbadge/sqlx) crate, you can switch them by setting the features. 
It uses [**Ramhorns**](https://github.com/maciejhirsz/ramhorns) the high performance template engine implementation of [**Mustache**](https://mustache.github.io/) 

It invokes like blow:
```
<dysql_macro>!(| <dto>, <conn_or_tran> [, return_type] | [-> dialect] { ...sql string... });
```
> Note: **Dialect can be blank**, and the default value is **postgres**, and dialect also supports  **mysql**, **sqlite**.

## Example (Sqlx)

### Cargo.toml:
```toml
[dependencies]
dysql = "0.3"
dysql-macro = {version = "0.3", features = ["sqlx"]}
sqlx = { version = "0.6", features = [ "runtime-tokio-native-tls" , "postgres" ] }
tokio = { version = "1.0", features = ["full"] }
ramhorns = "0.14"
tokio-postgres = { version = "0.7", features = ["with-chrono-0_4"] }
```

### main.rs
```rust
use dysql_macro::{fetch_all, fetch_one, fetch_scalar, execute};
use ramhorns::Content;
use sqlx::{postgres::PgPoolOptions, FromRow};

#[tokio::main]
async fn main() -> dysql::DySqlResult<()> {
    let conn = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://root:111111@127.0.0.1/my_database").await?;

    let rows = sqlx::query_as::<_, (i32, String, i32)>("SELECT id, name, age FROM test_user")
        .fetch_all(&conn).await?;

    rows.iter().for_each(|row| {
        println!("id: {}, name: {}, age: {}", row.0, row.1, row.2);
    });
    
    // fetch all
    let dto = UserDto{ id: None, name: None, age: Some(15) };
    let rst = fetch_all!(|dto, conn, User| {
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

    // fetch one
    let dto = UserDto{ id: Some(2), name: None, age: None };
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
    let mut tran = conn.begin().await?;
    let dto = UserDto{ id: Some(3), name: None, age: None };
    let affected_rows_num = execute!(|dto, &mut tran| {
        r#"delete from test_user where id = :id"#
    });

    assert_eq!(1, affected_rows_num);
    tran.rollback().await?;

    // insert with transaction and get id back (postgres only)
    let mut tran = conn.begin().await?;
    let dto = UserDto{ id: Some(4), name: Some("lisi".to_owned()), age: Some(50) };
    let insert_id = fetch_scalar!(|dto, &mut tran, i32| {
        r#"insert into test_user (id, name, age) values (:id, :name, :age) returning id"#
    });
    assert_eq!(4, insert_id);
    tran.rollback().await?;

    //// insert with transaction and get id back (except postgres)
    // let mut tran = conn.begin().await?;
    // let dto = UserDto{ id: Some(4), name: Some("lisi".to_owned()), age: Some(50) };
    // let last_insert_id = insert!(|dto, tran| -> mysql {
    //     r#"insert into test_user (id, name, age) values (4, 'aa', 1)"#
    // });
    // assert_eq!(4, last_insert_id);
    // tran.rollback().await?;

    Ok(())
}

#[derive(Content)]
struct UserDto {
    id: Option<i32>,
    name: Option<String>,
    age: Option<i32>
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
#[derive(FromRow)]
struct User {
    id: i32,
    name: Option<String>,
    age: Option<i32>
}

```

## Example (tokio-postgres)
please see: [Dysql tokio-postgres example](https://github.com/evanzp0/dysql-project/tree/main/examples/with_tokio_postgres)

### License

Dysql is free software, and is released under the terms of the Apache License version 2. See [LICENSE](LICENSE).