use std::{sync::{RwLock, Arc}, collections::HashMap};

use crypto::{md5::Md5, digest::Digest};
use once_cell::sync::OnceCell;
use ramhorns::{Template, Content};

use crate::{DySqlResult, Kind, DySqlError, ErrorInner};

pub static SQL_TEMPLATE_CACHE: OnceCell<RwLock<HashMap<String, Arc<Template>>>> = OnceCell::new();
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

#[allow(dead_code)]
fn get_sql_template_cache() -> &'static RwLock<HashMap<String, Arc<Template<'static>>>> {
    let cache = SQL_TEMPLATE_CACHE.get_or_init(|| {
        RwLock::new(HashMap::new())
    });

    cache
}

pub fn get_sql_template(template_id: &str) -> Option<Arc<Template<'static>>> {
    let cache = get_sql_template_cache();

    let template_stub = cache.read().unwrap();
    let template = template_stub.get(template_id);

    if let Some(tmpl) = template {
        // println!("get template from cache: {}", template_id);
        return Some(tmpl.clone())
    }

    None
}

pub fn put_sql_template(template_id: &str, sql: &'static str) -> DySqlResult<Arc<Template<'static>>> {
    // println!("put template to cache: {}", template_id);
    let cache = get_sql_template_cache();

    let template = Template::new(sql).map_err(|e| {
        DySqlError(ErrorInner::new(Kind::TemplateParseError, Some(Box::new(e))))
    })?;
    cache.write().unwrap().insert(template_id.to_string(), Arc::new(template));

    let template = get_sql_template(template_id);

    if let Some(tmpl) = template {
        return Ok(tmpl.clone())
    }

    Err(DySqlError(ErrorInner::new(Kind::TemplateNotFound, None)))

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

#[allow(unused)]
#[derive(Content, Debug)]
pub struct Value<T> {
    pub value: T
}

impl<T> Value<T> {
    pub fn new(value: T) -> Self {
        Self {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::SqlDialect;
    
    #[test]
    fn test_to_string() {
        let s = SqlDialect::postgres.to_string();
        assert_eq!("postgres", s);
    }
}