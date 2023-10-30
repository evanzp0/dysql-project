use std::{sync::RwLock, collections::HashMap};

use crypto::{md5::Md5, digest::Digest};
use once_cell::sync::OnceCell;

pub static STATIC_SQL_FRAGMENT_MAP: OnceCell<RwLock<HashMap<String, String>>> = OnceCell::new();

pub fn get_sql_fragment(name: &str)-> Option<String> {
    let cache = STATIC_SQL_FRAGMENT_MAP.get().expect("Unexpect error: get_sql_fragment()");
    let fragment = cache.read().expect("Unexpect error: get_sql_fragment()");
    let fragment = fragment.get(name);
    let rst = match fragment {
        Some(v) => Some(v.to_owned()),
        None => None,
    };

    rst
}

pub fn md5<S:Into<String>>(input: S) -> String {
    let mut md5 = Md5::new();
    md5.input_str(&input.into());
    md5.result_str()
}

#[derive(Debug)]
pub enum QueryType {
    FetchAll,
    FetchOne,
    FetchScalar,
    Execute,
    Insert,
    Page,
}