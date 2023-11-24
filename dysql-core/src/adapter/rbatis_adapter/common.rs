use core::panic;
use std::sync::Arc;

use dysql_tpl::{Content, Template, SimpleValue};
use rbatis::{RBatis, executor::Executor};
use rbs::Value;
use serde::de::DeserializeOwned;

use crate::{SqlDialect, DySqlError, Pagination};

#[cfg(feature = "rbatis-sqlite")]
use crate::RbatisSqliteQuery;
#[cfg(feature = "rbatis-pg")]
use crate::RbatisPostgresQuery;
#[cfg(feature = "rbatis-mysql")]
use crate::RbatisMysqlQuery1;

pub struct RbatisAdapterRouter {
    dialect: SqlDialect
}

impl RbatisAdapterRouter {
    pub fn new(dialect: SqlDialect) -> Self {
        Self {dialect}
    }

    pub async fn fetch_one<E, D, U>(self, executor: &E, named_template: Arc<Template>, dto: Option<D>)
        -> Result<U, DySqlError>
    where 
        E: Executor,
        D: Content + Send + Sync,
        U: DeserializeOwned,
    {
        use SqlDialect::*;
        match self.dialect {
            #[cfg(feature = "rbatis-pg")]
            postgres => RbatisPostgresQuery::new(postgres).fetch_one(executor, named_template, dto).await,
            #[cfg(feature = "rbatis-mysql")]
            mysql => RbatisMysqlQuery1::new(postgres).fetch_one(executor, named_template, dto).await,
            #[cfg(feature = "rbatis-sqlite")]
            sqlite => RbatisSqliteQuery::new(sqlite).fetch_one(executor, named_template, dto).await,
            _ => panic!("{:?} dialect not support", self.dialect),
        }
    }

    pub async fn fetch_all<E, D, U>(self, executor: &E, named_template: Arc<Template>, dto: Option<D>)
        -> Result<Vec<U>, DySqlError>
    where 
        E: Executor,
        D: Content + Send + Sync,
        U: DeserializeOwned,
    {
        use SqlDialect::*;
        match self.dialect {
            #[cfg(feature = "rbatis-pg")]
            postgres => RbatisPostgresQuery::new(postgres).fetch_all(executor, named_template, dto).await,
            #[cfg(feature = "rbatis-mysql")]
            mysql => RbatisMysqlQuery1::new(postgres).fetch_all(executor, named_template, dto).await,
            #[cfg(feature = "rbatis-sqlite")]
            sqlite => RbatisSqliteQuery::new(sqlite).fetch_all(executor, named_template, dto).await,
            _ => panic!("{:?} dialect not support", self.dialect),
        }
    }

    pub async fn fetch_scalar<E, D, U>(self, executor: &E, named_template: Arc<Template>, dto: Option<D>)
        -> Result<U, DySqlError>
    where 
        E: Executor,
        D: Content + Send + Sync,
        U: DeserializeOwned,
    {
        use SqlDialect::*;
        match self.dialect {
            #[cfg(feature = "rbatis-pg")]
            postgres => RbatisPostgresQuery::new(postgres).fetch_scalar(executor, named_template, dto).await,
            #[cfg(feature = "rbatis-mysql")]
            mysql => RbatisMysqlQuery1::new(postgres).fetch_scalar(executor, named_template, dto).await,
            #[cfg(feature = "rbatis-sqlite")]
            sqlite => RbatisSqliteQuery::new(postgres).fetch_scalar(executor, named_template, dto).await,
            _ => panic!("{:?} dialect not support", self.dialect),
        }
    }

    pub async fn execute<E, D>(self, executor: &E, named_template: Arc<Template>, dto: Option<D>)
        -> Result<u64, DySqlError>
    where 
        E: Executor,
        D: Content + Send + Sync,
    {
        use SqlDialect::*;
        match self.dialect {
            #[cfg(feature = "rbatis-pg")]
            postgres => RbatisPostgresQuery::new(postgres).execute(executor, named_template, dto).await,
            #[cfg(feature = "rbatis-mysql")]
            mysql => RbatisMysqlQuery1::new(postgres).execute(executor, named_template, dto).await,
            #[cfg(feature = "rbatis-sqlite")]
            sqlite => RbatisSqliteQuery::new(sqlite).execute(executor, named_template, dto).await,
            _ => panic!("{:?} dialect not support", self.dialect),
        }
    }

    pub async fn insert<E, D, U>(self, executor: &E, named_template: Arc<Template>, dto: Option<D>)
        -> Result<Option<U>, DySqlError>
    where 
        E: Executor,
        D: Content + Send + Sync,
        U: DeserializeOwned,
    {
        use SqlDialect::*;
        match self.dialect {
            #[cfg(feature = "rbatis-pg")]
            postgres => RbatisPostgresQuery::new(postgres).insert(executor, named_template, dto).await,
            #[cfg(feature = "rbatis-mysql")]
            mysql => RbatisMysqlQuery1::new(postgres).insert(executor, named_template, dto).await,
            #[cfg(feature = "rbatis-sqlite")]
            sqlite => RbatisSqliteQuery::new(sqlite).insert(executor, named_template, dto).await,
            _ => panic!("{:?} dialect not support", self.dialect),
        }
    }

