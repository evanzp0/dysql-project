use dysql::{PageDto, Pagination};
use ramhorns::Content;
// use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::{connect, NoTls};

#[tokio::main]
async fn main() {
    let conn = connect_db().await;
    let dto = UserDto {
        id: None,
        name: None,
        age: Some(13),
    };
    
    let mut pg_dto = PageDto::new(3, 10, dto);
    let pg_dto = &mut pg_dto;

    let _rst = 'rst_block: {
            let mut param_values: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
            let _sql_tpl = ramhorns::Template::new(
                    "SELECT count(*) FROM (select * from test_user where 1 = 1 {{#data}} {{#name}}and name = :data.name{{/name}} {{#age}}and age > :data.age{{/age}} {{/data}} order by id) as _tmp",
                )
                .unwrap();
            let sql_tpl = match dysql::get_sql_template(
                "032cc106d973f5ef71dd96f4bcc9084b",
            ) {
                Some(tpl) => tpl,
                None => {
                    dysql::put_sql_template(
                            "032cc106d973f5ef71dd96f4bcc9084b",
                            "SELECT count(*) FROM (select * from test_user where 1 = 1 {{#data}} {{#name}}and name = :data.name{{/name}} {{#age}}and age > :data.age{{/age}} {{/data}} order by id) as _tmp",
                        )
                        .expect("Unexpected error when put_sql_template")
                }
            };
            let sql_rendered = sql_tpl.render(pg_dto);
            let extract_rst = dysql::extract_params(
                &sql_rendered,
                dysql::SqlDialect::from("postgres".to_owned()),
            );
            if let Err(e) = extract_rst {
                break 'rst_block Err(
                    dysql::DySqlError(
                        dysql::ErrorInner::new(
                            dysql::Kind::ExtractSqlParamterError,
                            Some(Box::new(e)),
                        ),
                    ),
                );
            }
            let (sql, param_names) = extract_rst.unwrap();
            for i in 0..param_names.len() {
                if param_names[i] == "data.name" {
                    param_values.push(&pg_dto.data.name);
                }
                if param_names[i] == "data.age" {
                    param_values.push(&pg_dto.data.age);
                }
            }
            let stmt = conn.prepare(&sql).await;
            if let Err(e) = stmt {
                break 'rst_block Err(
                    dysql::DySqlError(
                        dysql::ErrorInner::new(
                            dysql::Kind::PrepareStamentError,
                            Some(Box::new(e)),
                        ),
                    ),
                );
            }
            let stmt = stmt.expect("Unexpected error");
            let params = param_values.into_iter();
            let params = params.as_slice();
            let row = conn.query_one(&stmt, &params).await;
            if let Err(e) = row {
                break 'rst_block Err(
                    dysql::DySqlError(
                        dysql::ErrorInner::new(
                            dysql::Kind::QueryError,
                            Some(Box::new(e)),
                        ),
                    ),
                );
            }
            let row = row.expect("Unexpected error");
            let count: i64 = row.get(0);

            {
                let _tmp_pg_dto: &mut PageDto<_> = pg_dto;
                _tmp_pg_dto.init(count as u64);
            }

            let _sql_tpl = ramhorns::Template::new(
                    "select * from test_user where 1 = 1 {{#data}} {{#name}}and name = :data.name{{/name}} {{#age}}and age > :data.age{{/age}} {{/data}} order by id limit {{page_size}} offset {{start}}",
                )
                .unwrap();
            let sql_tpl = match dysql::get_sql_template(
                "05bf2c9b5caae72076e5720168e2ca7b",
            ) {
                Some(tpl) => tpl,
                None => {
                    dysql::put_sql_template(
                            "05bf2c9b5caae72076e5720168e2ca7b",
                            "select * from test_user where 1 = 1 {{#data}} {{#name}}and name = :data.name{{/name}} {{#age}}and age > :data.age{{/age}} {{/data}} order by id limit {{page_size}} offset {{start}}",
                        )
                        .expect("Unexpected error when put_sql_template")
                }
            };
            let sql_rendered = sql_tpl.render(pg_dto);
            let extract_rst = dysql::extract_params(
                &sql_rendered,
                dysql::SqlDialect::from("postgres".to_owned()),
            );
            if let Err(e) = extract_rst {
                break 'rst_block Err(
                    dysql::DySqlError(
                        dysql::ErrorInner::new(
                            dysql::Kind::ExtractSqlParamterError,
                            Some(Box::new(e)),
                        ),
                    ),
                );
            }
            let (sql, _param_names) = extract_rst.unwrap();
            let stmt = conn.prepare(&sql).await;
            if let Err(e) = stmt {
                break 'rst_block Err(
                    dysql::DySqlError(
                        dysql::ErrorInner::new(
                            dysql::Kind::PrepareStamentError,
                            Some(Box::new(e)),
                        ),
                    ),
                );
            }
            let stmt = stmt.expect("Unexpected error");
            let mut param_values: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();

            for i in 0..param_names.len() {
                if param_names[i] == "data.name" {
                    param_values.push(&pg_dto.data.name);
                }
                if param_names[i] == "data.age" {
                    param_values.push(&pg_dto.data.age);
                }
            }

            let params = param_values.into_iter();
            let params = params.as_slice();
            let rows = conn.query(&stmt, &params).await;
            if let Err(e) = rows {
                break 'rst_block Err(
                    dysql::DySqlError(
                        dysql::ErrorInner::new(
                            dysql::Kind::QueryError,
                            Some(Box::new(e)),
                        ),
                    ),
                );
            }
            let rows = rows.expect("Unexpected error");
            let rst = rows
                .iter()
                .map(|row| User::from_row_ref(row).expect("query unexpected error"))
                .collect::<Vec<User>>();
            let pg_data = dysql::Pagination::from_dto(pg_dto, rst);
            Ok(pg_data)
            
            // let data = vec![PageDto::new(0, 0, UserDto {id: Some(1), name :Some("".to_owned()),age:Some(1) })];
            // let pg_data = dysql::Pagination::from_dto(pg_dto, data);
            // Ok(pg_data)

        }.unwrap();
}

#[derive(Content)]
struct UserDto {
    id: Option<i64>,
    name: Option<String>,
    age: Option<i32>,
}

#[allow(dead_code)]
#[derive(PostgresMapper, Debug, PartialEq)]
#[pg_mapper(table = "test_user")]
struct User {
    id: i64,
    name: Option<String>,
    age: Option<i32>,
}

async fn connect_db() -> tokio_postgres::Client {
    let (client, connection) = connect(
        "host=127.0.0.1 user=root password=111111 dbname=my_database",
        NoTls,
    )
    .await
    .unwrap();
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client
}
