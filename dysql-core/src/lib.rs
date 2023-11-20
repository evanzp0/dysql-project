#![feature(return_position_impl_trait_in_trait)]
mod extract_sql;
mod sql_dialect;
mod error;
mod dysql_context;
mod utils;
mod deps_version;
mod adapter;
mod trim_sql;
mod dto;

pub use extract_sql::*;
pub use sql_dialect::*;
pub use error::*;
pub use dysql_context::*;
pub use utils::*;
pub use deps_version::*;
pub use adapter::*;
pub use trim_sql::*;
pub use dto::*;

#[macro_export]
macro_rules! impl_bind_param_value {
    (
        $query:ident, $p_val:ident, [$($vtype:ty),+]
    ) => {
        paste!{
            match $p_val {
                $(
                    dysql_tpl::SimpleValue::[<t_ $vtype>](val) => $query.bind(val),
                )*
                dysql_tpl::SimpleValue::t_str(val) => $query.bind(unsafe {&*val}),
                dysql_tpl::SimpleValue::t_String(val) => $query.bind(unsafe {&*val}),
                dysql_tpl::SimpleValue::t_Utc(val) => $query.bind(val),
                dysql_tpl::SimpleValue::None(val) => $query.bind(val),
                _ => Err(DySqlError(ErrorInner::new(Kind::BindParamterError, None, Some(format!("the type of {:?} is not support", $p_val)))))?,
            }
        }
    };
}