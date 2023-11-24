// use dysql::{insert, Content};


// #[tokio::main]
// async fn main() {
//     let mut conn = connect_sqlite_db().await;
//     let mut tran = conn.acquire_begin().await.unwrap();

//     let dto = UserDto{ id: None, name: Some("lisi".to_owned()), age: Some(50), id_rng: None };

//     let insert_id = 'rst_block: {
//         #[cfg(feature = "tokio-postgres")]
//         use dysql::TokioPgExecutorAdatper;
//         #[cfg(feature = "sqlx")]
//         use dysql::SqlxExecutorAdatper;
//         #[cfg(feature = "rbatis")]
//         use dysql::RbatisExecutorAdatper;
//         let named_template = match dysql::get_sql_template(7621833242989289051u64) {
//             Some(tpl) => tpl,
//             None => {
//                 let serd_template = [
//                     1u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     54u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     105u8,
//                     110u8,
//                     115u8,
//                     101u8,
//                     114u8,
//                     116u8,
//                     32u8,
//                     105u8,
//                     110u8,
//                     116u8,
//                     111u8,
//                     32u8,
//                     116u8,
//                     101u8,
//                     115u8,
//                     116u8,
//                     95u8,
//                     117u8,
//                     115u8,
//                     101u8,
//                     114u8,
//                     32u8,
//                     40u8,
//                     110u8,
//                     97u8,
//                     109u8,
//                     101u8,
//                     44u8,
//                     32u8,
//                     97u8,
//                     103u8,
//                     101u8,
//                     41u8,
//                     32u8,
//                     118u8,
//                     97u8,
//                     108u8,
//                     117u8,
//                     101u8,
//                     115u8,
//                     32u8,
//                     40u8,
//                     58u8,
//                     110u8,
//                     97u8,
//                     109u8,
//                     101u8,
//                     44u8,
//                     32u8,
//                     58u8,
//                     97u8,
//                     103u8,
//                     101u8,
//                     41u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     7u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     54u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     54u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     0u8,
//                     105u8,
//                     110u8,
//                     115u8,
//                     101u8,
//                     114u8,
//                     116u8,
//                     32u8,
//                     105u8,
//                     110u8,
//                     116u8,
//                     111u8,
//                     32u8,
//                     116u8,
//                     101u8,
//                     115u8,
//                     116u8,
//                     95u8,
//                     117u8,
//                     115u8,
//                     101u8,
//                     114u8,
//                     32u8,
//                     40u8,
//                     110u8,
//                     97u8,
//                     109u8,
//                     101u8,
//                     44u8,
//                     32u8,
//                     97u8,
//                     103u8,
//                     101u8,
//                     41u8,
//                     32u8,
//                     118u8,
//                     97u8,
//                     108u8,
//                     117u8,
//                     101u8,
//                     115u8,
//                     32u8,
//                     40u8,
//                     58u8,
//                     110u8,
//                     97u8,
//                     109u8,
//                     101u8,
//                     44u8,
//                     32u8,
//                     58u8,
//                     97u8,
//                     103u8,
//                     101u8,
//                     41u8,
//                 ];
//                 dysql::put_sql_template(7621833242989289051u64, &serd_template)
//                     .expect("Unexpected error when put_sql_template")
//             }
//         };
//         let query = (&mut tran).create_query();
//         let insert_rst = query
//             .insert::<_, _, i32>(&mut tran, named_template, Some(dto))
//             .await;
//         let rst = match insert_rst {
//             Ok(Some(insert_id)) => Ok(insert_id),
//             Ok(None) => {
//                 let query = tran.create_query();
//                 query.fetch_insert_id(&mut tran).await
//             }
//             Err(e) => {
//                 break 'rst_block Err(
//                     dysql::DySqlError(
//                         dysql::ErrorInner::new(
//                             dysql::Kind::QueryError,
//                             Some(Box::new(e)),
//                             None,
//                         ),
//                     ),
//                 );
//             }
//         };
//         let rst = rst.map(|v| v as i32);
//         rst
//     }
//     .unwrap();

//     println!("{:#?}", insert_id);
//     tran.rollback().await.unwrap();
// }

// use rbatis::RBatis;
// use rbdc_sqlite::Driver;
// use serde::{Serialize, Deserialize};

// #[derive(Content, Clone, Debug, Serialize, Deserialize)]
// pub struct UserDto {
//     pub id: Option<i64>,
//     pub name: Option<String>,
//     pub age: Option<i32>,
//     pub id_rng: Option<Vec<i32>>,
// }

// impl UserDto {
//     pub fn new(id: Option<i64>, name: Option<String>, age: Option<i32>, id_rng: Option<Vec<i32>>) -> Self {
//         Self { id, name, age, id_rng }
//     }
// }

// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
// pub struct User {
//     pub id: i64,
//     pub name: Option<String>,
//     pub age: Option<i32>,
// }



// async fn connect_sqlite_db() -> RBatis {
//     let rb = RBatis::new();
//     rb.init(Driver{},"sqlite::memory:").unwrap();

//     rb.exec(r#"
//         CREATE TABLE test_user (
//             id INTEGER PRIMARY KEY AUTOINCREMENT,
//             name VARCHAR(255) NULL,
//             age INT NULL
//         )"#,
//         vec![]
//     ).await.unwrap();

//     rb.exec("INSERT INTO test_user (name, age) VALUES ('huanglan', 10)", vec![]).await.unwrap();
//     rb.exec("INSERT INTO test_user (name, age) VALUES ('zhanglan', 21)", vec![]).await.unwrap();
//     rb.exec("INSERT INTO test_user (name, age) VALUES ('zhangsan', 35)", vec![]).await.unwrap();
//     rb.exec("INSERT INTO test_user (name, age) VALUES ('a4', 12)", vec![]).await.unwrap();
//     rb.exec("INSERT INTO test_user (name, age) VALUES ('a5', 21)", vec![]).await.unwrap();
//     rb.exec("INSERT INTO test_user (name, age) VALUES ('a6', 22)", vec![]).await.unwrap();
//     rb.exec("INSERT INTO test_user (name, age) VALUES ('a7', 24)", vec![]).await.unwrap();
//     rb.exec("INSERT INTO test_user (name, age) VALUES ('a8', 31)", vec![]).await.unwrap();
//     rb.exec("INSERT INTO test_user (name, age) VALUES ('a9', 33)", vec![]).await.unwrap();

//     rb
// }

fn main() {}