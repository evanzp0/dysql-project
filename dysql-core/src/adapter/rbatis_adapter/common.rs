use core::panic;
use std::sync::Arc;

use dysql_tpl::{Content, Template, SimpleValue};
use rbatis::{RBatis, executor::Executor};
use rbs::Value;
use serde::de::DeserializeOwned;

use crate::{SqlDialect, DySqlError, Pagination};

#[cfg(feature = "rbatis-sqlite")]
use crate::RbatisSqliteAdapter;
#[cfg(feature = "rbatis-pg")]
use crate::RbatisPostgresAdapter;
#[cfg(feature = "rbatis-mysql")]
use crate::RbatisMysqlAdapter;

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

pub trait RbatisExecutorAdatper
{
    fn get_dialect(&self) -> SqlDialect;

    /// 查询并返回多个指定类型的对象
    async fn dy_fetch_all<D, U>(self, template_id: u64, named_template: Arc<Template>, dto: Option<D>)
        -> Result<Vec<U>, DySqlError>
    where 
        D: Content + Send + Sync,
        U: DeserializeOwned;

    /// 查询并返回一个指定类型的对象
    async fn dy_fetch_one<D, U>(self, template_id: u64, named_template: Arc<Template>, dto: Option<D>)
        -> Result<U, DySqlError>
    where 
        D: Content + Send + Sync,
        U: DeserializeOwned;

    /// 查询并返回一个指定类型的单值
    async fn dy_fetch_scalar<D, U>(self, template_id: u64, named_template: Arc<Template>, dto: Option<D>)
        -> Result<U, DySqlError>
    where 
        D: Content + Send + Sync,
        U: DeserializeOwned;

    /// 执行一条sql命令并返回受其影响的记录数
    async fn dy_execute<D>(self, template_id: u64, named_template: Arc<Template>, dto: Option<D>)
        -> Result<u64, DySqlError>
    where 
        D: Content + Send + Sync;

    /// 新增一条记录
    async fn dy_insert<D, U>(self, template_id: u64, named_template: Arc<Template>, dto: Option<D>)
        -> Result<Option<U>, DySqlError>
    where 
        D: Content + Send + Sync,
        U: DeserializeOwned;

    /// 获取新增记录的ID
    async fn dy_fetch_insert_id<U>(self) 
        -> Result<Option<U>, crate::DySqlError>
    where
        U: serde::de::DeserializeOwned;

    /// 用于在分页查询中获取符合条件的总记录数
    async fn dy_page_count<D, U>(self, template_id: u64, named_template: Arc<Template>, dto: Option<D>)
        -> Result<U, DySqlError>
    where 
        D: Content + Send + Sync,
        U: DeserializeOwned;

    /// 用返回分页查询中获取符合条件的结果
    async fn dy_page_all<D, U>(self, template_id: u64, named_template: Arc<Template>, page_dto: &crate::PageDto<D>)
        -> Result<Pagination<U>, DySqlError>
    where 
        D: Content + Send + Sync,
        U: DeserializeOwned;
}

macro_rules! impl_rbatis_adapter_fetch_all_0 {
    () => {
        async fn dy_fetch_all<D, U>(self, template_id: u64, named_template: Arc<Template>, dto: Option<D>)
            -> Result<Vec<U>, DySqlError>
        where 
            D: Content + Send + Sync,
            U: DeserializeOwned,
        {
            use SqlDialect::*;
            match self.get_dialect() {
                #[cfg(feature = "rbatis-pg")]
                postgres => RbatisPostgresAdapter::new(postgres).dy_fetch_all(self, template_id, named_template, dto).await,
                #[cfg(feature = "rbatis-mysql")]
                mysql => RbatisMysqlAdapter::new(mysql).dy_fetch_all(self, template_id, named_template, dto).await,
                #[cfg(feature = "rbatis-sqlite")]
                sqlite => RbatisSqliteAdapter::new(sqlite).dy_fetch_all(self, template_id, named_template, dto).await,
                _ => panic!("{:?} dialect not support", self.get_dialect()),
            }
        }
    };
}

