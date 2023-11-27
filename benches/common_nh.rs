#![allow(unused)]
use futures::future::BoxFuture;

pub fn block_on<T>(task: T) -> T::Output
where
    T: std::future::Future + Send + 'static,
    T::Output: Send + 'static,
{
    tokio::task::block_in_place(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("tokio block_on fail")
            .block_on(task)
    })
}

pub trait QPS {
    fn qps(&self, total: u64);
    fn time(&self, total: u64);
    fn cost(&self);
}

impl QPS for std::time::Instant {
    fn qps(&self, total: u64) {
        let time = self.elapsed();
        println!(
            "use QPS: {} QPS/s",
            (total as u128 * 1000000000 as u128 / time.as_nanos() as u128)
        );
    }

    fn time(&self, total: u64) {
        let time = self.elapsed();
        println!(
            "use Time: {:?} ,each:{} ns/op",
            &time,
            time.as_nanos() / (total as u128)
        );
    }

    fn cost(&self) {
        let time = self.elapsed();
        println!("cost:{:?}", time);
    }
}

#[macro_export]
macro_rules! rbench {
    ($total:expr,$body:block) => {{
        let now = std::time::Instant::now();
        for _ in 0..$total {
            $body;
        }
        now.time($total);
        now.qps($total);
    }};
}


//// rbatis mock suite

#[derive(Debug, Clone)]
struct MockDriver {}

impl rbatis::rbdc::db::Driver for MockDriver {
    fn name(&self) -> &str {
        "sqlite"
    }

    fn connect(&self, url: &str) -> BoxFuture<Result<Box<dyn rbatis::rbdc::db::Connection>, rbatis::Error>> {
        Box::pin(async { Ok(Box::new(MockConnection {}) as Box<dyn rbatis::rbdc::db::Connection>) })
    }

    fn connect_opt<'a>(
        &'a self,
        opt: &'a dyn rbatis::rbdc::db::ConnectOptions,
    ) -> BoxFuture<Result<Box<dyn rbatis::rbdc::db::Connection>, rbatis::Error>> {
        Box::pin(async { Ok(Box::new(MockConnection {}) as Box<dyn rbatis::rbdc::db::Connection>) })
    }

    fn default_option(&self) -> Box<dyn rbatis::rbdc::db::ConnectOptions> {
        Box::new(MockConnectOptions {})
    }
}

#[derive(Clone, Debug)]
struct MockConnection {}

impl rbatis::rbdc::db::Connection for MockConnection {
    fn get_rows(
        &mut self,
        sql: &str,
        params: Vec<rbs::Value>,
    ) -> BoxFuture<Result<Vec<Box<dyn rbatis::rbdc::db::Row>>, rbatis::Error>> {
        let _ = params;
        Box::pin(async { Ok(vec![]) })
    }

    fn exec(&mut self, sql: &str, params: Vec<rbs::Value>) -> BoxFuture<Result<rbatis::rbdc::db::ExecResult, rbatis::Error>> {
        Box::pin(async {
            Ok(rbatis::rbdc::db::ExecResult {
                rows_affected: 0,
                last_insert_id: rbs::Value::Null,
            })
        })
    }

    fn close(&mut self) -> BoxFuture<Result<(), rbatis::Error>> {
        Box::pin(async { Ok(()) })
    }

    fn ping(&mut self) -> BoxFuture<Result<(), rbatis::Error>> {
        Box::pin(async { Ok(()) })
    }
}

#[derive(Clone, Debug)]
struct MockConnectOptions {}

impl rbatis::rbdc::db::ConnectOptions for MockConnectOptions {
    fn connect(&self) -> BoxFuture<Result<Box<dyn rbatis::rbdc::db::Connection>, rbatis::Error>> {
        Box::pin(async { Ok(Box::new(MockConnection {}) as Box<dyn rbatis::rbdc::db::Connection>) })
    }

    fn set_uri(&mut self, uri: &str) -> Result<(), rbatis::Error> {
        Ok(())
    }
}