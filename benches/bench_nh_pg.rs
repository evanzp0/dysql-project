mod common_nh;

use common_nh::*;

use dysql::{fetch_all, Value};
use rbatis::executor::Executor;

#[derive(Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)] // rbatis need it
#[derive(dysql::Content)] // dysql need it
pub struct UserDto {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub age: Option<i32>,
    pub id_rng: Option<Vec<i32>>,
}

impl UserDto {
    pub fn new(id: Option<i64>, name: Option<String>, age: Option<i32>, id_rng: Option<Vec<i32>>) -> Self {
        Self { id, name, age, id_rng }
    }
}

// this mod is for diesel
mod schema {
    diesel::table! {
        test_user {
            id -> BigInt,
            name -> Nullable<VarChar>,
            age -> Nullable<Integer>,
        }
    }
}

#[derive(Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)] // rbatis need it
#[derive(sqlx::FromRow)] // sqlx need it
#[derive(tokio_pg_mapper_derive::PostgresMapper, PartialEq)] // tokio-postgres need it
#[pg_mapper(table="test_user")] // tokio-postgres need it
#[derive(diesel::Queryable, diesel::Selectable)] // diesel need it
#[diesel(table_name = schema::test_user)] // diesel need it
pub struct User {
    pub id: i64,
    pub name: Option<String>,
    pub age: Option<i32>,
}

fn diesel_pg_db() -> diesel::PgConnection {
    use diesel::Connection;

    let db_url = "postgres://root:111111@localhost:5432/my_database";
    let conn = diesel::PgConnection::establish(db_url).unwrap();
    
    conn
}

async fn tokio_pg_db() -> tokio_postgres::Client {
    use tokio_postgres::{NoTls, connect};
    let (client, connection) = connect("host=127.0.0.1 user=root password=111111 dbname=my_database", NoTls).await.unwrap();
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    client
}

pub async fn sqlx_pg_db() -> sqlx::PgConnection {
    use sqlx::ConnectOptions;
    let options = sqlx::postgres::PgConnectOptions::new()
        .host("127.0.0.1")
        .port(5432)
        .username("root")
        .password("111111")
        .database("my_database");
    
    let conn = options.connect().await.unwrap();
    conn
}

async fn rbatis_pg_db() -> rbatis::executor::RBatisConnExecutor {
    let rb = rbatis::RBatis::new();
    rb.init(rbdc_pg::Driver{},"postgres://root:111111@localhost:5432/my_database").unwrap();
    rb.acquire().await.unwrap()
}

rbatis::pysql!(pysql_select(rb: &dyn Executor, name:&str)  -> Result<rbs::Value, rbatis::Error> =>
    r#"`select `
         ` * `
      `from test_user where name=#{name}`
"#);

// cargo test --release --bench bench_nh_pg --no-fail-fast -- --exact -Z unstable-options --show-output

#[test]
fn bench_raw_sqlx() {
    let f = async move {
        let mut conn = sqlx_pg_db().await;
        let name = "a5";
        let sql ="select * from test_user where name = $1 ";
        rbench!(100000, {
            let query = sqlx::query_as::<sqlx::Postgres, User>(&sql);
            let query = query.bind(&name);
            let _rst = query.fetch_all(&mut conn).await.unwrap();
        });
    };
    block_on(f);
}

#[test]
fn bench_dysql_sqlx() {
    let f = async move {
        let mut conn = sqlx_pg_db().await;
        let dto = Value::new("a5");
        rbench!(100000, {
            fetch_all!(|&mut conn, &dto| -> User {
                "select * from test_user where 1 = 1 and name = :value"
            }).unwrap();
        });
    };
    block_on(f);
}

#[test]
fn bench_raw_rbatis() {
    let f = async move {
        let conn = rbatis_pg_db().await;
        let name = "a5".to_owned();
        rbench!(100000, {
            pysql_select(&conn, &name).await.unwrap();
        });
    };
    block_on(f);
}

#[test]
fn bench_dysql_rbatis() {
    let f = async move {
        let conn = rbatis_pg_db().await;
        let dto = Value::new("a5");
        rbench!(100000, {
            dysql::fetch_all!(|&conn, &dto| -> User {
                "select * from test_user where name = :value"
            }).unwrap();
        });
    };
    block_on(f);
}

#[test]
fn bench_raw_tokio_pg() {
    use tokio_pg_mapper::FromTokioPostgresRow;

    let f = async move {
        let conn = tokio_pg_db().await;
        let sql ="select * from test_user where name = $1 ";
        let name = "a5";
        rbench!(100000, {
            let stmt = conn.prepare(&sql).await.unwrap();
            let rows = conn.query(&stmt, &[&name]).await.unwrap();

            let _ = rows
                .iter()
                .map(|row| <User>::from_row_ref(row).expect("query unexpected error"))
                .collect::<Vec<User>>();
        });
    };
    block_on(f);
}

#[test]
fn bench_dysql_tokio_pg() {
    let f = async move {
        let conn = tokio_pg_db().await;
        let dto = Value::new("a5");
        rbench!(100000, {
            dysql::fetch_all!(|&conn, &dto| -> User {
                "select * from test_user where name = :value"
            }).unwrap();
        });
    };
    block_on(f);
}

#[test]
fn bench_diesel() {
    use self::schema::test_user::dsl::*;
    use diesel::{QueryDsl, ExpressionMethods, RunQueryDsl};

    let mut db = diesel_pg_db();
    let f_name = "a";
    rbench!(100000, {
        let _: Vec<User> = test_user
            .filter(name.eq(f_name))
            .load(&mut db)
            .unwrap();
    });
}


/*
---- bench_diesel stdout ----
use Time: 25.90198887s ,each:259019 ns/op
use QPS: 3860 QPS/s

---- bench_raw_sqlx stdout ----
use Time: 28.776344373s ,each:287763 ns/op
use QPS: 3475 QPS/s

---- bench_dysql_sqlx stdout ----
use Time: 29.58345996s ,each:295834 ns/op
use QPS: 3380 QPS/s

---- bench_raw_rbatis stdout ----
use Time: 29.680996296s ,each:296809 ns/op
use QPS: 3369 QPS/s

---- bench_dysql_rbatis stdout ----
use Time: 29.920421087s ,each:299204 ns/op
use QPS: 3342 QPS/s

---- bench_dysql_tokio_pg stdout ----
use Time: 53.982295655s ,each:539822 ns/op
use QPS: 1852 QPS/s

---- bench_raw_tokio_pg stdout ----
use Time: 42.387503598s ,each:423875 ns/op
use QPS: 2359 QPS/s
*/