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

const LOOP_TIMES: u64 = 100_000;

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

// cargo test --release --bench bench_sync_async --no-fail-fast -- --exact -Z unstable-options --show-output --test-threads=1

async fn seaorm_pg_db() -> sea_orm::DatabaseConnection {
    let opt = sea_orm::ConnectOptions::new("postgres://root:111111@127.0.0.1:5432/my_database");

    let db = sea_orm::Database::connect(opt).await.unwrap();
    db
}

#[tokio::test]
async fn bench_raw_sqlx() {
    let f = async move {
        let mut conn = sqlx_pg_db().await;
        let name = "a5";
        let sql ="select * from test_user where name = $1 ";
        rbench!(LOOP_TIMES, {
            let query = sqlx::query_as::<sqlx::Postgres, User>(&sql);
            let query = query.bind(&name);
            let _rst = query.fetch_all(&mut conn).await.unwrap();
        });
    };
    f.await
}

#[tokio::test]
async fn bench_dysql_sqlx() {
    let f = async move {
        let mut conn = sqlx_pg_db().await;
        let dto = Value::new("a5");
        rbench!(LOOP_TIMES, {
            fetch_all!(|&mut conn, &dto| -> User {
                "select * from test_user where 1 = 1 and name = :value"
            }).unwrap();
        });
    };
    f.await
}

#[tokio::test]
async fn bench_raw_rbatis() {
    let f = async move {
        let conn = rbatis_pg_db().await;
        let name = "a5".to_owned();
        rbench!(LOOP_TIMES, {
            pysql_select(&conn, &name).await.unwrap();
        });
    };
    f.await
}

#[tokio::test]
async fn bench_dysql_rbatis() {
    let f = async move {
        let conn = rbatis_pg_db().await;
        let dto = Value::new("a5");
        rbench!(LOOP_TIMES, {
            dysql::fetch_all!(|&conn, &dto| -> User {
                "select * from test_user where name = :value"
            }).unwrap();
        });
    };
    f.await
}

#[tokio::test]
async fn bench_raw_tokio_pg() {
    use tokio_pg_mapper::FromTokioPostgresRow;

    let f = async move {
        let conn = tokio_pg_db().await;
        let sql ="select * from test_user where name = $1 ";
        let name = "a5";
        rbench!(LOOP_TIMES, {
            let stmt = conn.prepare(&sql).await.unwrap();
            let rows = conn.query(&stmt, &[&name]).await.unwrap();

            let _ = rows
                .iter()
                .map(|row| <User>::from_row_ref(row).expect("query unexpected error"))
                .collect::<Vec<User>>();
        });
    };
    f.await
}

#[tokio::test]
async fn bench_dysql_tokio_pg() {
    let f = async move {
        let conn = tokio_pg_db().await;
        let dto = Value::new("a5");
        rbench!(LOOP_TIMES, {
            dysql::fetch_all!(|&conn, &dto| -> User {
                "select * from test_user where name = :value"
            }).unwrap();
        });
    };
    f.await
}

#[tokio::test]
async fn bench_diesel() {
    use self::schema::test_user::dsl::*;
    use diesel::{QueryDsl, ExpressionMethods, RunQueryDsl};

    let mut db = diesel_pg_db();
    let f_name = "a";
    rbench!(LOOP_TIMES, {
        let _: Vec<User> = test_user
            .filter(name.eq(f_name))
            .load(&mut db)
            .unwrap();
    });
}

mod entity;
use crate::entity::test_user; // sea-orm need it

#[tokio::test]
async fn bench_seaorm() {
    use crate::entity::test_user::Entity as TestUser;
    use sea_orm::{EntityTrait, Condition, QueryFilter, ColumnTrait};
    
    let f = async move {
        let db = seaorm_pg_db().await;
        let name = "a";
        rbench!(LOOP_TIMES, {
            TestUser::find()
                .filter(
                    Condition::all()
                        .add(test_user::Column::Name.eq(name))
                )
                .all(&db)
                .await
                .unwrap();
        });
    };
    f.await
}

/* 
single thread test
---- bench_diesel stdout ----
use Time: 13.049055977s ,each:130490 ns/op
use QPS: 7663 QPS/s

---- bench_raw_sqlx stdout ----
use Time: 14.33185806s ,each:143318 ns/op
use QPS: 6977 QPS/s

---- bench_dysql_sqlx stdout ----
use Time: 14.929903056s ,each:149299 ns/op
use QPS: 6697 QPS/s

---- bench_raw_rbatis stdout ----
use Time: 15.329683855s ,each:153296 ns/op
use QPS: 6523 QPS/s

---- bench_dysql_rbatis stdout ----
use Time: 15.402828658s ,each:154028 ns/op
use QPS: 6492 QPS/s

---- bench_seaorm stdout ----
use Time: 28.690514513s ,each:286905 ns/op
use QPS: 3485 QPS/s

---- bench_dysql_tokio_pg stdout ----
use Time: 31.739262464s ,each:317392 ns/op
use QPS: 3150 QPS/s

---- bench_raw_tokio_pg stdout ----
use Time: 31.764859702s ,each:317648 ns/op
use QPS: 3148 QPS/s
*/


/*
mutiple thread test
---- bench_diesel stdout ----
use Time: 25.583256439s ,each:255832 ns/op
use QPS: 3908 QPS/s

---- bench_dysql_sqlx stdout ----
use Time: 26.284589602s ,each:262845 ns/op
use QPS: 3804 QPS/s

---- bench_raw_sqlx stdout ----
use Time: 27.101317399s ,each:271013 ns/op
use QPS: 3689 QPS/s

---- bench_raw_rbatis stdout ----
use Time: 27.896039216s ,each:278960 ns/op
use QPS: 3584 QPS/s

---- bench_dysql_rbatis stdout ----
use Time: 28.661677223s ,each:286616 ns/op
use QPS: 3488 QPS/s

---- bench_seaorm stdout ----
use Time: 31.053245511s ,each:310532 ns/op
use QPS: 3220 QPS/s

---- bench_raw_tokio_pg stdout ----
use Time: 38.61950466s ,each:386195 ns/op
use QPS: 2589 QPS/s

---- bench_dysql_tokio_pg stdout ----
use Time: 50.808218137s ,each:508082 ns/op
use QPS: 1968 QPS/s
*/