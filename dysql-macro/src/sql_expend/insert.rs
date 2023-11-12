use dysql_core::SqlDialect;
use quote::quote;

use crate::{sql_expand::SqlExpand, gen_type_path};

pub struct Insert;

impl SqlExpand for Insert {

    fn expand(&self, st: &crate::DySqlFragmentContext) -> syn::Result<proc_macro2::TokenStream> {
        let dto = &st.dto;
        let cot = &st.cot;
        let dialect: SqlDialect = st.dialect.to_string().into();
    
        // gen return type fro postgres
        let i64_path = Some(gen_type_path("i64"));
        let ret_type = match &st.ret_type {
            Some(_) => &st.ret_type,
            None => &i64_path,
        };

        let cot = super::gen_cot_quote(st, cot);
        let (param_strings, param_idents) = self.extra_params(st)?;

        // declare sql and bind params at runtime
        let declare_rt = self.gen_named_sql_declare(st, None, false)?;

        let ret = match dto {
            Some(_) => {
                match dialect {
                    SqlDialect::postgres => quote!(
                        let mut query = sqlx::query_scalar::<_, #ret_type>(&sql);
                        for i in 0..param_names.len() {
                            #(
                                if param_names[i] == #param_strings {
                                    query = query.bind(&#dto.#param_idents);
                                }
                            )*
                        }
                
                        let insert_id = query.fetch_one(#cot).await;
                        if let Err(e) = insert_id {
                            break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::QueryError, Some(Box::new(e)), None)))
                        }
                        let insert_id = insert_id.expect("Unexpected error");
                        Ok(insert_id)
                    ),
                    SqlDialect::mysql => quote!(
                        let mut query = sqlx::query(&sql);
                        for i in 0..param_names.len() {
                            #(
                                if param_names[i] == #param_strings {
                                    query = query.bind(&#dto.#param_idents);
                                }
                            )*
                        }
            
                        let _rst = query.execute(#cot).await;
                        let insert_id = sqlx::query_as::<_, (u64,)>("SELECT LAST_INSERT_ID();")
                            .fetch_one(#cot)
                            .await;
                        if let Err(e) = insert_id {
                            break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::QueryError, Some(Box::new(e)), None)))
                        }
                        let insert_id = insert_id.expect("Unexpected error").0;
                        Ok(insert_id)
                    ),
                    SqlDialect::sqlite => quote!(
                        let mut query = sqlx::query(&sql);
                        for i in 0..param_names.len() {
                            #(
                                if param_names[i] == #param_strings {
                                    query = query.bind(&#dto.#param_idents);
                                }
                            )*
                        }
            
                        let _rst = query.execute(#cot).await;
                        let insert_id = sqlx::query_as::<_, (i32,)>("SELECT last_insert_rowid();")
                            .fetch_one(#cot)
                            .await;
                        if let Err(e) = insert_id {
                            break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::QueryError, Some(Box::new(e)), None)))
                        }
                        let insert_id = insert_id.expect("Unexpected error").0;
                        Ok(insert_id)
                    ),
                }
            },
            None => match dialect {
                SqlDialect::postgres => quote!(
                    let mut query = sqlx::query_scalar::<_, #ret_type>(&sql);
                    let insert_id = query.fetch_one(#cot).await;
                    if let Err(e) = insert_id {
                        break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::QueryError, Some(Box::new(e)), None)))
                    }
                    let insert_id = insert_id.expect("Unexpected error");
                    Ok(insert_id)
                ),
                SqlDialect::mysql => quote!(
                    let mut query = sqlx::query(&sql);
                    let _rst = query.execute(#cot).await;
                    let insert_id = sqlx::query_as::<_, (u64,)>("SELECT LAST_INSERT_ID();")
                        .fetch_one(#cot)
                        .await;
                    if let Err(e) = insert_id {
                        break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::QueryError, Some(Box::new(e)), None)))
                    }
                    let insert_id = insert_id.expect("Unexpected error").0;
                    Ok(insert_id)
                ),
                SqlDialect::sqlite => quote!(
                    let mut query = sqlx::query(&sql);
                    let _rst = query.execute(#cot).await;
                    let insert_id = sqlx::query_as::<_, (i32,)>("SELECT last_insert_rowid();")
                        .fetch_one(#cot)
                        .await;
                    if let Err(e) = insert_id {
                        break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::QueryError, Some(Box::new(e)), None)))
                    }
                    let insert_id = insert_id.expect("Unexpected error").0;
                    Ok(insert_id)
                ),
            },
        };
    
        let ret = quote!('rst_block: {
            #declare_rt
            #ret
        });

        Ok(ret)
    }
}