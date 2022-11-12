//! Do dynamic-sql query through proc-macro
//! 
//! It bases on [**tokio-postgres**] and [**sqlx**] crate (default feature), you can switch them by setting the features. 
//! It uses [**Ramhorns**] the high performance template engine implementation of [**Mustache**]
//! 
//! ## Example (Sqlx)
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
//!     let page_dto = ...;
//!     let pagination = page!(|&mut page_dto, &conn| -> User).unwrap();
//! }
//! ```
//! 
//! ## Example (tokio-postgres)
//! Full example please see: [Dysql tokio-postgres example](https://github.com/evanzp0/dysql-project/tree/main/examples/with_tokio_postgres)
//! 
//! ## Example (sqlx)
//! Full example please see: [Dysql sqlx example](https://github.com/evanzp0/dysql-project/tree/main/examples/with_sqlx)

#[cfg(not(feature = "tokio-postgres"))]
mod dy_sqlx;
#[cfg(not(feature = "tokio-postgres"))]
use dy_sqlx::expand;

#[cfg(feature = "tokio-postgres")]
mod dy_tokio_postgres;
#[cfg(feature = "tokio-postgres")]
use dy_tokio_postgres::expand;

mod sql_expand;

use proc_macro::TokenStream;
use syn::{punctuated::Punctuated, parse_macro_input, Token};
use std::{collections::HashMap, sync::RwLock};
use quote::quote;

use dysql::{QueryType, get_dysql_config, STATIC_SQL_FRAGMENT_MAP, get_sql_fragment};

#[derive(Debug)]
struct SqlFragment {
    name: String,
    value: String,
}

impl syn::parse::Parse for SqlFragment {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name= input.parse::<syn::LitStr>()?.value();
        input.parse::<syn::Token!(,)>()?;
        let value= input.parse::<syn::LitStr>()?.value();

        Ok(Self { name, value })
    }
} 

#[allow(dead_code)]
#[derive(Debug)]
struct SqlClosure {
    dto: Option<syn::Ident>,
    is_dto_ref: bool,
    cot: syn::Ident, // database connection or transaction
    is_cot_ref: bool,
    is_cot_ref_mut: bool,
    sql_name: Option<String>,
    ret_type: Option<syn::Path>, // return type
    dialect: syn::Ident,
    body: String,
}

impl syn::parse::Parse for SqlClosure {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // parse closure parameters

        let mut is_dto_ref = false;
        //// parse dto
        input.parse::<syn::Token!(|)>()?;
        if let Ok(_) = input.parse::<syn::Token!(&)>() {
            is_dto_ref = true;
            input.parse::<syn::Token!(mut)>().ok();
        };
        let dto = match input.parse::<syn::Ident>() {
            Ok(i) => Some(i),
            Err(e) => match input.parse::<syn::Token!(_)>() {
                Ok(_) => None,
                Err(_) => return Err(e),
            },
        };

        //// parse cot
        let mut is_cot_ref = false;
        let mut is_cot_ref_mut = false;
        input.parse::<syn::Token!(,)>()?;
        //////parse ref mut
        let cot: syn::Ident = match input.parse::<syn::Token!(&)>() {
            Ok(_) => {
                is_cot_ref = true;
                match input.parse::<syn::Token!(mut)>() {
                    Ok(_) => {
                        is_cot_ref = false;
                        is_cot_ref_mut = true;
                        input.parse()?
                    },
                    Err(_) => input.parse()?,
                }
            },
            Err(_) => {
                input.parse()?
            },
        };

        // parse sql_name
        let mut sql_name = None;
        match input.parse::<syn::Token!(|)>() {
            Ok(_) => (),
            Err(_) => {
                input.parse::<syn::Token!(,)>()?;
                sql_name = match input.parse::<syn::LitStr>() {
                    Ok(s) => Some(s.value()),
                    Err(_) => None,
                };
                input.parse::<syn::Token!(|)>()?;
            },
        }

        // parses token(->) first ,and then parses the tuple of return type and dialect
        let dialect: syn::Ident;
        let ret_type:Option<syn::Path>;
        match input.parse::<syn::Token!(->)>() {
            Ok(_) => {
                // try to parse return type path: ret_type, or ( ... )
                match input.parse::<syn::Path>() {
                    Ok(p) => {
                        ret_type = Some(p);
                        dialect = get_default_dialect(&input.span());
                    },
                    Err(_) => match parse_return_tuple(input) {
                        // try to parse return tuple : ( ret_type, dialect ) or ( ret_type, _ )
                        Ok(tp) => {
                            match tp.parse::<syn::Path>() {
                                Ok(p) => {
                                    ret_type = Some(p);
                                    tp.parse::<syn::Token!(,)>()?;
                                    if let Err(_) = tp.parse::<syn::Token!(_)>() {
                                        match tp.parse::<syn::Ident>() {
                                            Ok(i) => dialect = i,
                                            Err(_) => return Err(syn::Error::new(proc_macro2::Span::call_site(), "Need specify the dialect")),
                                        }
                                    } else {
                                        dialect = get_default_dialect(&input.span());
                                    }
                                },
                                // try to parse ( _, dialect )
                                Err(_) => match tp.parse::<syn::Token!(_)>() {
                                    Ok(_) => {
                                        ret_type = None;
                                        tp.parse::<syn::Token!(,)>()?;
                                        if let Err(_) = tp.parse::<syn::Token!(_)>() {
                                            match tp.parse::<syn::Ident>() {
                                                Ok(i) => dialect = i,
                                                Err(_) => return Err(syn::Error::new(proc_macro2::Span::call_site(), "Need specify the dialect")),
                                            }
                                        } else {
                                            dialect = get_default_dialect(&input.span());
                                        }
                                    },
                                    Err(_) => return Err(syn::Error::new(proc_macro2::Span::call_site(), "Need specify the return type")),
                                },
                            }
                        },
                        Err(_) => return Err(syn::Error::new(proc_macro2::Span::call_site(), "Need specify the return type and dialect")),
                    },
                }
            },
            Err(_) => {
                ret_type = None;
                dialect = get_default_dialect(&input.span());
            },
        };

