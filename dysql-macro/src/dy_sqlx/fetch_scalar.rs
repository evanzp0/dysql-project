use quote::quote;

use crate::sql_expand::SqlExpand;

pub struct FetchScalar;

impl SqlExpand for FetchScalar {

    fn expand(&self, st: &crate::SqlClosure) -> syn::Result<proc_macro2::TokenStream> {
        let dto = &st.dto;
        let cot = &st.cot;
        let ret_type = &st.ret_type;
    
        let cot_ref = if st.is_cot_ref_mut {
            quote!(&mut )
        } else if st.is_cot_ref {
            quote!(&)
        } else {
            quote!()
        };

        let (param_strings, param_idents) = self.extra_params(st)?;

        // declare sql and bind params at runtime
        let declare_rt = self.gen_declare_rt(st)?;

        let ret = match dto {
            Some(_) => quote!(
                let mut query = sqlx::query_scalar::<_, #ret_type>(&sql);
                for i in 0..param_names.len() {
                    #(
                        if param_names[i] == #param_strings {
                            query = query.bind(&#dto.#param_idents);
                        }
                    )*
                }
        
                let rst = query.fetch_one(#cot_ref #cot).await;
                if let Err(e) = rst {
                    break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::QueryError, Some(Box::new(e)))))
                }
                let rst = rst.expect("Unexpected error");
                Ok(rst)
            ),
            None => quote!(
                let mut query = sqlx::query_scalar::<_, #ret_type>(&sql);
                let rst = query.fetch_one(#cot_ref #cot).await;
                if let Err(e) = rst {
                    break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::QueryError, Some(Box::new(e)))))
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