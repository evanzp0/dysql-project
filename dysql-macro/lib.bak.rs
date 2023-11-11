#![feature(proc_macro_span)]

//! Do dynamic-sql query through proc-macro
//! 
//! It bases on [**sqlx**] crate (default feature), you can switch them by setting the features. 
//! It uses [**Ramhorns-ext**] the high performance template engine implementation of [**Mustache**]
//! 
//! ## Example
//! 
//! ### main.rs
//! ```ignore
//! //...
//! 
//! # #[tokio::main]
//! async fn main() {
//!     let conn = connect_postgres_db().await;
//!     
//!     // fetch all
//!     let dto = UserDto{ id: None, name: None, age: Some(15) };
//!     let rst = fetch_all!(|&dto, &conn| -> User {
//!         r#"SELECT * FROM test_user 
//!         WHERE 1 = 1
//!           {{#name}}AND name = :name{{/name}}
//!           {{#age}}AND age > :age{{/age}}
//!         ORDER BY id"#
//!     }).unwrap();
//!     assert_eq!(
//!         vec![
//!             User { id: 2, name: Some("zhanglan".to_owned()), age: Some(21) }, 
//!             User { id: 3, name: Some("zhangsan".to_owned()), age: Some(35) }
//!         ], 
//!         rst
//!     );
//! 
//!     let rst = fetch_one!(...).unwrap();
//! 
//!     let rst = fetch_scalar!(...).unwrap();
//!     
//!     let affected_rows_num = execute!(...).unwrap();
//!     
//!     let insert_id = insert!(...).unwrap();
//! 
//!     sql!('sql_fragment_1', "select * from table1");
//!     let rst = fetch_one!(|...| sql_fragment_1 + "where age > 10").unwrap();
//! 
//!     let mut page_dto = ...;
//!     let pagination = page!(|&mut page_dto, &conn| -> User).unwrap();
//! }
//! ```
//! 
//! ## Example
//! Full example please see: [Dysql sqlx example](https://github.com/evanzp0/dysql-project/tests)

mod sqlx_expend;
mod sql_macro_fragment;
mod sql_expand;

use dysql_core::SqlDialect;
use proc_macro::TokenStream;
use sql_expand::SqlExpand;
use sql_macro_fragment::{STATIC_SQL_FRAGMENT_MAP, SqlMacroFragment};
use syn::{punctuated::Punctuated, parse_macro_input, Token};
use std::{collections::HashMap, sync::RwLock, path::PathBuf};
use quote::quote;
use std::env;

use sqlx_expend::FetchAll;
use sql_macro_fragment::get_sql_fragment;

/// 用于解析 dysql 所有过程宏的语句
#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct DySqlFragment {
    dto: Option<syn::Ident>,
    sql_name: Option<String>,
    ret_type: Option<syn::Path>, // return type
    // dialect: syn::Ident,
    body: String,
    source_file: PathBuf,
    // executor_type: Option<syn::Path>,
}

impl syn::parse::Parse for DySqlFragment {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        dotenv::dotenv().ok();

        // parse closure parameters ---------------------
        input.parse::<syn::Token!(|)>()?;

        let dto: Option<syn::Ident>;
        // let executor_type: Option<syn::Path>;
        // let dialect: syn::Ident;