        // parse closure sql body
        let body = parse_body(input)?;
        let body: Vec<String> = body.split('\n').into_iter().map(|f| f.trim().to_owned()).collect();
        let body = body.join(" ").to_owned();

        let sc = SqlClosure { dto, is_dto_ref, cot, is_cot_ref, is_cot_ref_mut, sql_name, ret_type, dialect, body };
        // eprintln!("{:#?}", sc);

        Ok(sc)
    }
}

fn parse_body(input: &syn::parse::ParseBuffer) -> Result<String, syn::Error> {
    let body_buf;
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

pub(crate) fn gen_path(s: &str) -> syn::Path {
    let seg = syn::PathSegment {
        ident: syn::Ident::new(s, proc_macro2::Span::call_site()),
        arguments: syn::PathArguments::None,
    };
    let mut punct: Punctuated<syn::PathSegment, syn::Token![::]> = Punctuated::new();
    punct.push_value(seg);
    let path = syn::Path{ leading_colon: None, segments: punct };

    path
}

fn parse_return_tuple(input: syn::parse::ParseStream) -> syn::Result<syn::parse::ParseBuffer> {
    let tuple_buf;
    syn::parenthesized!(tuple_buf in input);

    Ok(tuple_buf)
}

fn get_default_dialect(span: &proc_macro2::Span) -> syn::Ident {
    match get_dysql_config().dialect {
        dysql::SqlDialect::postgres => syn::Ident::new(&dysql::SqlDialect::postgres.to_string(), span.clone()),
        dysql::SqlDialect::mysql => syn::Ident::new(&dysql::SqlDialect::mysql.to_string(), span.clone()),
        dysql::SqlDialect::sqlite => syn::Ident::new(&dysql::SqlDialect::sqlite.to_string(), span.clone()),
    }
}

///
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
    let st = syn::parse_macro_input!(input as SqlClosure);
    if st.ret_type.is_none() { panic!("ret_type can't be null.") }

    match expand(&st, QueryType::FetchAll) {
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
#[proc_macro]
pub fn fetch_one(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as SqlClosure);
    if st.ret_type.is_none() { panic!("ret_type can't be null.") }

    match expand(&st, QueryType::FetchOne) {
        Ok(ret) => ret.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

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
#[proc_macro]
pub fn fetch_scalar(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as SqlClosure);
    if st.ret_type.is_none() { panic!("ret_type can't be null.") }

    match expand(&st, QueryType::FetchScalar) {
        Ok(ret) => ret.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

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
#[proc_macro]
pub fn execute(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as SqlClosure);

    match expand(&st, QueryType::Execute) {
        Ok(ret) => ret.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

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
#[proc_macro]
pub fn insert(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as SqlClosure);

    match expand(&st, QueryType::Insert) {
        Ok(ret) => ret.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

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
    let st = parse_macro_input!(input as SqlFragment);
    let cache = STATIC_SQL_FRAGMENT_MAP.get_or_init(|| {
        RwLock::new(HashMap::new())
    });

    cache.write().unwrap().insert(st.name, st.value.to_string());

    quote!().into()
}

///
/// page query
/// 
/// # Examples
///
/// Basic usage:
/// 
/// ```ignore
/// let conn = connect_db().await;
/// let dto = UserDto::new(None, None, Some(13));
/// let mut pg_dto = PageDto::new(3, 10, &dto);
/// 
/// let rst = page!(|&mut pg_dto, &conn| -> User {
///     "select * from test_user 
///     where 1 = 1
///         {{#data}}{{#name}}and name = :data.name{{/name}}{{/data}}
///         {{#data}}{{#age}}and age > :data.age{{/age}}{{/data}}
///     order by id"
/// }).unwrap();
/// 
/// assert_eq!(7, rst.total);
/// /// ```
#[proc_macro]
pub fn page(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as SqlClosure);
    if st.ret_type.is_none() { panic!("ret_type can't be null.") }

    match expand(&st, QueryType::Page) {
        Ok(ret) => ret.into(),
        Err(e) => e.into_compile_error().into(),
    }
}