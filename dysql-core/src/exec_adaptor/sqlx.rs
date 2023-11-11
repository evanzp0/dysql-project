use dysql_tpl::Content;

use super::ExecutorAdapter;

pub struct Sqlx;

impl ExecutorAdapter for Sqlx {
    fn executor<D>(dto: D) where D: Content + 'static {
        
    }
}