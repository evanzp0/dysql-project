# About Dysql

**Dysql** is a highly Performant Rust SQL Toolkit and ORM Library. An async, pure Rust SQL crate featuring compile-time Dynamic SQL.
是一个高性能轻量级的基于 Rust 的 异步的 orm 库，它提供编译时的动态 SQL 。

它基于  [**sqlx**](https://github.com/launchbadge/sqlx)、rbac、tokio-postgres 等底层上，提供了一个类似于 ibatis 的动态 sql 模版语言，来生成动态 SQL, 支持 CRUD 以及分页。

该模版语言的使用规则如下:
```(
dysql_macro_name!(| conn_or_tran, dto | [-> return_type ] { ...sql string... });
```

## 实例

### Cargo.toml:
```toml
[dependencies]
dysql = "2"
sqlx = { version = "0.7", features = [ "runtime-tokio-native-tls" , "postgres" ] }
tokio = { version = "1.0", features = ["full"] }
```

### main.rs
```rust

use std::error::Error;

use dysql::{PageDto, SortModel, sql, fetch_one, insert, fetch_scalar, execute, page, fetch_all, Value};
use sqlx::{Pool, MySql, mysql::MySqlPoolOptions};
use dysql::Content;
use sqlx::FromRow;

use crate::common::{UserDto, User};

#[derive(Content, Clone)]
pub struct UserDto {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub age: Option<i32>,
    pub id_rng: Option<Vec<i32>>,
}

#[allow(dead_code)]
impl UserDto {
    pub fn new(id: Option<i64>, name: Option<String>, age: Option<i32>, id_rng: Option<Vec<i32>>) -> Self {
        Self { id, name, age, id_rng }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
#[derive(FromRow)]
pub struct User {
    pub id: i64,
    pub name: Option<String>,
    pub age: Option<i32>,
}

pub async fn connect_mysql_db() -> Pool<MySql> {
    let conn = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:111111@127.0.0.1/my_database").await.unwrap();

    conn
}

#[tokio::main]
async fn main() {
    let conn = connect_mysql_db().await;

    let dto = UserDto{ id: None, name: None, age: Some(13) , id_rng: None };
    let rst = fetch_all!(|&conn, &dto| -> User {
        r#"SELECT * FROM test_user 
        WHERE 1 = 1
          {{#name}}AND name = :name{{/name}}
          {{#age}}AND age > :age{{/age}}
        ORDER BY id"#
    }).unwrap();
    assert_eq!(7, rst.len());

    let rst = fetch_all!(|&conn| -> User {
        r#"SELECT * FROM test_user"#
    }).unwrap();
    rst!("{}", rst.len());
}
```

### License

Dysql is free software, and is released under the terms of the Apache License version 2. See [LICENSE](LICENSE).