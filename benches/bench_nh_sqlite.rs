mod entity;

use rbatis::{RBatis, executor::Executor};

use sqlx::{SqliteConnection, sqlite::{SqliteConnectOptions, SqliteJournalMode}};

mod common_nh;
use common_nh::*;

use crate::entity::test_user; // sea-orm need it

rbatis::pysql!(pysql_select(rb: &dyn Executor, name:&str)  -> Result<rbs::Value, rbatis::Error> =>
    r#"`select `
         ` * `
      `from test_user where name=#{name}`
"#);

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
#[derive(diesel::Queryable, diesel::Selectable)] // diesel need it
#[diesel(table_name = schema::test_user)] // diesel need it
// sea-orm need stuffs are in entity folder
pub struct User {
    pub id: i64,
    pub name: Option<String>,
    pub age: Option<i32>,
}

fn init_diesel_connection() -> diesel::SqliteConnection {
    use diesel::{Connection, RunQueryDsl, insert_into, ExpressionMethods};

    let db_url = "sqlite::memory:"; // "file:test.db"
    let mut conn = diesel::SqliteConnection::establish(db_url).unwrap();
    diesel::sql_query("DROP TABLE IF EXISTS test_user;")
    .execute(&mut conn)
    .unwrap();
    
    // create table
    diesel::sql_query(
        "CREATE TABLE test_user(\
            id INTEGER PRIMARY KEY AUTOINCREMENT,\
            name VARCHAR,\
            age INTEGER\
        );"
    )
    .execute(&mut conn)
    .unwrap();

    use schema::test_user::dsl::*;

    let _rst = insert_into(test_user)
        .values(&vec![
            (name.eq("a"), age.eq(10)),
            (name.eq("a"), age.eq(20)),
            (name.eq("a"), age.eq(20)),
            (name.eq("a"), age.eq(20)),
            (name.eq("a"), age.eq(20)),
            (name.eq("b"), age.eq(20)),
            (name.eq("b"), age.eq(20)),
            (name.eq("b"), age.eq(20)),
            (name.eq("b"), age.eq(20)),
        ])
        .execute(&mut conn)
        .unwrap();

    conn
}

async fn init_seaorm_connection() -> sea_orm::DatabaseConnection {
    use sea_orm::ConnectionTrait;
    use sea_orm::Set;
    use crate::entity::test_user::Entity as TestUser;
    use sea_orm::EntityTrait;

    let db = sea_orm::Database::connect("sqlite::memory:").await.unwrap();
    let builder = db.get_database_backend();
    let schema = sea_orm::Schema::new(builder);
    let table_create_statement = schema.create_table_from_entity(TestUser);
    let _ = db.execute(builder.build(&table_create_statement)).await;

    let _ = TestUser::insert_many(
        vec![
            test_user::ActiveModel { id: Set(1), name: Set(Some("a".to_owned())), age: Set(Some(10)) },
            test_user::ActiveModel { id: Set(2), name: Set(Some("a".to_owned())), age: Set(Some(10)) },
            test_user::ActiveModel { id: Set(3), name: Set(Some("a".to_owned())), age: Set(Some(10)) },
            test_user::ActiveModel { id: Set(4), name: Set(Some("a".to_owned())), age: Set(Some(10)) },
            test_user::ActiveModel { id: Set(5), name: Set(Some("a".to_owned())), age: Set(Some(10)) },
            test_user::ActiveModel { id: Set(6), name: Set(Some("b".to_owned())), age: Set(Some(10)) },
            test_user::ActiveModel { id: Set(7), name: Set(Some("b".to_owned())), age: Set(Some(10)) },
            test_user::ActiveModel { id: Set(8), name: Set(Some("b".to_owned())), age: Set(Some(10)) },
            test_user::ActiveModel { id: Set(9), name: Set(Some("b".to_owned())), age: Set(Some(10)) },
        ]
    )
    .exec(&db)
    .await
    .unwrap();

    db
}
async fn init_rbatis_connection() -> rbatis::executor::RBatisConnExecutor {
    let rb = RBatis::new();
    rb.init(rbdc_sqlite::Driver {},"sqlite::memory:").unwrap();

    rb.exec(r#"
        CREATE TABLE test_user (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name VARCHAR(255) NULL,
            age INT NULL
        )"#,
        vec![]
    ).await.unwrap();

    rb.exec("INSERT INTO test_user (name, age) VALUES ('a', 10)", vec![]).await.unwrap();
    rb.exec("INSERT INTO test_user (name, age) VALUES ('a', 21)", vec![]).await.unwrap();
    rb.exec("INSERT INTO test_user (name, age) VALUES ('a', 35)", vec![]).await.unwrap();
    rb.exec("INSERT INTO test_user (name, age) VALUES ('a', 12)", vec![]).await.unwrap();
    rb.exec("INSERT INTO test_user (name, age) VALUES ('a', 21)", vec![]).await.unwrap();
    rb.exec("INSERT INTO test_user (name, age) VALUES ('b', 22)", vec![]).await.unwrap();
    rb.exec("INSERT INTO test_user (name, age) VALUES ('b', 24)", vec![]).await.unwrap();
    rb.exec("INSERT INTO test_user (name, age) VALUES ('b', 31)", vec![]).await.unwrap();
    rb.exec("INSERT INTO test_user (name, age) VALUES ('b', 33)", vec![]).await.unwrap();

    rb.acquire().await.unwrap()
}

async fn init_sqlx_db() -> SqliteConnection {
    use std::str::FromStr;
    use sqlx::ConnectOptions;
    let mut conn = SqliteConnectOptions::from_str("sqlite::memory:")
        .unwrap()
        .journal_mode(SqliteJournalMode::Wal)
        .read_only(false)
        .connect()
        .await.unwrap();

    // prepare test data
    sqlx::query("DROP TABLE IF EXISTS test_user").execute(&mut conn).await.unwrap();
    sqlx::query(r#"
        CREATE TABLE test_user (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name VARCHAR(255) NULL,
            age INT NULL
        )"#
    ).execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('a', 10)").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('a', 21)").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('a', 35)").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('a', 12)").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('a', 21)").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('b', 22)").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('b', 24)").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('b', 31)").execute(&mut conn).await.unwrap();
    sqlx::query("INSERT INTO test_user (name, age) VALUES ('b', 33)").execute(&mut conn).await.unwrap();
    
    conn
}

