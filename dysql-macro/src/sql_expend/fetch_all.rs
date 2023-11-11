use quote::quote;

use crate::sql_expand::SqlExpand;

pub struct FetchAll;

impl SqlExpand for FetchAll {

    fn expand(&self, st: &crate::DySqlFragment) -> syn::Result<proc_macro2::TokenStream> {
        let dto_ident = &st.dto;

        // declare named_sql at runtime
        let named_sql_declare = self.gen_named_sql_declare(st, &st.body, false)?;

        let query_declare = if let Some(dto) = dto_ident {
            quote!(
                let query = dysql::Query::new(
                    dysql::QueryCmd::FetchAll(named_sql), 
                    Some(#dto)
                );
            )
        } else {
            quote!(
                let query = dysql::Query::new(query_type, None);
            )
        };

        let ret = quote!('rst_block: {
            #named_sql_declare  // let named_sql = ....;
            #query_declare      // let query = dysql::Query::new....;

            query
        });

        Ok(ret)
    }
}