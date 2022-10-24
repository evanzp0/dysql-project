#[cfg(feature = "sqlx")]
mod dy_sqlx;
#[cfg(feature = "sqlx")]
use dy_sqlx::expand;

#[cfg(not(feature = "sqlx"))]
mod dy_tokie_postgres;
#[cfg(not(feature = "sqlx"))]
use dy_tokie_postgres::expand;

use dysql::{QueryType, get_dysql_config};
use proc_macro::TokenStream;

#[allow(dead_code)]
#[derive(Debug)]
struct SqlClosure {
    dto: syn::Ident,
    cot: syn::Ident, // database connection or transaction
    is_cot_ref: bool,
    is_cot_ref_mut: bool,
    ret_type: Option<syn::Path>, // return type
    dialect: syn::Ident,
    body: String,
}

impl syn::parse::Parse for SqlClosure {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // parse closure parameters

        //// parse dto
        input.parse::<syn::Token!(|)>()?;
        let dto = match input.parse::<syn::Ident>() {
            Ok(i) => i,
            Err(_) => match input.parse::<syn::Token!(_)>() {
                Ok(t) => syn::Ident::new( "_empty_dto", t.span),
                Err(e) => return Err(e),
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

        //// parse return type
        let mut ret_type = None;
        match input.parse::<syn::Token!(|)>() {
            Ok(_) => (),
            Err(_) => {
                input.parse::<syn::Token!(,)>()?;
                ret_type = match input.parse::<syn::Path>() {
                    Ok(p) => Some(p),
                    Err(_) => None,
                };
                input.parse::<syn::Token!(|)>()?;
            },
        }

        // parse closure returning sql dialect
        let dialect: syn::Ident = match input.parse::<syn::Token!(->)>() {
            Ok(_) => input.parse()?,
            Err(_) => {
                match get_dysql_config().dialect {
                    dysql::SqlDialect::postgres => syn::Ident::new(&dysql::SqlDialect::postgres.to_string(), input.span()),
                    dysql::SqlDialect::mysql => syn::Ident::new(&dysql::SqlDialect::mysql.to_string(), input.span()),
                    dysql::SqlDialect::sqlite => syn::Ident::new(&dysql::SqlDialect::sqlite.to_string(), input.span()),
                }
            },
        };

        // parse closure sql body
        let body_buf;
        syn::braced!(body_buf in input);
        let body: syn::LitStr = body_buf.parse()?;
        let body = body.value();
        let body:Vec<_> = body.split("\n").map(|f| f.trim()).collect();
        let body = body.join(" ");
        let sc = SqlClosure { dto, cot, is_cot_ref, is_cot_ref_mut, ret_type, dialect, body };
        // eprintln!("{:#?}", sc);

        Ok(sc)
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
/// let rst = fetch_all!(|dto, conn, User| {
///     r#"select * from test_user 
///     where 1 = 1
///         {{#name}}and name = :name{{/name}}
///         {{#age}}and age > :age{{/age}}
///     order by id"#
/// });
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
    if st.ret_type.is_none() { panic!("Call macro fetch_all!(|..., ..., return_type|) error, return_type can't be null.") }

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
/// let rst = fetch_one!(|dto, conn, User| {
///     r#"select * from test_user 
///     where id = :id
///     order by id"#
/// });
/// 
/// assert_eq!(User { id: 2, name: Some("zhanglan".to_owned()), age: Some(21) }, rst);
/// ```
#[proc_macro]
pub fn fetch_one(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as SqlClosure);
    if st.ret_type.is_none() { panic!("Call macro fetch_all!(|..., ..., return_type|) error, return_type can't be null.") }

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
/// let rst = fetch_scalar!(|_, conn, i64| {
///     r#"select count (*) from test_user"#
/// });
/// assert_eq!(3, rst);
/// ```
#[proc_macro]
pub fn fetch_scalar(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as SqlClosure);
    if st.ret_type.is_none() { panic!("Call macro fetch_all!(|..., ..., return_type|) error, return_type can't be null.") }

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
/// let mut tran = get_transaction().await?;
/// 
/// let dto = UserDto::new(Some(2), None, None);
/// let rst = execute!(|dto, tran| {
///     r#"delete from test_user where id = :id"#
/// });
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
/// let mut tran = get_transaction().await?;
/// 
/// let dto = UserDto{ id: Some(4), name: Some("lisi".to_owned()), age: Some(50) };
/// let last_insert_id = insert!(|dto, tran| -> mysql {
///     r#"insert into test_user (id, name, age) values (4, 'aa', 1)"#  // works for mysql and sqlite
///     // r#"insert into test_user (id, name, age) values (4, 'aa', 1) returning id"#  // works for postgres
/// });
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