        // test if reached the end of `|`
        match input.parse::<syn::Token!(|)>() {
            Ok(_) => { dto = None }, // bound end
            Err(_) => {
                // parse dto
                match input.parse::<syn::Token!(_)>() {
                    Ok(_) => { dto = None },
                    Err(_) =>  match input.parse::<syn::Ident>() {
                        Err(e) => return Err(e),
                        Ok(d) => { dto = Some(d) },
                    },
                }

                match input.parse::<syn::Token!(|)>() {
                    Ok(_) => (), // bound end
                    Err(_) => match input.parse::<syn::Token!(,)>() {
                        Err(e) => return Err(e),
                        Ok(_) => {
                            // parse executor_type
                            match input.parse::<syn::Token!(_)>() {
                                Ok(_) => { executor_type = None },
                                Err(_) => match input.parse::<syn::Path>() {
                                    Err(e) => return Err(e),
                                    Ok(t) => { executor_type = Some(t) },
                                },
                            }

                            match input.parse::<syn::Token!(|)>() {
                                Ok(_) => { dialect = get_default_dialect(&input.span()) }, // bound end
                                Err(_) => match input.parse::<syn::Token!(,)>() {
                                    Err(e) => return Err(e),
                                    Ok(_) => {
                                        // parse dialect
                                        if let Err(_) = input.parse::<syn::Token!(_)>() {
                                            match input.parse::<syn::Ident>() {
                                                Ok(i) => dialect = i,
                                                Err(_) => return Err(syn::Error::new(proc_macro2::Span::call_site(), "Need specify the dialect")),
                                            }
                                        } else {
                                            dialect = get_default_dialect(&input.span())
                                        }

                                        input.parse::<syn::Token!(|)>()?; // bound end
                                    }
                                }
                            }
                        }, 
                    },
                    
                }
            },
        }

        // parses token(->) first ,and then parses the tuple of return type and sql_name
        let mut sql_name = None;
        let ret_type:Option<syn::Path>;
        match input.parse::<syn::Token!(->)>() {
            Ok(_) => {
                // try to parse return type path: ret_type, or ( ... )
                match input.parse::<syn::Path>() {
                    Ok(p) => {
                        ret_type = Some(p);
                    },
                    Err(_) => match parse_return_tuple(input) {
                        // try to parse return tuple : ( ret_type, sql_name )
                        Ok(tp) => {
                            // try to parse ( ret_type, ... )
                            match tp.parse::<syn::Path>() {
                                Ok(p) => {
                                    ret_type = Some(p);
                                },
                                // try to parse ( _, ... )
                                Err(_) => match tp.parse::<syn::Token!(_)>() {
                                    Ok(_) => {
                                        ret_type = None;
                                    },
                                    Err(_) => return Err(syn::Error::new(proc_macro2::Span::call_site(), "Need specify the return type")),
                                },
                            }

                            // try to parse ( ..., sql_name )
                            tp.parse::<syn::Token!(,)>()?;
                            if let Err(_) = tp.parse::<syn::Token!(_)>() {
                                match tp.parse::<syn::LitStr>() {
                                    Ok(s) => sql_name = Some(s.value()),
                                    Err(_) => return Err(syn::Error::new(proc_macro2::Span::call_site(), "Need specify the sql_name")),
                                }
                            }
                        },
                        Err(_) => return Err(syn::Error::new(proc_macro2::Span::call_site(), "Need specify the return type and dialect")),
                    },
                }
            },
            Err(_) => {
                ret_type = None;
            },
        };

        // parse closure sql body
        let body = parse_body(input)?;
        let body: Vec<String> = body.split('\n').into_iter().map(|f| f.trim().to_owned()).collect();
        let body = body.join(" ").to_owned();

        let span: proc_macro::Span = input.span().unwrap();
        let source_file = span.source_file().path();

        let sc = DySqlFragment { dto, sql_name, ret_type, dialect, body, source_file, executor_type };
        // eprintln!("{:#?}", sc);

        Ok(sc)
    }
}

/// 解析 sql body
fn parse_body(input: &syn::parse::ParseBuffer) -> Result<String, syn::Error> {
    let body_buf;
    // 解析大括号
    syn::braced!(body_buf in input);
    
    let ts = body_buf.cursor().token_stream().into_iter();
    let mut sql = String::new();
    for it in ts {
        match it {
            proc_macro2::TokenTree::Group(_) => {
                return Err(syn::Error::new(input.span(), "error not support group in sql".to_owned()));
            },
            proc_macro2::TokenTree::Ident(_) => {
                let v: syn::Ident = body_buf.parse()?;
                let sql_fragment = get_sql_fragment(&v.to_string());
                
                if let Some(s) = sql_fragment {
                    sql.push_str(&s);
                } else {
                    return Err(syn::Error::new(input.span(), "error not found sql identity".to_owned()));
                }
            },
            proc_macro2::TokenTree::Punct(v) => {
                if v.to_string() == "+" {
                    body_buf.parse::<Token!(+)>()?;
                } else {
                    return Err(syn::Error::new(input.span(), "error only support '+' expr".to_owned()));
                }
            },
            proc_macro2::TokenTree::Literal(_) => {
                let rst: syn::LitStr = body_buf.parse()?;
                
                sql.push_str(&rst.value());
            },
        };
    }

    Ok(sql)
}

