use std::{sync::{RwLock, Arc}, collections::HashMap};

use once_cell::sync::OnceCell;
use ramhorns_ext::{Template, Content};

use crate::{DySqlResult, Kind, DySqlError, ErrorInner};

pub static SQL_TEMPLATE_CACHE: OnceCell<RwLock<HashMap<String, Arc<Template>>>> = OnceCell::new();

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
        DySqlError(ErrorInner::new(Kind::TemplateParseError, Some(Box::new(e)), None))
    })?;
    cache.write().unwrap().insert(template_id.to_string(), Arc::new(template));

    let template = get_sql_template(template_id);

    if let Some(tmpl) = template {
        return Ok(tmpl.clone())
    }

    Err(DySqlError(ErrorInner::new(Kind::TemplateNotFound, None, None)))

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