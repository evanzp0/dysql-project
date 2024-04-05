use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct UserDto {
    pub age: u8,
}

#[tokio::main]
async fn main() {
    use std::str::FromStr;
    use sqlx::ConnectOptions;

    let mut conn = sqlx::sqlite::SqliteConnectOptions::from_str("sqlite::memory:")
        .unwrap()
        .journal_mode( sqlx::sqlite::SqliteJournalMode::Wal)
        .read_only(false)
        .connect()
        .await.unwrap();

    // prepare test data
    sqlx::query("DROP TABLE IF EXISTS test_user").execute(&mut conn).await.unwrap();
    sqlx::query(r#"
        CREATE TABLE test_user (
            age INT NOT NULL
        )"#
    ).execute(&mut conn).await.unwrap();

    sqlx::query("INSERT INTO test_user (age) VALUES (?)")
        .bind(1 as u8)
        .execute(&mut conn)
        .await
        .unwrap();

    let rst = sqlx::query_as::<_, UserDto>("SELECT * FROM test_user WHERE age = 1")
        .bind(1 as u8)
        .fetch_one(&mut conn)
        .await
        .unwrap();

    println!("{:?}", rst);
}