/// 根据 s 生成 syn::Path 对象，用于 dysql 中有返回值的过程宏
pub(crate) fn gen_type_path(s: &str) -> syn::Path {
    let seg = syn::PathSegment {
        ident: syn::Ident::new(s, proc_macro2::Span::call_site()),
        arguments: syn::PathArguments::None,
    };
    let mut punct: Punctuated<syn::PathSegment, syn::Token![::]> = Punctuated::new();
    punct.push_value(seg);
    let path = syn::Path{ leading_colon: None, segments: punct };

    path
}

/// 用来解析 dysql closure 语句中的 return tuple : 
/// ( ret_type, sql_name ) or ( ret_type, _ ) or ( _, sql_name )
fn parse_return_tuple(input: syn::parse::ParseStream) -> syn::Result<syn::parse::ParseBuffer> {
    let tuple_buf;
    syn::parenthesized!(tuple_buf in input);

    Ok(tuple_buf)
}

/// 从环境变量 DYSQL_DEFAULT_DB_DIALECT 中获取默认的 SqlDialect
fn get_default_dialect(span: &proc_macro2::Span) -> syn::Ident {
    let default_dialect = if let Ok(v) = env::var("DYSQL_DEFAULT_DB_DIALECT") {
        SqlDialect::from(v)
    } else {
        SqlDialect::postgres
    };

    syn::Ident::new(&default_dialect.to_string(), span.clone())
}


