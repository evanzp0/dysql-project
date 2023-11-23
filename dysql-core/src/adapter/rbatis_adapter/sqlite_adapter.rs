
use crate::{SqlDialect, impl_rbatis_adapter_fetch_one};

pub struct RbatisSqliteQuery{
    dialect: SqlDialect
}

impl RbatisSqliteQuery {
    pub fn new(dialect: SqlDialect) -> Self {
        Self { dialect }
    }
}

impl RbatisSqliteQuery {
    impl_rbatis_adapter_fetch_one!();
}