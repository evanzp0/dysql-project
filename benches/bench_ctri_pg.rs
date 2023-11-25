use dysql::{Content, fetch_all, Value};
use criterion::{Criterion, BenchmarkId, criterion_group, criterion_main};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use sqlx::FromRow;

#[derive(Content, Clone)]
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

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
#[derive(FromRow)]
pub struct User {
    pub id: i64,
    pub name: Option<String>,
    pub age: Option<i32>,
}

async fn init_connection() -> Pool<Postgres> {
    let conn = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://root:111111@127.0.0.1/my_database").await.unwrap();
    conn
}

// Here we have an async function to benchmark
async fn do_describe_sqlx(db: &Pool<Postgres>, dto: &Value<i32>) {
    let _rst = fetch_all!(|db, &dto| -> User {
        r#"SELECT * FROM test_user WHERE 1 = 1 AND age > :value"#
    }).unwrap();
}

fn describe_sqlx(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let db = runtime.block_on(init_connection());
    let dto = Value::new(1);
    c.bench_with_input(
        BenchmarkId::new("fetch_all", "age > 1"),
        &(&db, &dto),
        move |b, param_ref| {
            let &(db, dto) = param_ref;
            b.to_async(&runtime).iter(|| 
                do_describe_sqlx(db, dto)
            );
        },
    );
}

criterion_group!(
    benches,
    describe_sqlx,
);
criterion_main!(benches);

