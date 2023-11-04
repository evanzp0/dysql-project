use quote::quote;

use crate::sql_expand::SqlExpand;

pub struct FetchAll;

impl SqlExpand for FetchAll {

    fn expand(&self, st: &crate::SqlClosure) -> syn::Result<proc_macro2::TokenStream> {
        let dto = &st.dto;
        let cot = &st.cot;
        let ret_type = &st.ret_type;
    
        let (param_strings, param_idents) = self.extra_params(st)?;

        // declare sql and bind params at runtime
        let declare_rt = self.gen_declare_rt(st, None, false)?;

        let ret = quote!(
            let mut param_values: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
    
            #declare_rt

            for i in 0..param_names.len() {
                #(
                    if param_names[i] == #param_strings {
                        param_values.push(&#dto.#param_idents);
                    }
                )*
            }

            let stmt = #cot.prepare(&sql).await;
            if let Err(e) = stmt {
                break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::PrepareStamentError, Some(Box::new(e)), None)))
            }
            let stmt = stmt.expect("Unexpected error");
    
            let params = param_values.into_iter();
            let params = params.as_slice();
    
            let rows = #cot.query(&stmt, &params).await;
            if let Err(e) = rows {
                break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::QueryError, Some(Box::new(e)), None)))
            }
            let rows = rows.expect("Unexpected error");
    
            let rst = rows
                .iter()
                .map(|row| #ret_type::from_row_ref(row).expect("query unexpected error"))
                .collect::<Vec<#ret_type>>();
    
            Ok(rst)
        );
    
        let ret = quote!('rst_block: {
            #ret
        });

        Ok(ret)
    }
}