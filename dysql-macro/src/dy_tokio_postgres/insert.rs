use quote::quote;

use crate::{sql_expand::SqlExpand, gen_path};

pub struct Insert;

impl SqlExpand for Insert {

    fn expand(&self, st: &crate::SqlClosure) -> syn::Result<proc_macro2::TokenStream> {
        let dto = &st.dto;
        let cot = &st.cot;
        let i64_path = Some(gen_path("i64"));
        let ret_type = match &st.ret_type {
            Some(_) => &st.ret_type,
            None => &i64_path,
        };

        let (param_strings, param_idents) = self.extra_params(st)?;

        // declare sql and bind params at runtime
        let declare_rt = self.gen_declare_rt(st, None)?;

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
                break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::PrepareStamentError, Some(Box::new(e)))))
            }
            let stmt = stmt.expect("Unexpected error");
    
            let params = param_values.into_iter();
            let params = params.as_slice();
    
            let row = #cot.query_one(&stmt, &params).await;
            if let Err(e) = row {
                break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::QueryError, Some(Box::new(e)))))
            }
            let row = row.expect("Unexpected error");
            let rst: #ret_type = row.get(0);
    
            Ok(rst)
        );
    
        let ret = quote!('rst_block: {
            #ret
        });

        Ok(ret)
    }
}