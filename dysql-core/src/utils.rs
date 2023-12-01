#![allow(unused)]
use std::collections::HashMap;
use std::sync::{RwLock, Arc};

use std::hash::{Hash, Hasher};

use fnv::FnvHasher;

use log::trace;
use once_cell::sync::OnceCell;
use dysql_tpl::{Template, Content};

use crate::{DySqlError, ErrorInner, Kind, DySqlResult, DysqlContext};

pub static SQL_TEMPLATE_CACHE: OnceCell<RwLock<DysqlContext>> = OnceCell::new();

#[allow(dead_code)]
pub fn get_sql_template_cache() -> &'static RwLock<DysqlContext> {
    let cache = SQL_TEMPLATE_CACHE.get_or_init(|| {
        let p_sql = DysqlContext::default();
        RwLock::new(p_sql)
    });

    cache
}

pub fn get_sql_template(template_id: u64) -> Option<Arc<Template>> {
    let rst = get_sql_template_cache()
        .read()
        .unwrap()
        .get_template(template_id);
    
    if log::log_enabled!(log::Level::Trace) {
        if let Some(_) = rst {
            trace!("hit: {}", template_id);
        } else {
            trace!("not hit: {}", template_id);
        }
    }

    rst
}

pub fn put_sql_template(template_id: u64, serd_template: &[u8]) -> DySqlResult<Arc<Template>> {
    let template = Template::deserialize(serd_template);

    // let template = Template::new(sql).map_err(|e| {
    //     DySqlError(ErrorInner::new(Kind::TemplateParseError, Some(Box::new(e)), None))
    // })?;

    let template = Arc::new(template);

    get_sql_template_cache()
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

    let meta_id = hash_it(&source_file);
    get_sql_template_cache()
        .write()
        .unwrap()
        .save_sql_template(meta_id, source_file, template_id, template, sql_name);

    Ok(())
}

pub fn hash_it<T: Hash>(name: T) -> u64 {
    let mut hasher = FnvHasher::default();
    name.hash(&mut hasher);
    hasher.finish()
}

pub fn gen_named_sql<D>(named_template: Arc<Template>, dto: &Option<D>) -> Result<String, DySqlError>
where 
    D: Content + Send + Sync
{
    let named_sql = if let Some(dto) = dto {
        named_template.render_sql(dto)
    } else {
        named_template.source().to_owned()
    };
    
    Ok(named_sql)
}

// fn ptr_to_str<'a>(ptr: *const str, len: usize) -> &'static str {
//     let p = ptr as * const u8;
//     unsafe {
//         std::str::from_utf8_unchecked(
//             std::slice::from_raw_parts(p, len)
//         )
//     }
// }

// use std::any::{Any, TypeId};
// pub trait InstanceOf
// where
//     Self: Any,
// {
//     fn instance_of<U: ?Sized + Any>(&self) -> bool {
//         TypeId::of::<Self>() == TypeId::of::<U>()
//     }

//     fn instance_of_mut<U: ?Sized + Any>(&mut self) -> bool {
//         TypeId::of::<Self>() == TypeId::of::<U>()
//     }
// }
// impl<T: ?Sized + Any> InstanceOf for T {}

// /// 包装出一个用于 SQL 参数绑定的闭包
// pub fn wrap_binder_fn<V, Binder, F>(f: F) -> impl FnOnce(V, Binder) -> Binder
// where 
//     F: FnOnce(V, Binder) -> Binder,
//     V: Content + 'static,
// {
//     f
// }

#[cfg(test)]
pub mod test_suite {
    use futures::future::BoxFuture;

    pub fn block_on<T>(task: T) -> T::Output
    where
        T: std::future::Future + Send + 'static,
        T::Output: Send + 'static,
    {
        tokio::task::block_in_place(|| {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("tokio block_on fail")
                .block_on(task)
        })
    }

    pub trait QPS {
        fn qps(&self, total: u64);
        fn time(&self, total: u64);
        fn cost(&self);
    }

    impl QPS for std::time::Instant {
        fn qps(&self, total: u64) {
            let time = self.elapsed();
            println!(
                "use QPS: {} QPS/s",
                (total as u128 * 1000000000 as u128 / time.as_nanos() as u128)
            );
        }

        fn time(&self, total: u64) {
            let time = self.elapsed();
            println!(
                "use Time: {:?} ,each:{} ns/op",
                &time,
                time.as_nanos() / (total as u128)
            );
        }

        fn cost(&self) {
            let time = self.elapsed();
            println!("cost:{:?}", time);
        }
    }

    #[macro_export]
    macro_rules! rbench {
        ($total:expr,$body:block) => {{
            use crate::utils::test_suite::QPS;
            let now = std::time::Instant::now();
            for _ in 0..$total {
                $body;
            }
            now.time($total);
            now.qps($total);
        }};
    }
}
