#![feature(proc_macro_span)]

//! Dysql 是一个轻量级的编译时生成 SQL 模板的库，它在运行时根据传入的 DTO 自动生成动态的 SQL 并设置数据参数，
//! 在底层 Dysql 使用 sqlx, tokio-postgres, rbac 等框架执行最终的 SQL。

mod sql_fragment;
mod sql_expand;

use proc_macro::TokenStream;
use sql_expand::SqlExpand;
use sql_fragment::{STATIC_SQL_FRAGMENT_MAP, SqlFragment};
use syn::{punctuated::Punctuated, parse_macro_input, Token};
use std::{collections::HashMap, sync::RwLock, path::PathBuf};
use quote::quote;

use sql_fragment::get_sql_fragment;

/// 用于解析 dysql 所有过程宏的语句
#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct DyClosure {
    dto_ref_kind: RefKind,
    is_executor_deref: bool,
    executor: syn::Ident,
    dto: Option<syn::Ident>,
    sql_name: Option<String>,
    ret_type: Option<syn::Path>, // return type
    body: String,
    source_file: PathBuf,
}

#[derive(Debug)]
enum RefKind {
    Immutable,
    Mutable,
    None
}

impl DyClosure {
    pub(crate) fn gen_executor_token(&self) -> proc_macro2::TokenStream {
        let mut rst = match self.dto_ref_kind {
            RefKind::Immutable => quote!(&),
            RefKind::Mutable => quote!(&mut),
            RefKind::None => quote!(),
        };

        if self.is_executor_deref {
            rst.extend(quote!(*))
        }

        let executor = &self.executor;
        rst.extend(quote!(#executor));

        rst.into()
    }
}

impl syn::parse::Parse for DyClosure {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // 加载 .env 文件中的环境变量，读取自动持久化 sql 文件的参数
        dotenv::dotenv().ok();

        // 测试是否 | 开始
        input.parse::<syn::Token!(|)>()?;

        // 解析 executor 的引用(可能为 &mut, &)
        let dto_ref_kind: RefKind;
        match input.parse::<syn::Token!(&)>() {
            Err(_) => dto_ref_kind = RefKind::None,
            Ok(_) => match input.parse::<syn::Token!(mut)>() {
                Err(_) => dto_ref_kind = RefKind::Immutable,
                Ok(_) => dto_ref_kind = RefKind::Mutable,
            }
        }

        // 解析 executor (可能为 *executor, executor)
        let is_executor_deref: bool;
        let executor: syn::Ident;
        match input.parse::<syn::Token!(*)>() {
            Ok(_) => is_executor_deref = true,
            Err(_) => is_executor_deref = false,
        }
        match input.parse::<syn::Ident>() {
            Err(e) => return Err(e),
            Ok(ex) => executor = ex,
        }

        // 测试是否 | 结束, 并解析 ',dto '(dto 可能为 _, dto)
        let sql_name: Option<String>;
        let dto: Option<syn::Ident>;
        match input.parse::<syn::Token!(|)>() {
            Ok(_) => {
                sql_name = None;
                dto = None;
            },
            Err(_) => match input.parse::<syn::Token!(,)>() {
                Err(e) =>  return Err(e),
                Ok(_) => {
                    // 解析 dto
                    match input.parse::<syn::Token!(_)>() {
                        Ok(_) => dto = None,
                        Err(_) => match input.parse::<syn::Ident>() {
                            Err(e) => return Err(e),
                            Ok(d) => dto = Some(d),
                        }
                    }

                    // 测试是否 | 结束，并解析 , 'sql_name |'
                    match input.parse::<syn::Token!(|)>() {
                        // | 结束
                        Ok(_) => sql_name = None,
                        // 解析是否为接下来是否为 " , sql_name |"
                        Err(_) => match input.parse::<syn::Token!(,)>() {
                            Err(e) => return Err(e),
                            // 解析 sql_name
                            Ok(_) => {
                                match input.parse::<syn::Token!(_)>() {
                                    Ok(_) => { sql_name = None },
                                    Err(_) => match input.parse::<syn::LitStr>() {
                                        Ok(s) => sql_name = Some(s.value()),
                                        Err(_) => return Err(syn::Error::new(proc_macro2::Span::call_site(), "need specify the sql_name")),
                                    }
                                }
                                // | 结束
                                input.parse::<syn::Token!(|)>()?; 
                            }
                        }
                    }
                }
            }
        }

        // 解析 -> 符号
        let ret_type:Option<syn::Path>;
        match input.parse::<syn::Token!(->)>() {
            // 解析 ret_type
            Ok(_) => match input.parse::<syn::Path>() {
                Ok(p) => ret_type = Some(p),
                Err(_) => 
                    return Err(syn::Error::new(proc_macro2::Span::call_site(), "Need specify the return type")),
            }
            Err(_) => ret_type = None,
        };

        // 解析 { sql body } 
        let body = parse_body(input)?;
        let body: Vec<String> = body.split('\n').into_iter().map(|f| f.trim().to_owned()).collect();
        let body = body.join(" ").to_owned();

        // 获取当前被解析的文件位置
        let span: proc_macro::Span = input.span().unwrap();
        let source_file = span.source_file().path();

        let dsf = DyClosure { dto_ref_kind, is_executor_deref, executor, dto, sql_name, ret_type, body, source_file };
        // eprintln!("{:#?}", dsf);

        Ok(dsf)
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
/// let rst = fetch_all!(|dto| -> User {
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
// #[proc_macro]
// pub fn fetch_all(input: TokenStream) -> TokenStream {
//     // 将 input 解析成 SqlClosure
//     let st = syn::parse_macro_input!(input as DyClosure);

//     // fetch_all 必须要指定单个 item 的返回值类型
//     if st.ret_type.is_none() { panic!("ret_type can't be null.") }

//     match FetchAll.expand(&st) {
//         Ok(ret) => ret.into(),
//         Err(e) => e.into_compile_error().into(),
//     }
// }

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
/// let rst = fetch_one!(|dto| -> User {
///     r#"select * from test_user 
///     where id = :id
///     order by id"#
/// }).unwrap();
/// 
/// assert_eq!(User { id: 2, name: Some("zhanglan".to_owned()), age: Some(21) }, rst);
/// ```
#[proc_macro]
pub fn fetch_one(input: TokenStream) -> TokenStream {
    // 将 input 解析成 SqlClosure
    let st = syn::parse_macro_input!(input as DyClosure);

    // fetch_one 必须要指定单个 item 的返回值类型
    if st.ret_type.is_none() { panic!("ret_type can't be null.") }

    match SqlExpand.fetch_one(&st) {
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
/// let rst = fetch_scalar!(|_| -> i64 {
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
    let st = parse_macro_input!(input as SqlFragment);
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