use async_trait::async_trait;
use tokio_postgres::{Statement, Error, types::ToSql, Row, ToStatement};
use crate::SqlDialect;

pub struct TokioPgQuery;

/// 使用了 async_trait 会导致 trait 中的异步方法的返回值都被包装成 BoxPin，
/// 这样会有性能损耗，但是 tokio-postgres 的性能本来就差，所以为了偷懒我就这么干了，
/// 否则需要为 tokio-postgres 的 transaction 和 connection 注入不同的 ExecutorAdapter，
/// 来为它们创建不同的 QueryAdapter 对象，然后再在各自的 QueryAdapter 中，
/// 分别实现对 transaction 和 connection 的 query 方法的转发调用
#[async_trait]
pub trait TokioPgExecutorAdatper
{
    fn create_query(&self) -> TokioPgQuery
    {
        TokioPgQuery
    }

    fn get_dialect(&self) -> SqlDialect 
    {
        return SqlDialect::postgres
    }

    async fn prepare(&self, query: &str) -> Result<Statement, Error>;

    async fn query<T>(
        &self, 
        statement: &T, 
        params: &[&(dyn ToSql + Sync)]
    ) -> Result<Vec<Row>, Error> 
    where
        T: ?Sized + ToStatement + Sync;

    async fn query_one<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Row, Error>
    where
        T: ?Sized + ToStatement + Sync;

    async fn execute<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<u64, Error>
    where
        T: ?Sized + ToStatement + Sync;
}

#[macro_export]
macro_rules! impl_bind_tokio_pg_param_value {
    (
        $param_values:ident, $p_val:ident, [$($vtype:ty),+]
    ) => {
        paste::paste!{
            match $p_val {
                $(
                    dysql_tpl::SimpleValue::[<t_ $vtype>](val) => $param_values.push(val),
                )*
                dysql_tpl::SimpleValue::t_str(val) => {
                    let tmp = unsafe { &**val};
                    let len = tmp.len();
                    
                    fn ptr_to_str<'a>(ptr: *const str, len: usize) -> &'static str {
                        let p = ptr as * const u8;
                        unsafe {
                            std::str::from_utf8_unchecked(
                                std::slice::from_raw_parts(p, len)
                            )
                        }
                    }
                    // let a = ptr_to_str(*val, len);

                    // $param_values.push(&a);

                },
                // dysql_tpl::SimpleValue::t_String(val) => $param_values.push((unsafe {&*val})),
                dysql_tpl::SimpleValue::None(val) => $param_values.push(val),
                _ => Err(DySqlError(ErrorInner::new(Kind::BindParamterError, None, Some(format!("the type of {:?} is not support", $p_val)))))?,
            }
        }
    };
}

