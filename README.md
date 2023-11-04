# About Dysql

**Dysql** is a highly Performant Rust SQL Toolkit and ORM Library. An async, pure Rust SQL crate featuring compile-time Dynamic SQL.

It bases on [**sqlx**](https://github.com/launchbadge/sqlx) crate (default feature), you can switch them by setting the features. 
It uses [**Ramhorns-ext**](https://github.com/maciejhirsz/ramhorns) the high performance template engine implementation of [**Mustache**](https://mustache.github.io/) template language (a very very very simply template language) :-).

It invokes like blow:
```(
dysql_macro!(| dto, conn_or_tran | [-> return_type | -> (return_type ,dialect)] { ...sql string... });
```
> Note: **Dialect can be blank**, and the default value is **postgres**, and dialect also supports  **mysql**, **sqlite**.

## TODO
working_dir 迭代查找 .dysql 处理

## Example
Full example please see: [Dysql sqlx example](https://github.com/evanzp0/dysql-project/tests)

### Cargo.toml:
```toml
[dependencies]
dysql = "0.11"
sqlx = { version = "0.6", features = [ "runtime-tokio-native-tls" , "postgres" ] }
tokio = { version = "1.0", features = ["full"] }
```

### main.rs
```rust
...

#[tokio::main]
async fn main() {
    let conn = connect_postgres_db().await;
    sql!("select_sql_fragment_1", "SELECT * FROM test_user ")

    // fetch all
    let dto = UserDto{ id: None, name: None, age: Some(15) };
    let rst = fetch_all!(|&dto, &conn| -> User {
        select_sql_fragment_1 +
        r#"WHERE 1 = 1
          {{#name}}AND name = :name{{/name}}
          {{#age}}AND age > :age{{/age}}
        ORDER BY id"#
    }).unwrap();
    assert_eq!(
        vec![
            User { id: 2, name: Some("zhanglan".to_owned()), age: Some(21) }, 
            User { id: 3, name: Some("zhangsan".to_owned()), age: Some(35) }
        ], 
        rst
    );

    // fetch one

    let dto = dysql::Value::new(2_i64); // use value wrapper object
    let rst = fetch_one!(|&dto, &conn| -> User {
        select_sql_fragment_1 +
        r#"where 1 = 1
            and id = :value
        order by id"#
    }).unwrap();
    assert_eq!(User { id: 2, name: Some("zhanglan".to_owned()), age: Some(21) }, rst);

    // fetch scalar value
    let rst = fetch_scalar!(|_, &conn| -> i64 {
        r#"select count (*) from test_user"#
    }).unwrap();
    assert_eq!(3, rst);

    // execute with transaction
    let affected_rows_num = execute!(|&dto, &mut tran| {
        r#"delete from test_user where id = :id"#
    }).unwrap();
    ...

    // insert with transaction and get id back (postgres)
    let insert_id = insert!(|&dto, &mut tran| {
        r#"insert into test_user (id, name, age) values (:id, :name, :age) returning id"#
    }).unwrap();
    ...

    // insert with transaction and get id back (mysql / sqlite)
    let dto = UserDto{ id: Some(4), name: Some("lisi".to_owned()), age: Some(50) };
    let insert_id = insert!(|dto, &mut tran| -> (_, mysql) { // you can use 'sqlite' replace the 'mysql' dialect
        r#"insert into test_user (name, age) values ('aa', 1)"#
    }).unwrap();
    ...

    // page query
    let dto = UserDto{ id: None, name: None, age: Some(13) };
    let mut pg_dto = PageDto::new(3, 10, &dto);
    let rst = page!(|&mut pg_dto, &conn| -> User {
        "select * from test_user 
        where 1 = 1
        {{#data}}
            {{#name}}and name = :data.name{{/name}}
            {{#age}}and age > :data.age{{/age}}
        {{/data}}
        order by id"
    }).unwrap();
    assert_eq!(7, rst.total);
    ...

    // trim sql by ![F_DEL(xxx)] and ![B_DEL(xxx)]
    let rst = fetch_all!(|pg_dto, &conn| -> User {
        // after trim the sql is: 
        //   "select * from test_user where name like '%' || :data.name || '%' and age > :data.age and id in ( 1, 2, 3 ) order by id"
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
        {{/data}}
        order by id"
    }).unwrap();
    assert_eq!(2, rst.len());
}
```

### License

Dysql is free software, and is released under the terms of the Apache License version 2. See [LICENSE](LICENSE).