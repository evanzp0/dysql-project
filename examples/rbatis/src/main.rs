use rbatis::RBatis;
use rbdc_pg::PgDriver;
use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() {
    let rb = RBatis::new();
    rb.init(PgDriver{},"postgres://root:111111@localhost:5432/my_database").unwrap();
    let driver_type = rb.driver_type().unwrap();
    println!("{}", driver_type);
    let dto = UserDto{ id: Some(2), name: Some("ab".to_owned()), age: Some(13) , id_rng: None };
    let sql = "select * from test_user where name = $1";

    let rb_args = vec![rbs::to_value!(&dto.name)];

    let rst: Option<User> = rb
        .query_decode(sql, rb_args.clone())
        .await
        .unwrap();
    println!("{:#?}", rst);

    let mut tran = rb.acquire_begin().await.unwrap();
    let insert_sql = "insert into test_user (name, age) values ('ab', 1) returning id";

    let r = tran.exec(&insert_sql, vec![]).await.unwrap();
    println!("r = {:#?}", r);
    let sql = "select * from test_user order by id";
    let r = tran.query(&sql, vec![]).await.unwrap();
    let rst = rbatis::decode::decode::<Option<Vec<User>>>(r);

    println!("{:#?}", rst);
    tran.rollback().await.ok();
}


#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: Option<String>,
    pub age: Option<i32>,
}