macro_rules! impl_rbatis_adapter_fetch_one_0 {
    () => {
        async fn dy_fetch_one<D, U>(self, template_id: u64, named_template: Arc<Template>, dto: Option<D>)
        -> Result<U, DySqlError>
        where 
            D: Content + Send + Sync,
            U: DeserializeOwned,
        {
            use SqlDialect::*;
            match self.get_dialect() {
                #[cfg(feature = "rbatis-pg")]
                postgres => RbatisPostgresAdapter::new(postgres).dy_fetch_one(self, template_id, named_template, dto).await,
                #[cfg(feature = "rbatis-mysql")]
                mysql => RbatisMysqlAdapter::new(mysql).dy_fetch_one(self, template_id, named_template, dto).await,
                #[cfg(feature = "rbatis-sqlite")]
                sqlite => RbatisSqliteAdapter::new(sqlite).dy_fetch_one(self, template_id, named_template, dto).await,
                _ => panic!("{:?} dialect not support", self.get_dialect()),
            }
        }
    };
}

macro_rules! impl_rbatis_adapter_fetch_scalar_0 {
    () => {
        async fn dy_fetch_scalar<D, U>(self, template_id: u64, named_template: Arc<Template>, dto: Option<D>)
        -> Result<U, DySqlError>
        where 
            D: Content + Send + Sync,
            U: DeserializeOwned,
        {
            use SqlDialect::*;
            match self.get_dialect() {
                #[cfg(feature = "rbatis-pg")]
                postgres => RbatisPostgresAdapter::new(postgres).dy_fetch_scalar(self, template_id, named_template, dto).await,
                #[cfg(feature = "rbatis-mysql")]
                mysql => RbatisMysqlAdapter::new(mysql).dy_fetch_scalar(self, template_id, named_template, dto).await,
                #[cfg(feature = "rbatis-sqlite")]
                sqlite => RbatisSqliteAdapter::new(sqlite).dy_fetch_scalar(self, template_id, named_template, dto).await,
                _ => panic!("{:?} dialect not support", self.get_dialect()),
            }
        }
    };
}

macro_rules! impl_rbatis_adapter_fetch_execute_0 {
    () => {
        async fn dy_execute<D>(self, template_id: u64, named_template: Arc<Template>, dto: Option<D>)
            -> Result<u64, DySqlError>
        where 
            D: Content + Send + Sync,
        {
            use SqlDialect::*;
            match self.get_dialect() {
                #[cfg(feature = "rbatis-pg")]
                postgres => RbatisPostgresAdapter::new(postgres).dy_execute(self, template_id, named_template, dto).await,
                #[cfg(feature = "rbatis-mysql")]
                mysql => RbatisMysqlAdapter::new(mysql).dy_execute(self, template_id, named_template, dto).await,
                #[cfg(feature = "rbatis-sqlite")]
                sqlite => RbatisSqliteAdapter::new(sqlite).dy_execute(self, template_id, named_template, dto).await,
                _ => panic!("{:?} dialect not support", self.get_dialect()),
            }
        }
    };
}

macro_rules! impl_rbatis_adapter_insert_0 {
    () => {
        async fn dy_insert<D, U>(self, template_id: u64, named_template: Arc<Template>, dto: Option<D>)
            -> Result<Option<U>, DySqlError>
        where 
            D: Content + Send + Sync,
            U: DeserializeOwned,
        {
            use SqlDialect::*;
            match self.get_dialect() {
                #[cfg(feature = "rbatis-pg")]
                postgres => RbatisPostgresAdapter::new(postgres).dy_insert(self, template_id, named_template, dto).await,
                #[cfg(feature = "rbatis-mysql")]
                mysql => RbatisMysqlAdapter::new(mysql).dy_insert(self, template_id, named_template, dto).await,
                #[cfg(feature = "rbatis-sqlite")]
                sqlite => RbatisSqliteAdapter::new(sqlite).dy_insert(self, template_id, named_template, dto).await,
                _ => panic!("{:?} dialect not support", self.get_dialect()),
            }
        }
    };
}

macro_rules! impl_rbatis_adapter_fetch_insert_id_0 {
    () => {
        async fn dy_fetch_insert_id<U>(self) 
            -> Result<Option<U>, crate::DySqlError>
        where
            U: serde::de::DeserializeOwned,
        {
            use SqlDialect::*;
            match self.get_dialect() {
                #[cfg(feature = "rbatis-pg")]
                postgres => RbatisPostgresAdapter::new(postgres).dy_fetch_insert_id(self).await,
                #[cfg(feature = "rbatis-mysql")]
                mysql => RbatisMysqlAdapter::new(mysql).dy_fetch_insert_id(self).await,
                #[cfg(feature = "rbatis-sqlite")]
                sqlite => RbatisSqliteAdapter::new(sqlite).dy_fetch_insert_id(self).await,
                _ => panic!("{:?} dialect not support", self.get_dialect()),
            }
        }
    };
}

