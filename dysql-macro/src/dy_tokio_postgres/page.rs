use quote::quote;

use crate::sql_expand::SqlExpand;

pub struct Page;

impl SqlExpand for Page {

    fn expand(&self, st: &crate::SqlClosure) -> syn::Result<proc_macro2::TokenStream> {
        let dto = &st.dto;
        let cot = &st.cot;
        let ret_type = &st.ret_type;
    
        let (param_strings, param_idents) = self.extra_params(st)?;

        // count query ----------------------

        // declare sql and bind params at runtime
        let count_sql = format!("SELECT count(*) FROM ({}) as _tmp", &st.body);
        let declare_rt = self.gen_declare_rt(st, Some(&count_sql))?;

        let rst_count = quote!(
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
    
            let params = param_values.clone().into_iter();
            let params = params.as_slice();
    
            let row = #cot.query_one(&stmt, &params).await;
            if let Err(e) = row {
                break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::QueryError, Some(Box::new(e)))))
            }
            let row = row.expect("Unexpected error");
            let count: i64 = row.get(0);

            #dto.init(count as u64);
        );
    
        // page query ----------------------

        // declare sql and bind params at runtime
        let mut page_sql = st.body.to_owned();
        page_sql.push_str(" limit {{page_size}} offset {{start}}");
        let declare_rt = self.gen_declare_rt(st, Some(&page_sql))?;

        let rst_page = quote!(
            #declare_rt

            let stmt = #cot.prepare(&sql).await;
            if let Err(e) = stmt {
                break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::PrepareStamentError, Some(Box::new(e)))))
            }
            let stmt = stmt.expect("Unexpected error");
    
            let params = param_values.into_iter();
            let params = params.as_slice();
    
            let rows = #cot.query(&stmt, &params).await;
            if let Err(e) = rows {
                break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::QueryError, Some(Box::new(e)))))
            }
            let rows = rows.expect("Unexpected error");
    
            let rst = rows
                .iter()
                .map(|row| #ret_type::from_row_ref(row).expect("query unexpected error"))
                .collect::<Vec<#ret_type>>();
    
            let pg_data = dysql::Pagination::from_dto(&#dto, rst);
            Ok(pg_data)
        );

        let ret = quote!('rst_block: {
            #rst_count
            #rst_page
        });

        Ok(ret)
    }
}