    pub async fn fetch_insert_id<E, U>(self, executor: &mut E) 
        -> Result<U, crate::DySqlError>
    where
        E: rbatis::executor::Executor,
        U: serde::de::DeserializeOwned,
    {
        use SqlDialect::*;
        match self.dialect {
            #[cfg(feature = "rbatis-pg")]
            postgres => RbatisPostgresQuery::new(postgres).fetch_insert_id(executor).await,
            #[cfg(feature = "rbatis-mysql")]
            mysql => RbatisMysqlQuery1::new(postgres).fetch_insert_id(executor).await,
            #[cfg(feature = "rbatis-sqlite")]
            sqlite => RbatisSqliteQuery::new(sqlite).fetch_insert_id(executor).await,
            _ => panic!("{:?} dialect not support", self.dialect),
        }
    }

    pub async fn page_count<E, D, U>(self, executor: &E, named_template: Arc<Template>, dto: Option<D>)
        -> Result<U, DySqlError>
    where 
        E: Executor,
        D: Content + Send + Sync,
        U: DeserializeOwned,
    {
        use SqlDialect::*;
        match self.dialect {
            #[cfg(feature = "rbatis-pg")]
            postgres => RbatisPostgresQuery::new(postgres).page_count(executor, named_template, dto).await,
            #[cfg(feature = "rbatis-mysql")]
            mysql => RbatisMysqlQuery1::new(postgres).page_count(executor, named_template, dto).await,
            #[cfg(feature = "rbatis-sqlite")]
            sqlite => RbatisSqliteQuery::new(sqlite).page_count(executor, named_template, dto).await,
            _ => panic!("{:?} dialect not support", self.dialect),
        }
    }

    pub async fn page_all<E, D, U>(self, executor: &E, named_template: Arc<Template>, page_dto: &crate::PageDto<D>)
        -> Result<Pagination<U>, DySqlError>
    where 
        E: Executor,
        D: Content + Send + Sync,
        U: DeserializeOwned,
    {
        use SqlDialect::*;
        match self.dialect {
            #[cfg(feature = "rbatis-pg")]
            postgres => RbatisPostgresQuery::new(postgres).page_all(executor, named_template, page_dto).await,
            #[cfg(feature = "rbatis-mysql")]
            mysql => RbatisMysqlQuery1::new(postgres).page_all(executor, named_template, page_dto).await,
            #[cfg(feature = "rbatis-sqlite")]
            sqlite => RbatisSqliteQuery::new(sqlite).page_all(executor, named_template, page_dto).await,
            _ => panic!("{:?} dialect not support", self.dialect),
        }
    }
}

pub trait RbatisExecutorAdatper
{
    fn create_query(&self) -> RbatisAdapterRouter
    {
        use SqlDialect::*;
        match self.get_dialect() {
            postgres => RbatisAdapterRouter::new(postgres),
            mysql => RbatisAdapterRouter::new(mysql),
            sqlite => RbatisAdapterRouter::new(sqlite),
        }
    }

    fn get_dialect(&self) -> SqlDialect; 
}

impl RbatisExecutorAdatper for rbatis::RBatis {
    fn get_dialect(&self) -> crate::SqlDialect {
        let driver_type = self.driver_type().unwrap();
        SqlDialect::from(driver_type)
    }
}

impl RbatisExecutorAdatper for &rbatis::RBatis {
    fn get_dialect(&self) -> crate::SqlDialect {
        let driver_type = self.driver_type().unwrap();
        SqlDialect::from(driver_type)
    }
}

impl RbatisExecutorAdatper for rbatis::executor::RBatisTxExecutor {
    fn get_dialect(&self) -> crate::SqlDialect {
        let driver_type = self.rb.driver_type().unwrap();
        SqlDialect::from(driver_type)
    }
}

impl RbatisExecutorAdatper for &rbatis::executor::RBatisTxExecutor {
    fn get_dialect(&self) -> SqlDialect {
        let driver_type = self.rb.driver_type().unwrap();
        SqlDialect::from(driver_type)
    }
}

impl RbatisExecutorAdatper for &mut rbatis::executor::RBatisTxExecutor {
    fn get_dialect(&self) -> SqlDialect {
        let driver_type = self.rb.driver_type().unwrap();
        SqlDialect::from(driver_type)
    }
}

pub fn simple_2_value(simple_value: SimpleValue) -> Value {
    match simple_value {
        SimpleValue::t_usize(v) => Value::U64(v as u64),
        SimpleValue::t_isize(v) => Value::I64(v as i64),
        SimpleValue::t_i64(v) => Value::I64(v),
        SimpleValue::t_u64(v) => Value::U64(v),
        SimpleValue::t_i32(v) => Value::I32(v),
        SimpleValue::t_u32(v) => Value::U32(v),
        SimpleValue::t_i16(v) => Value::I32(v as i32),
        SimpleValue::t_u16(v) => Value::U32(v as u32),
        SimpleValue::t_i8(v) => Value::I32(v as i32),
        SimpleValue::t_u8(v) => Value::U32(v as u32),
        SimpleValue::t_f32(v) => Value::F32(v),
        SimpleValue::t_f64(v) => Value::F64(v),
        SimpleValue::t_bool(v) => Value::Bool(v),
        SimpleValue::t_str(v) => Value::String(v.as_str().unwrap().to_owned()),
        SimpleValue::t_String(v) => Value::String(v.as_string().unwrap().clone()),
        SimpleValue::None(_) => Value::Null,
        _ => panic!("{:?} type not support", simple_value),
    }
}