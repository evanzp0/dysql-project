use std::future::Future;

use criterion::{Criterion, BenchmarkId, criterion_group, criterion_main};
use rbatis::{RBatis, executor::RBatisConnExecutor};
use rbdc_sqlite::Driver;

async fn init_connection() -> RBatisConnExecutor {
    let rb = RBatis::new();
    rb.init(Driver{},"sqlite::memory:").unwrap();

    rb.exec(r#"
        CREATE TABLE test_user (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name VARCHAR(255) NULL,
            age INT NULL
        )"#,
        vec![]
    ).await.unwrap();

    rb.exec("INSERT INTO test_user (name, age) VALUES ('huanglan', 10)", vec![]).await.unwrap();
    rb.exec("INSERT INTO test_user (name, age) VALUES ('zhanglan', 21)", vec![]).await.unwrap();
    rb.exec("INSERT INTO test_user (name, age) VALUES ('zhangsan', 35)", vec![]).await.unwrap();
    rb.exec("INSERT INTO test_user (name, age) VALUES ('a4', 12)", vec![]).await.unwrap();
    rb.exec("INSERT INTO test_user (name, age) VALUES ('a5', 21)", vec![]).await.unwrap();
    rb.exec("INSERT INTO test_user (name, age) VALUES ('a6', 22)", vec![]).await.unwrap();
    rb.exec("INSERT INTO test_user (name, age) VALUES ('a7', 24)", vec![]).await.unwrap();
    rb.exec("INSERT INTO test_user (name, age) VALUES ('a8', 31)", vec![]).await.unwrap();
    rb.exec("INSERT INTO test_user (name, age) VALUES ('a9', 33)", vec![]).await.unwrap();

    let a = rb.acquire().await.unwrap();
    
    a
}


fn describe_trivial(c: &mut Criterion) {
    // let runtime = tokio::runtime::Builder::new_multi_thread()
    //         .enable_all()
    //         .build()
    //         .expect("tokio block_on fail");
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let _db = runtime.block_on(init_connection());
    let size = 1;
    c.bench_with_input(
        BenchmarkId::new("select", "trivial"),
        &size,
        move |b, _db_ref| {
            // Insert a call to `to_async` to convert the bencher to async mode.
            // The timing loops are the same as with the normal bencher.
            b.to_async(&runtime).iter(|| 
                // do_describe_trivial(db_ref)
                async {}
            );
        },
    );
}

criterion_group!(
    benches,
    describe_trivial,
);
criterion_main!(benches);

pub fn block_on<T>(task: T) -> T::Output
where
    T: Future + Send + 'static,
    T::Output: Send + 'static,
{
    tokio::task::block_in_place(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("tokio block_on fail")
            .block_on(task)
        // tokio::runtime::Handle::current().block_on(task)
    })
}