/// fetch all datas that filtered by dto
/// 
/// # Examples
///
/// Basic usage:
/// 
/// ```ignore
/// let mut conn = connect_db().await;
/// 
/// let dto = UserDto {id: None, name: None, age: 13};
/// let rst = fetch_all!(|&dto, &conn| -> User {
///     r#"select * from test_user 
///     where 1 = 1
///         {{#name}}and name = :name{{/name}}
///         {{#age}}and age > :age{{/age}}
///     order by id"#
/// }).unwrap();
/// 
/// assert_eq!(
///     vec![
///         User { id: 2, name: Some("zhanglan".to_owned()), age: Some(21) }, 
///         User { id: 3, name: Some("zhangsan".to_owned()), age: Some(35) },
///     ], 
///     rst
/// );
/// ```
#[proc_macro]
pub fn fetch_all(input: TokenStream) -> TokenStream {
    // 将 input 解析成 SqlClosure
    let st = syn::parse_macro_input!(input as DySqlFragment);

    // fetch_all 必须要指定单个 item 的返回值类型
    if st.ret_type.is_none() { panic!("ret_type can't be null.") }

    match FetchAll.expand(&st) {
        Ok(ret) => ret.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

///
/// fetch one data that filtered by dto
/// 
/// # Examples
///
/// Basic usage:
/// 
/// ```ignore
/// let mut conn = connect_db().await;
/// 
/// let dto = UserDto {id: 2, name: None, age: None};
/// let rst = fetch_one!(|&dto, &conn| -> User {
///     r#"select * from test_user 
///     where id = :id
///     order by id"#
/// }).unwrap();
/// 
/// assert_eq!(User { id: 2, name: Some("zhanglan".to_owned()), age: Some(21) }, rst);
/// ```
// #[proc_macro]
// pub fn fetch_one(input: TokenStream) -> TokenStream {
//     let st = syn::parse_macro_input!(input as DySqlFragmentContext);
//     if st.ret_type.is_none() { panic!("ret_type can't be null.") }

//     match FetchOne.expand(&st) {
//         Ok(ret) => ret.into(),
//         Err(e) => e.into_compile_error().into(),
//     }
// }

///
/// Fetch a scalar value from query
/// 
/// # Examples
///
/// Basic usage:
/// 
/// ```ignore
/// let mut conn = connect_db().await;
/// 
/// let rst = fetch_scalar!(|_, &conn| -> i64 {
///     r#"select count (*) from test_user"#
/// }).unwrap();
/// assert_eq!(3, rst);
/// ```
// #[proc_macro]
// pub fn fetch_scalar(input: TokenStream) -> TokenStream {
//     let st = syn::parse_macro_input!(input as DySqlFragmentContext);
//     if st.ret_type.is_none() { panic!("ret_type can't be null.") }

//     match FetchScalar.expand(&st) {
//         Ok(ret) => ret.into(),
//         Err(e) => e.into_compile_error().into(),
//     }
// }

///
/// Execute query
/// 
/// # Examples
///
/// Basic usage:
/// 
/// ```ignore
/// let mut tran = get_transaction().await.unwrap();
/// 
/// let dto = UserDto::new(Some(2), None, None);
/// let rst = execute!(|&dto, &mut tran| {
///     r#"delete from test_user where id = :id"#
/// }).unwrap();
/// assert_eq!(1, rst);
/// 
/// tran.rollback().await?;
/// ```
// #[proc_macro]
// pub fn execute(input: TokenStream) -> TokenStream {
//     let st = syn::parse_macro_input!(input as DySqlFragmentContext);

//     match Execute.expand(&st) {
//         Ok(ret) => ret.into(),
//         Err(e) => e.into_compile_error().into(),
//     }
// }

///
/// Insert data
/// **Note:** if you use this macro under **postgres** database, you should add "returning id" at the end of sql statement by yourself.
/// 
/// # Examples
///
/// Basic usage:
/// 
/// ```ignore
/// let mut tran = get_transaction().await.unwrap();

/// let dto = UserDto{ id: Some(4), name: Some("lisi".to_owned()), age: Some(50) };
/// let last_insert_id = insert!(|&dto, &mut tran| -> (_, mysql) {
///     r#"insert into test_user (id, name, age) values (4, 'aa', 1)"#  // works for mysql and sqlite
///     // r#"insert into test_user (id, name, age) values (4, 'aa', 1) returning id"#  // works for postgres
/// }).unwrap();
/// assert_eq!(4, last_insert_id);
/// 
/// tran.rollback().await?;
/// ```
// #[proc_macro]
// pub fn insert(input: TokenStream) -> TokenStream {
//     let st = syn::parse_macro_input!(input as DySqlFragmentContext);

//     match Insert.expand(&st) {
//         Ok(ret) => ret.into(),
//         Err(e) => e.into_compile_error().into(),
//     }
// }

///
/// Define a global sql fragment
/// 
/// # Examples
///
/// Basic usage:
/// 
/// ```ignore
/// sql!("select_sql", "select * from table1 ")
/// 
/// let last_insert_id = fetch_all!(|&dto, &mut tran| {
///     select_sql + "where age > 10 "
/// }).unwrap();
/// 
/// tran.rollback().await?;
/// ```
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    let st = parse_macro_input!(input as SqlMacroFragment);
    let cache = STATIC_SQL_FRAGMENT_MAP.get_or_init(|| {
        RwLock::new(HashMap::new())
    });

    cache.write().unwrap().insert(st.name, st.value.to_string());

    quote!().into()
}

// page query
// 
// # Examples
//
// Basic usage:
// 
// ```ignore
// let conn = connect_db().await;
// let dto = UserDto::new(None, None, Some(13));
// let mut pg_dto = PageDto::new(3, 10, &dto);
// 
// let rst = page!(|&mut pg_dto, &conn| -> User {
//     "select * from test_user 
//     where 1 = 1
//         {{#data}}
//             {{#name}}and name = :data.name{{/name}}
//             {{#age}}and age > :data.age{{/age}}
//         {{/data}}
//     order by id"
// }).unwrap();
// 
// assert_eq!(7, rst.total);
// ```

// #[proc_macro]
// pub fn page(input: TokenStream) -> TokenStream {
//     let st = syn::parse_macro_input!(input as DySqlFragmentContext);
//     if st.ret_type.is_none() { panic!("ret_type can't be null.") }

//     match Page.expand(&st) {
//         Ok(ret) => ret.into(),
//         Err(e) => e.into_compile_error().into(),
//     }
// }