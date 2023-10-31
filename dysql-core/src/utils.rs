use std::{sync::RwLock, collections::HashMap, hash::{Hash, Hasher}};

use fnv::FnvHasher;
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

pub fn hash_str(name: &str) -> u64 {
    let mut hasher = FnvHasher::default();
    name.hash(&mut hasher);
    hasher.finish()
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