macro_rules! impl_rbatis_adapter_fetch_page_count_0 {
    () => {
        async fn dy_page_count<D, U>(self, template_id: u64, named_template: Arc<Template>, dto: Option<D>)
            -> Result<U, DySqlError>
        where 
            D: Content + Send + Sync,
            U: DeserializeOwned,
        {
            use SqlDialect::*;
            match self.get_dialect() {
                #[cfg(feature = "rbatis-pg")]
                postgres => RbatisPostgresAdapter::new(postgres).dy_page_count(self, template_id, named_template, dto).await,
                #[cfg(feature = "rbatis-mysql")]
                mysql => RbatisMysqlAdapter::new(mysql).dy_page_count(self, template_id, named_template, dto).await,
                #[cfg(feature = "rbatis-sqlite")]
                sqlite => RbatisSqliteAdapter::new(sqlite).dy_page_count(self, template_id, named_template, dto).await,
                _ => panic!("{:?} dialect not support", self.get_dialect()),
            }
        }
    };
}

macro_rules! impl_rbatis_adapter_fetch_page_all_0 {
    () => {
        async fn dy_page_all<D, U>(self, template_id: u64, named_template: Arc<Template>, page_dto: &crate::PageDto<D>)
            -> Result<Pagination<U>, DySqlError>
        where 
            D: Content + Send + Sync,
            U: DeserializeOwned,
        {
            use SqlDialect::*;
            match self.get_dialect() {
                #[cfg(feature = "rbatis-pg")]
                postgres => RbatisPostgresAdapter::new(postgres).dy_page_all(self, template_id, named_template, page_dto).await,
                #[cfg(feature = "rbatis-mysql")]
                mysql => RbatisMysqlAdapter::new(mysql).dy_page_all(self, template_id, named_template, page_dto).await,
                #[cfg(feature = "rbatis-sqlite")]
                sqlite => RbatisSqliteAdapter::new(sqlite).dy_page_all(self, template_id, named_template, page_dto).await,
                _ => panic!("{:?} dialect not support", self.get_dialect()),
            }
        }
    };
}

impl RbatisExecutorAdatper for &rbatis::RBatis {
    fn get_dialect(&self) -> crate::SqlDialect {
        let driver_type = self.driver_type().unwrap();
        SqlDialect::from(driver_type)
    }

    impl_rbatis_adapter_fetch_all_0!();
    impl_rbatis_adapter_fetch_one_0!();
    impl_rbatis_adapter_fetch_scalar_0!();
    impl_rbatis_adapter_fetch_execute_0!();
    impl_rbatis_adapter_insert_0!();
    impl_rbatis_adapter_fetch_insert_id_0!();
    impl_rbatis_adapter_fetch_page_count_0!();
    impl_rbatis_adapter_fetch_page_all_0!();
}

impl RbatisExecutorAdatper for &rbatis::executor::RBatisTxExecutor {
    fn get_dialect(&self) -> crate::SqlDialect {
        let driver_type = self.rb.driver_type().unwrap();
        SqlDialect::from(driver_type)
    }

    impl_rbatis_adapter_fetch_all_0!();
    impl_rbatis_adapter_fetch_one_0!();
    impl_rbatis_adapter_fetch_scalar_0!();
    impl_rbatis_adapter_fetch_execute_0!();
    impl_rbatis_adapter_insert_0!();
    impl_rbatis_adapter_fetch_insert_id_0!();
    impl_rbatis_adapter_fetch_page_count_0!();
    impl_rbatis_adapter_fetch_page_all_0!();
}

impl RbatisExecutorAdatper for &rbatis::executor::RBatisConnExecutor {
    fn get_dialect(&self) -> crate::SqlDialect {
        let driver_type = self.rb.driver_type().unwrap();
        SqlDialect::from(driver_type)
    }

    impl_rbatis_adapter_fetch_all_0!();
    impl_rbatis_adapter_fetch_one_0!();
    impl_rbatis_adapter_fetch_scalar_0!();
    impl_rbatis_adapter_fetch_execute_0!();
    impl_rbatis_adapter_insert_0!();
    impl_rbatis_adapter_fetch_insert_id_0!();
    impl_rbatis_adapter_fetch_page_count_0!();
    impl_rbatis_adapter_fetch_page_all_0!();
}
