use std::{sync::RwLock, collections::HashMap};

use once_cell::sync::OnceCell;

/// 存放 sql!() 宏定义的 sql fragment
pub(crate) static STATIC_SQL_FRAGMENT_MAP: OnceCell<RwLock<HashMap<String, String>>> = OnceCell::new();

/// 获取 sql!() 宏定义的 sql fragment
pub(crate) fn get_sql_fragment(name: &str)-> Option<String> {
    let cache = STATIC_SQL_FRAGMENT_MAP.get().expect("Unexpect error: get_sql_fragment()");
    let fragment = cache.read().expect("Unexpect error: get_sql_fragment()");
    let fragment = fragment.get(name);
    let rst = match fragment {
        Some(v) => Some(v.to_owned()),
        None => None,
    };

    rst
}

/// 用于解析 sql!(sql_fragment_name, sql_fragment) 宏
/// 该宏用于定义公共的 sql 语句部分
#[derive(Debug)]
pub(crate) struct SqlMacroFragment {
    pub(crate) name: String,
    pub(crate) value: String,
}

impl syn::parse::Parse for SqlMacroFragment {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name= input.parse::<syn::LitStr>()?.value();
        input.parse::<syn::Token!(,)>()?;
        let value= input.parse::<syn::LitStr>()?.value();

        Ok(Self { name, value })
    }
} 