//cargo test --release --package dysql --bench bench_nh_sqlite--no-fail-fast -- --exact -Z unstable-options --show-output
//cargo test --release --bench bench_nh_sqlite --no-fail-fast -- --exact -Z unstable-options --show-output

#[test]
fn bench_raw_sqlx() {
    let f = async move {
        let mut conn = init_sqlx_db().await;
            let name = "a";
            let sql ="select * from test_user where name = ? ";
        rbench!(100000, {
            let query = sqlx::query_as::<_, User>(&sql);
            let query = query.bind(&name);
            let _rst = query.fetch_all(&mut conn).await.unwrap();
        });
    };
    block_on(f);
}

#[test]
fn bench_dysql_sqlx() {
    let f = async move {
        let mut conn = init_sqlx_db().await;
        let dto = dysql::Value::new("a");
        rbench!(100000, {
            dysql::fetch_all!(|&mut conn, &dto| -> User {
                "select * from test_user where 1 = 1 and name = :value"
            }).unwrap();
        });
    };
    block_on(f);
}

#[test]
fn bench_raw_rbatis() {
    let f = async move {
        let rbatis = init_rbatis_connection().await;
        let name = "a".to_owned();
        rbench!(100000, {
            pysql_select(&rbatis, &name).await.unwrap();
        });
    };
    block_on(f);
}

#[test]
fn bench_dysql_rbatis() {
    let f = async move {
        let rbatis = init_rbatis_connection().await;
        let dto = dysql::Value::new("a");
        rbench!(100000, {
            dysql::fetch_all!(|&rbatis, &dto| -> User {
                "select * from test_user where name = :value"
            }).unwrap();
        });
    };
    block_on(f);
}

#[test]
fn bench_seaorm() {
    use crate::entity::test_user::Entity as TestUser;
    use sea_orm::{EntityTrait, Condition, QueryFilter, ColumnTrait};
    
    let f = async move {
        let db = init_seaorm_connection().await;
        let name = "a";
        rbench!(100000, {
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
    block_on(f);
}

#[test]
fn bench_diesel() {
    use self::schema::test_user::dsl::*;
    use diesel::{QueryDsl, ExpressionMethods, RunQueryDsl};

    let mut db = init_diesel_connection();
    let f_name = "a";
    rbench!(100000, {
        let _: Vec<User> = test_user
            .filter(name.eq(f_name))
            .load(&mut db)
            .unwrap();
    });
}

/*
---- bench_diesel(with cache) stdout ----
use Time: 795.485704ms ,each:7954 ns/op
use QPS: 125708 QPS/s

---- bench_diesel(without cache) stdout ----
use Time: 1.182472056s ,each:11824 ns/op
use QPS: 84568 QPS/s

---- bench_raw_sqlx stdout ----
use Time: 5.341557794s ,each:53415 ns/op
use QPS: 18721 QPS/s

---- bench_dysql_sqlx stdout ----
use Time: 5.346581006s ,each:53465 ns/op
use QPS: 18703 QPS/s

---- bench_dysql_rbatis stdout ----
use Time: 8.076815204s ,each:80768 ns/op
use QPS: 12381 QPS/s

---- bench_raw_rbatis stdout ----
use Time: 8.736199606s ,each:87361 ns/op
use QPS: 11446 QPS/s

---- bench_seaorm stdout ----
use Time: 11.151077418s ,each:111510 ns/op
use QPS: 8967 QPS/s
*/
