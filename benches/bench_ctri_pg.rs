
use dysql::{fetch_all, Value};
use criterion::{Criterion, BenchmarkId, criterion_group, criterion_main};

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

// Here we have an async function to benchmark
async fn do_fetch_all_dysql_sqlx(db: &std::cell::RefCell<sqlx::PgConnection>, dto: &Value<&str>) {
    let mut db = db.borrow_mut();
    let _rst = fetch_all!(|db, &dto| -> User {
        r#"SELECT * FROM test_user WHERE name = :value"#
    }).unwrap();
}

fn fetch_all_dysql_sqlx(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let db = std::cell::RefCell::new(runtime.block_on(sqlx_pg_db()));
    let dto = Value::new("a5");
    c.bench_with_input(
        BenchmarkId::new("dysql sqlx-pg", "fetch_all"),
        &(&db, &dto),
        move |b, param_ref| {
            let &(db, dto) = param_ref;
            b.to_async(&runtime).iter(|| 
                do_fetch_all_dysql_sqlx(db, dto)
            );
        },
    );
}

async fn do_fetch_all_raw_sqlx(db: &std::cell::RefCell<sqlx::PgConnection>, dto: &Value<&str>) {
    let sql ="select * from test_user where name = $1 ";
    let mut db = db.borrow_mut();
    let query = sqlx::query_as::<_, User>(&sql);
    let query = query.bind(&dto.value);
    let _rst = query.fetch_all(&mut *db).await.unwrap();
}
 
fn fetch_all_raw_sqlx(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let db = std::cell::RefCell::new(runtime.block_on(sqlx_pg_db()));
    let dto = Value::new("a5");
    c.bench_with_input(
        BenchmarkId::new("raw sqlx-pg", "fetch_all"),
        &(&db, &dto),
        move |b, param_ref| {
            let &(db, dto) = param_ref;
            b.to_async(&runtime).iter(|| 
                do_fetch_all_raw_sqlx(db, dto)
            );
        },
    );
}


async fn do_fetch_all_diesel(db: &std::cell::RefCell::<diesel::PgConnection>, dto: &Value<&str>) {
    use self::schema::test_user::dsl::*;
    use diesel::{QueryDsl, ExpressionMethods, RunQueryDsl};
    let mut db = db.borrow_mut();

    let _: Vec<User> = test_user
            .filter(name.eq(dto.value))
            .load(&mut *db)
            .unwrap();
}
 
fn fetch_all_diesel(c: &mut Criterion) {
    let db = std::cell::RefCell::new(diesel_pg_db());
    let dto = Value::new("a5");
    c.bench_with_input(
        BenchmarkId::new("diesel", "fetch_all"),
        &(&db, &dto),
        move |b, param_ref| {
            let &(db, dto) = param_ref;
            b.iter(|| 
                do_fetch_all_diesel(&db, dto)
            );
        },
    );
}

criterion_group!(
    benches,
    fetch_all_dysql_sqlx,
    fetch_all_raw_sqlx,
    fetch_all_diesel
);
criterion_main!(benches);

