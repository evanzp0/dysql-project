use quote::quote;

use crate::sql_expand::SqlExpand;

pub struct FetchAll;

impl SqlExpand for FetchAll {

    fn expand(&self, st: &crate::DySqlFragmentContext) -> syn::Result<proc_macro2::TokenStream> {
        let dto = &st.dto;
        let cot = &st.cot;
        let ret_type = &st.ret_type;
    
        let cot = super::gen_cot_quote(st, cot);
        let (param_strings, param_idents) = self.extra_params(st)?;

        // declare sql & param_names, and bind params value at runtime
        let declare_rt = self.gen_declare_rt(st, None, false)?;

        let ret = match dto {
            Some(_) => quote!(
                let mut query = sqlx::query_as::<_, #ret_type>(&sql);
                for i in 0..param_names.len() {
                    #(
                        if param_names[i] == #param_strings {
                            query = query.bind(&#dto.#param_idents);
                        }
                    )*
                }
    
                let rst = query.fetch_all(#cot).await;
                if let Err(e) = rst {
                    break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::QueryError, Some(Box::new(e)), None)))
                }
                let rst = rst.expect("Unexpected error");
                Ok(rst)
            ),
            None => quote!(
                let mut query = sqlx::query_as::<_, #ret_type>(&sql);
                let rst = query.fetch_all(#cot).await;
                if let Err(e) = rst {
                    break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::QueryError, Some(Box::new(e)), None)))
                }
                let rst = rst.expect("Unexpected error");
                Ok(rst)
            ),
        };
    
        let ret = quote!('rst_block: {
            #declare_rt
            #ret
        });

        Ok(ret)
    }
}