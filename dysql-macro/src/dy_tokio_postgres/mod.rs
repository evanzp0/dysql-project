mod fetch_all;
mod fetch_one;
mod fetch_scalar;
mod execute;
mod insert;

use dysql::QueryType;

pub use fetch_all::*;
pub use fetch_one::*;
pub use fetch_scalar::*;
pub use execute::*;
pub use insert::*;

use crate::{SqlClosure, sql_expand::SqlExpand};

pub (crate) fn expand(st: &SqlClosure, query_type: QueryType) -> syn::Result<proc_macro2::TokenStream> {
    match query_type {
        QueryType::FetchAll => FetchAll.expand(st),
        QueryType::FetchOne => FetchOne.expand(st),
        QueryType::FetchScalar => FetchScalar.expand(st),
        QueryType::Execute => Execute.expand(st),
        QueryType::Insert => Insert.expand(st),
    }
}