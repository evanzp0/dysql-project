use sqlx::{Pool, Any, any::AnyPoolOptions};


pub struct SqlxConncection
{
    pub inner: Pool<Any>,
}

impl SqlxConncection
{
    pub async fn create_conn(url: &str) -> Self {

        // let conn = PoolOptions::<Any>::new().connect(url).await.unwrap();
        let conn = AnyPoolOptions::new()
            .max_connections(5)
            .connect("postgres://root:111111@127.0.0.1/my_database").await.unwrap();
        // let a = Pool::<sqlx::Any>::connect(url).await.unwrap();
        Self {
            inner: conn
        }
    }
}
