use std::sync::{RwLock, Arc};

use std::hash::{Hash, Hasher};

use fnv::FnvHasher;

use once_cell::sync::OnceCell;
use dysql_tpl::Template;

use crate::{DySqlError, ErrorInner, Kind, DySqlResult, PersistSql};

pub static SQL_CACHE: OnceCell<RwLock<PersistSql>> = OnceCell::new();

#[allow(dead_code)]
pub fn get_sql_cache(is_save: bool) -> &'static RwLock<PersistSql> {
    let cache = SQL_CACHE.get_or_init(|| {
        let p_sql = PersistSql::default(is_save);
        RwLock::new(p_sql)
    });

    cache
}

pub fn get_sql_template(template_id: u64) -> Option<Arc<Template>> {
    let rst = get_sql_cache(false)
        .read()
        .unwrap()
        .get_template(template_id);
    
    if let Some(_) = rst {
        println!("hit: {}", template_id);
    } else {
        println!("not hit: {}", template_id);
    }

    rst
}

pub fn put_sql_template(template_id: u64, serd_template: &[u8]) -> DySqlResult<Arc<Template>> {
    let template = Template::deserialize(serd_template);

    // let template = Template::new(sql).map_err(|e| {
    //     DySqlError(ErrorInner::new(Kind::TemplateParseError, Some(Box::new(e)), None))
    // })?;

    let template = Arc::new(template);

    get_sql_cache(false)
        .write()
        .unwrap()
        .insert_template(template_id, template.clone());

    Ok(template)
}

pub fn save_sql_template(source_file: &str, template_id: u64, sql: &str, sql_name: Option<String>) -> DySqlResult<()> {
    let source_file = if let Ok(path) = home::cargo_home() {
        // 如果处理是 repository 的文件，则源文件路径去除 cargo_home
        let cargo_home = path.to_str().expect("cargo_home path cannot to string");
        if source_file.starts_with(cargo_home) {
            source_file[cargo_home.len()..].to_owned()
        } else {
            source_file.to_owned()
        }
    } else {
        source_file.to_owned()
    };

    let template = Template::new(sql).map_err(|e| {
        DySqlError(ErrorInner::new(Kind::TemplateParseError, Some(Box::new(e)), None))
    })?;

    let template = Arc::new(template);

    let meta_id = hash_str(&source_file);
    get_sql_cache(true)
        .write()
        .unwrap()
        .save_sql_template(meta_id, source_file, template_id, template, sql_name);

    Ok(())
}


pub fn hash_str(name: &str) -> u64 {
    let mut hasher = FnvHasher::default();
    name.hash(&mut hasher);
    hasher.finish()
}
