# About Dysql

**Dysql** is a rust crate that do dynamic-sql query through proc-macro, it bases on [**tokio-postgres**] and [**sqlx**](https://github.com/launchbadge/sqlx) crate (default feature), you can switch them by setting the features. 
It uses [**Ramhorns**](https://github.com/maciejhirsz/ramhorns) the high performance template engine implementation of [**Mustache**](https://mustache.github.io/) 

It invokes like blow:
```
<dysql_macro>!(| <dto>, <conn_or_tran> [, return_type] | [-> dialect] { ...sql string... });
```
> Note: **Dialect can be blank**, and the default value is **postgres**, and dialect also supports  **mysql**, **sqlite**.

## Example (Sqlx)

Full example please see: [Dysql sqlx example](https://github.com/evanzp0/dysql-project/tree/main/examples/with_sqlx)

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
...

#[tokio::main]
async fn main() -> dysql::DySqlResult<()> {
    let conn = connect_postgres_db().await;
    
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
    let affected_rows_num = execute!(|dto, &mut tran| {
        r#"delete from test_user where id = :id"#
    });
    ...

    // insert with transaction and get id back (postgres)
    let insert_id = fetch_scalar!(|dto, &mut tran, i64| {
        r#"insert into test_user (id, name, age) values (:id, :name, :age) returning id"#
    });
    ...

    // insert with transaction and get id back (mysql)
    let dto = UserDto{ id: Some(4), name: Some("lisi".to_owned()), age: Some(50) };
    let insert_id = insert!(|dto, &mut tran| -> mysql {
        r#"insert into test_user (name, age) values ('aa', 1)"#
    });
    ...

    // insert with transaction and get id back (sqlite)
    let dto = UserDto{ id: Some(4), name: Some("lisi".to_owned()), age: Some(50) };
    let insert_id = insert!(|dto, &mut tran| -> sqlite {
        r#"insert into test_user (name, age) values ('aa', 1)"#
    });
    ...

}
```

## Example (tokio-postgres)
Full example please see: [Dysql tokio-postgres example](https://github.com/evanzp0/dysql-project/tree/main/examples/with_tokio_postgres)

### License

Dysql is free software, and is released under the terms of the Apache License version 2. See [LICENSE](LICENSE).