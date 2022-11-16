#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use dysql::PageDto;
use dysql_macro::page;
use ramhorns::Content;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::{connect, NoTls};
fn main() {
    let body = async {
        let conn = connect_db().await;
        let dto = UserDto {
            id: None,
            name: None,
            age: Some(13),
        };
        let pg_dto = PageDto::new(3, 10, &dto);
        let rst = 'rst_block: {
            let mut param_values: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
            let sql_tpl = ramhorns::Template::new(
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
            let sql_rendered = sql_tpl.render(&pg_dto);
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
            let params = param_values.clone().into_iter();
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
            let mut _tmp_pg_dto = pg_dto.clone();
            _tmp_pg_dto.init(count as u64);
            let sql_tpl = ramhorns::Template::new(
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
            let sql_rendered = sql_tpl.render(&_tmp_pg_dto);
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
            let pg_data = dysql::Pagination::from_dto(&_tmp_pg_dto, rst);
            Ok(pg_data)
        }
            .unwrap();
        match (&7, &rst.total) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
struct UserDto {
    id: Option<i64>,
    name: Option<String>,
    age: Option<i32>,
}
impl ::ramhorns::Content for UserDto {
    #[inline]
    fn capacity_hint(&self, tpl: &::ramhorns::Template) -> usize {
        tpl.capacity_hint() + self.id.capacity_hint(tpl) + self.name.capacity_hint(tpl)
            + self.age.capacity_hint(tpl)
    }
    #[inline]
    fn render_section<C, E>(
        &self,
        section: ::ramhorns::Section<C>,
        encoder: &mut E,
    ) -> std::result::Result<(), E::Error>
    where
        C: ::ramhorns::traits::ContentSequence,
        E: ::ramhorns::encoding::Encoder,
    {
        section.with(self).render(encoder)
    }
    #[inline]
    fn render_field_escaped<E>(
        &self,
        hash: u64,
        name: &str,
        encoder: &mut E,
    ) -> std::result::Result<bool, E::Error>
    where
        E: ::ramhorns::encoding::Encoder,
    {
        match hash {
            3133790766199404813u64 => self.id.render_escaped(encoder).map(|_| true),
            12661497617682247323u64 => self.name.render_escaped(encoder).map(|_| true),
            16357823180428290041u64 => self.age.render_escaped(encoder).map(|_| true),
            _ => Ok(false),
        }
    }
    #[inline]
    fn render_field_unescaped<E>(
        &self,
        hash: u64,
        name: &str,
        encoder: &mut E,
    ) -> std::result::Result<bool, E::Error>
    where
        E: ::ramhorns::encoding::Encoder,
    {
        match hash {
            3133790766199404813u64 => self.id.render_unescaped(encoder).map(|_| true),
            12661497617682247323u64 => self.name.render_unescaped(encoder).map(|_| true),
            16357823180428290041u64 => self.age.render_unescaped(encoder).map(|_| true),
            _ => Ok(false),
        }
    }
    fn render_field_section<P, E>(
        &self,
        hash: u64,
        name: &str,
        section: ::ramhorns::Section<P>,
        encoder: &mut E,
    ) -> std::result::Result<bool, E::Error>
    where
        P: ::ramhorns::traits::ContentSequence,
        E: ::ramhorns::encoding::Encoder,
    {
        match hash {
            3133790766199404813u64 => {
                self.id.render_section(section, encoder).map(|_| true)
            }
            12661497617682247323u64 => {
                self.name.render_section(section, encoder).map(|_| true)
            }
            16357823180428290041u64 => {
                self.age.render_section(section, encoder).map(|_| true)
            }
            _ => Ok(false),
        }
    }
    fn render_field_inverse<P, E>(
        &self,
        hash: u64,
        name: &str,
        section: ::ramhorns::Section<P>,
        encoder: &mut E,
    ) -> std::result::Result<bool, E::Error>
    where
        P: ::ramhorns::traits::ContentSequence,
        E: ::ramhorns::encoding::Encoder,
    {
        match hash {
            3133790766199404813u64 => {
                self.id.render_inverse(section, encoder).map(|_| true)
            }
            12661497617682247323u64 => {
                self.name.render_inverse(section, encoder).map(|_| true)
            }
            16357823180428290041u64 => {
                self.age.render_inverse(section, encoder).map(|_| true)
            }
            _ => Ok(false),
        }
    }
}
#[allow(dead_code)]
#[pg_mapper(table = "test_user")]
struct User {
    id: i64,
    name: Option<String>,
    age: Option<i32>,
}
impl tokio_pg_mapper::FromTokioPostgresRow for User {
    fn from_row(
        row: tokio_postgres::row::Row,
    ) -> ::std::result::Result<Self, tokio_pg_mapper::Error> {
        Ok(Self {
            id: row.try_get::<&str, i64>("id")?,
            name: row.try_get::<&str, Option<String>>("name")?,
            age: row.try_get::<&str, Option<i32>>("age")?,
        })
    }
    fn from_row_ref(
        row: &tokio_postgres::row::Row,
    ) -> ::std::result::Result<Self, tokio_pg_mapper::Error> {
        Ok(Self {
            id: row.try_get::<&str, i64>(&"id")?,
            name: row.try_get::<&str, Option<String>>(&"name")?,
            age: row.try_get::<&str, Option<i32>>(&"age")?,
        })
    }
    fn sql_table() -> String {
        "test_user".to_string()
    }
    fn sql_table_fields() -> String {
        " test_user.id ,  test_user.name ,  test_user.age ".to_string()
    }
    fn sql_fields() -> String {
        " id ,  name ,  age ".to_string()
    }
}
#[automatically_derived]
#[allow(dead_code)]
impl ::core::fmt::Debug for User {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "User",
            "id",
            &&self.id,
            "name",
            &&self.name,
            "age",
            &&self.age,
        )
    }
}
#[allow(dead_code)]
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for User {}
#[automatically_derived]
#[allow(dead_code)]
impl ::core::cmp::PartialEq for User {
    #[inline]
    fn eq(&self, other: &User) -> bool {
        self.id == other.id && self.name == other.name && self.age == other.age
    }
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
            {
                ::std::io::_eprint(
                    ::core::fmt::Arguments::new_v1(
                        &["connection error: ", "\n"],
                        &[::core::fmt::ArgumentV1::new_display(&e)],
                    ),
                );
            };
        }
    });
    client
}