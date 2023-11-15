use dysql::{Content, fetch_one, SqlxExecutorAdatper};
use sqlx::{FromRow, Postgres, Pool, postgres::PgPoolOptions};

#[tokio::main]
async fn main() {
    let conn = connect_postgres_db().await;
    let mut tran = conn.begin().await.unwrap();

    let dto = UserDto{ id: None, name: None, age: Some(13) , id_rng: None };


    let rst = fetch_one!(|&mut *tran, dto| -> User {
        r#"SELECT * FROM test_user 
        WHERE id = 1
        ORDER BY id"#
    });

    println!("{:?}", rst);


    // let rst = fetch_one!(|conn, dto| -> User {
    //     r#"SELECT * FROM test_user 
    //     WHERE 1 = 1
    //       {{#name}}AND name = :name{{/name}}
    //       {{#age}}AND age > :age{{/age}}
    //     ORDER BY id"#
    // });

    // let rst = {
    //     let sql_tpl = match dysql::get_sql_template(3245002281272997655u64) {
    //         Some(tpl) => tpl,
    //         None => {
    //             let serd_template = [
    //             ];
    //             dysql::put_sql_template(3245002281272997655u64, &serd_template)
    //                 .expect("Unexpected error when put_sql_template")
    //         }
    //     };
    //     let named_sql: String = sql_tpl.render(&dto);
    //     let _named_sql = dysql::SqlNodeLinkList::new(&named_sql).trim().to_string();

    //     let named_sql = "select * from test_user where id = 1";
    //     let query = conn.create_query(&named_sql, Some(dto));
    //     let rst: User = query.fetch_one(&conn).await.unwrap();
    //     // let query = tran.create_query(&named_sql, Some(dto));
    //     // let rst: User = query.fetch_one(&mut *tran).await.unwrap();
        
        
    //     println!("{:?}", rst);
    // };

    // let user: User = rst;
    // println!("{:?}", user)

}

#[derive(Content, Clone)]
struct UserDto {
    id: Option<i64>,
    name: Option<String>,
    age: Option<i32>,
    id_rng: Option<Vec<i32>>,
}

#[derive(Debug, PartialEq)]
#[derive(FromRow)]
struct User {
    id: i64,
    name: Option<String>,
    age: Option<i32>,
}

async fn connect_postgres_db() -> Pool<Postgres> {
    dotenv::dotenv().ok();

    let conn = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://root:111111@127.0.0.1/my_database").await.unwrap();
    // let conn: Pool<Postgres> = PgPool::connect("postgres://root:111111@127.0.0.1/my_database").await.unwrap();

    conn
}