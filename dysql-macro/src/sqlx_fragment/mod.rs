mod fetch_all;
mod fetch_one;
mod fetch_scalar;
mod execute;
mod insert;
mod page;

use dysql_core::get_sqlx_version;
pub use fetch_all::*;
pub use fetch_one::*;
pub use fetch_scalar::*;
pub use execute::*;
pub use insert::*;
pub use page::*;

use quote::quote;

use crate::{DySqlFragmentContext, sql_expand::SqlExpand, QueryType, RefType};

/// 根据 query_type 转发处理 dysql fragrament
pub (crate) fn expand(st: &DySqlFragmentContext, query_type: QueryType) -> syn::Result<proc_macro2::TokenStream> {
    match query_type {
        QueryType::FetchAll => FetchAll.expand(st),
        QueryType::FetchOne => FetchOne.expand(st),
        QueryType::FetchScalar => FetchScalar.expand(st),
        QueryType::Execute => Execute.expand(st),
        QueryType::Insert => Insert.expand(st),
        QueryType::Page => Page.expand(st),
    }
}

/// 根据 sqlx 不同版本对于事务的引用，生成 connection or tran 及其引用的 TokenStream
pub(crate) fn gen_cot_quote(st: &DySqlFragmentContext, cot_ident: &proc_macro2::Ident) -> proc_macro2::TokenStream {
    let cot_ref = match st.cot_ref_type {
        RefType::ReadOnly => quote!(&),
        RefType::Mutable => quote!(&mut ),
        RefType::None => quote!(),
    };

    let cot = if RefType::Mutable == st.cot_ref_type 
        && dysql_core::SqlxVer::V0_7 == get_sqlx_version() {
            quote!(*#cot_ident)
    } else {
        quote!(#cot_ident)
    };
    
    quote!(#cot_ref #cot)
}

/// 生成 dto 及其引用的 TokenStream
pub(crate) fn gen_dto_quote(st: &DySqlFragmentContext, dto_ident: &proc_macro2::Ident) -> proc_macro2::TokenStream {
    let dto_ref = match st.dto_ref_type {
        RefType::ReadOnly => quote!(&),
        RefType::Mutable => quote!(&mut ),
        RefType::None => quote!(),
    };

    quote!(#dto_ref #dto_ident)
}