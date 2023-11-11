
use dysql_tpl::Content;

mod sqlx;

pub use sqlx::*;

pub trait ExecutorAdapter {
    fn executor<D>(dto: D) where D: Content + 'static;
}

