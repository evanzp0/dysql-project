use quote::quote;

use crate::sql_expand::SqlExpand;

pub struct Execute;

impl SqlExpand for Execute {

    fn expand(&self, st: &crate::DySqlFragmentContext) -> syn::Result<proc_macro2::TokenStream> {
        let dto = &st.dto;
        let cot = &st.cot;

        let cot = super::gen_cot_quote(st, cot);
        let (param_strings, param_idents) = self.extra_params(st)?;

        // declare sql and bind params at runtime
        let declare_rt = self.gen_named_sql_declare(st, None, false)?;

        let ret = match dto {
            Some(_) => quote!(
                let mut query = sqlx::query(&sql);
                for i in 0..param_names.len() {
                    #(
                        if param_names[i] == #param_strings {
                            query = query.bind(&#dto.#param_idents);
                        }
                    )*
                }
        
                let rst = query.execute(#cot).await;
                if let Err(e) = rst {
                    break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::QueryError, Some(Box::new(e)), None)))
                }
                let rst = rst.expect("Unexpected error");
                let af_rows = rst.rows_affected();
                Ok(af_rows)
            ),
            None => quote!(
                let mut query = sqlx::query(&sql);
                let rst = query.execute(#cot).await;
                if let Err(e) = rst {
                    break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::QueryError, Some(Box::new(e)), None)))
                }
                let rst = rst.expect("Unexpected error");
                let af_rows = rst.rows_affected();
                Ok(af_rows)
            ),
        };
    
        let ret = quote!('rst_block: {
            #declare_rt
            #ret
        });

        Ok(ret)
    }
}
