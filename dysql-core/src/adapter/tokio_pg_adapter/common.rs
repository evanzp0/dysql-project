use async_trait::async_trait;

pub struct TokioPgQuery;

/// tokio-postgres Executor 的适配接口
pub trait TokioPgExecutorAdatper
{
    fn get_dialect(&self) -> crate::SqlDialect 
    {
        return crate::SqlDialect::postgres
    }

    /// 查询并返回多个指定类型的对象
    async fn dy_fetch_all<D, U>(self, template_id: u64, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>)
        -> Result<Vec<U>, crate::DySqlError>
    where 
        D: dysql_tpl::Content + Send + Sync,
        U: tokio_pg_mapper::FromTokioPostgresRow;

    /// 查询并返回一个指定类型的对象
    async fn dy_fetch_one<D, U>(self, template_id: u64, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>)
        -> Result<U, crate::DySqlError>
    where 
        D: dysql_tpl::Content + Send + Sync,
        U: tokio_pg_mapper::FromTokioPostgresRow;

    /// 查询并返回一个指定类型的单值
    async fn dy_fetch_scalar<D, U>(self, template_id: u64, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>)
        -> Result<U, crate::DySqlError>
    where 
        D: dysql_tpl::Content + Send + Sync,
        for<'a> U: tokio_postgres::types::FromSql<'a>;

    /// 执行一条sql命令并返回受其影响的记录数
    async fn dy_execute<D>(self, template_id: u64, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>)
        -> Result<u64, crate::DySqlError>
    where 
        D: dysql_tpl::Content + Send + Sync;

    /// 新增一条记录
    async fn dy_insert<D, U>(self, template_id: u64, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>)
        -> Result<Option<U>, crate::DySqlError>
    where 
        D: dysql_tpl::Content + Send + Sync,
        for<'a> U: tokio_postgres::types::FromSql<'a>;

    /// 获取新增记录的ID
    async fn dy_fetch_insert_id<U>(self)
        -> Result<Option<U>, crate::DySqlError>
    where
        for<'a> U: tokio_postgres::types::FromSql<'a>;

    /// 用于在分页查询中获取符合条件的总记录数
    async fn dy_page_count<D, U>(self, template_id: u64, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>)
        -> Result<U, crate::DySqlError>
    where 
        D: dysql_tpl::Content + Send + Sync,
        for<'a> U: tokio_postgres::types::FromSql<'a>;

    /// 用返回分页查询中获取符合条件的结果
    async fn dy_page_all<D, U>(self, template_id: u64, named_template: std::sync::Arc<dysql_tpl::Template>, page_dto: &crate::PageDto<D>)
        -> Result<crate::Pagination<U>, crate::DySqlError>
    where 
        D: dysql_tpl::Content + Send + Sync,
        U: tokio_pg_mapper::FromTokioPostgresRow;
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
                dysql_tpl::SimpleValue::t_str(val) =>  $param_values.push(val),
                dysql_tpl::SimpleValue::t_String(val) => $param_values.push(val),
                dysql_tpl::SimpleValue::None(val) => $param_values.push(val),
                _ => Err(crate::DySqlError(crate::ErrorInner::new(crate::Kind::BindParamterError, None, Some(format!("the type of {:?} is not support", $p_val)))))?,
            }
        }
    };
}

