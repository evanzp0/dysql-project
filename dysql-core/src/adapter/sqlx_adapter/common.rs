
/// 用于绑定 sql查询中的命名参数的宏
#[macro_export]
macro_rules! impl_bind_sqlx_param_value {
    (
        $query:ident, $p_val:ident, [$($vtype:ty),+]
    ) => {
        paste::paste!{
            match $p_val {
                $(
                    dysql_tpl::SimpleValue::[<t_ $vtype>](val) => $query.bind(val),
                )*
                dysql_tpl::SimpleValue::t_str(val) => $query.bind(unsafe {&*val.0}),
                dysql_tpl::SimpleValue::t_String(val) => $query.bind(unsafe {&*val.0}),
                dysql_tpl::SimpleValue::None(val) => $query.bind(val),
                _ => Err(crate::DySqlError(crate::ErrorInner::new(crate::Kind::BindParamterError, None, Some(format!("the type of {:?} is not support", $p_val)))))?,
            }
        }
    };
}

/// Sqlx Executor 的适配接口
pub trait SqlxExecutorAdatper
{
    type DB: sqlx::Database;
    type Row: sqlx::Row<Database = Self::DB>;

    /// 获取 DB 类型
    fn get_dialect(&self) -> crate::SqlDialect 
    {
        // 以下分支需要用条件宏进行编译
        #[cfg(feature = "sqlx-postgres")]
        if std::any::TypeId::of::<Self::DB>() == std::any::TypeId::of::<sqlx::Postgres>() {

            return crate::SqlDialect::postgres
        }
        
        #[cfg(feature = "sqlx-mysql")]
        if std::any::TypeId::of::<Self::DB>() == std::any::TypeId::of::<sqlx::MySql>() {

            return crate::SqlDialect::mysql
        } 
        
        #[cfg(feature = "sqlx-sqlite")]
        if std::any::TypeId::of::<Self::DB>() == std::any::TypeId::of::<sqlx::Sqlite>() {

            return crate::SqlDialect::sqlite
        }

        panic!("only support 'postgres', 'mysql', 'sqlite' sql dialect")
    }

    /// 查询并返回多个指定类型的对象
    async fn fetch_all<D, U>(self, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>) 
        -> Result<Vec<U>, crate::DySqlError>
    where
        D: dysql_tpl::Content + Send + Sync,
        for<'r> U: sqlx::FromRow<'r, Self::Row> + Send + Unpin;

    /// 查询并返回一个指定类型的对象
    async fn fetch_one<D, U>(self, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>) 
        -> Result<U, crate::DySqlError>
    where
        D: dysql_tpl::Content + Send + Sync,
        for<'r> U: sqlx::FromRow<'r, Self::Row> + Send + Unpin;

    /// 查询并返回一个指定类型的单值
    async fn fetch_scalar<D, U>(self, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>) 
        -> Result<U, crate::DySqlError>
    where
        D: dysql_tpl::Content + Send + Sync,
        for<'r> U: sqlx::Decode<'r, Self::DB> + sqlx::Type<Self::DB> + Send + Unpin;

    /// 执行一条sql命令并返回受其影响的记录数
    async fn execute<D>(self,named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>) 
        -> Result<u64, crate::DySqlError>
    where
        D: dysql_tpl::Content + Send + Sync;

    /// 新增一条记录
    async fn insert<D, U>(self, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>) 
        -> Result<Option<U>, crate::DySqlError>
    where
        D: dysql_tpl::Content + Send + Sync,
        for<'r> U: sqlx::Decode<'r, Self::DB> + sqlx::Type<Self::DB> + Send + Unpin;
    
    /// 获取新增记录的ID
    async fn fetch_insert_id<U>(self)
        -> Result<Option<U>, crate::DySqlError>
    where
        for<'r> U: sqlx::Decode<'r, Self::DB> + sqlx::Type<Self::DB> + Send + Unpin;

    /// 用于在分页查询中获取符合条件的总记录数
    async fn page_count<D, U>(self, named_template: std::sync::Arc<dysql_tpl::Template>, dto: Option<D>) 
        -> Result<U, crate::DySqlError>
    where
        D: dysql_tpl::Content + Send + Sync,
        for<'r> U: sqlx::Decode<'r, Self::DB> + sqlx::Type<Self::DB> + Send + Unpin;

    /// 用返回分页查询中获取符合条件的结果
    async fn page_all<D, U>(self, named_template: std::sync::Arc<dysql_tpl::Template>, page_dto: &crate::PageDto<D>) 
        -> Result<crate::Pagination<U>, crate::DySqlError>
    where
        D: dysql_tpl::Content + Send + Sync,
        for<'r> U: sqlx::FromRow<'r, Self::Row> + Send + Unpin;
}
