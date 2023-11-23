use std::sync::Arc;

use dysql_tpl::{Content, Template, SimpleValue};
use rbatis::{RBatis, executor::Executor};
use rbs::Value;
use serde::de::DeserializeOwned;
use crate::{SqlDialect, DySqlError, RbatisSqliteQuery};

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
            postgres => todo!(),
            mysql => todo!(),
            sqlite => RbatisSqliteQuery::new(postgres).fetch_one(executor, named_template, dto).await,
            mssql => todo!(),
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
            mssql => RbatisAdapterRouter::new(mssql),
        }
    }

    fn get_dialect(&self) -> SqlDialect; 
}

impl RbatisExecutorAdatper for RBatis {
    fn get_dialect(&self) -> SqlDialect {
        let driver_type = self.driver_type().unwrap();
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