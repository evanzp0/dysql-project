//! sqlx feature implement
use dysql::{QueryType, SqlDialect};
use quote::quote;

use crate::{SqlClosure, gen_path};

pub (crate) fn expand(st: &SqlClosure, query_type: QueryType) -> syn::Result<proc_macro2::TokenStream> {
    let dto = &st.dto;
    let body = &st.body;
    let dialect = &st.dialect.to_string();
    let template_id = dysql::md5(body);
    let is_dto_ref = &st.is_dto_ref;
    let dto_ref = if *is_dto_ref { quote!(&) }  else { quote!() }; 
    
    // check the template syntax is ok
    ramhorns::Template::new(body.clone()).unwrap(); 

    // get raw sql and all params as both string and ident type at compile time!
    let param_strings = match dto {
        Some(_) => dysql::extract_params(&body, dysql::SqlDialect::from(dialect.to_owned()))
            .map_err(|_| syn::Error::new(proc_macro2::Span::call_site(), format!("Parse sql error: {} ", body)))?
            .1,
        None => vec![],
    };
    let param_idents: Vec<_> = param_strings.iter().map( |p| proc_macro2::Ident::new(p, proc_macro2::Span::call_site()) ).collect();
    
    let expend_query_inner = match query_type {
        QueryType::FetchAll => expand_fetch_all(st, &param_strings, &param_idents),
        QueryType::FetchOne => expand_fetch_one(st, &param_strings, &param_idents),
        QueryType::FetchScalar => expand_fetch_scalar(st, &param_strings, &param_idents),
        QueryType::Execute => expand_execute(st, &param_strings, &param_idents),
        QueryType::Insert => expand_insert(st, &param_strings, &param_idents)?,
    };

    // gen sql render statement
    let expend_sql_inner = match dto {
        Some(_) => quote!(
            let sql_tpl = ramhorns::Template::new(#body).unwrap();
            let sql_tpl = match dysql::get_sql_template(#template_id) {
                Some(tpl) => tpl,
                None => dysql::put_sql_template(#template_id, #body).expect("Unexpected error when put_sql_template"),
            };
    
            let sql_rendered = unsafe{(*sql_tpl).render(#dto_ref #dto)};
            let extract_rst = dysql::extract_params(&sql_rendered, dysql::SqlDialect::from(#dialect.to_owned()));
            if let Err(e) = extract_rst {
                break 'rst_block Err(Box::new(dysql::DySqlError::new(&e.to_string(), Some(e))));
            }
            let (sql, param_names) = extract_rst.unwrap();
        ),
        None => quote!(
            let sql = #body;
            let param_names: Vec<String> = vec![];
        ),
    };

    let ret = quote!(
        'rst_block: {
            #expend_sql_inner
            #expend_query_inner
        }
    );

    Ok(ret)
}

pub (crate) fn expand_fetch_all(st: &SqlClosure, param_strings: &Vec<String>, param_idents: &Vec<proc_macro2::Ident>) -> proc_macro2::TokenStream {
    let cot = &st.cot;
    let ret_type = &st.ret_type;
    let dto = &st.dto;

    let cot_ref = if st.is_cot_ref_mut {
        quote!(&mut )
    } else if st.is_cot_ref {
        quote!(&)
    } else {
        quote!()
    };

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

            let rst = query.fetch_all(#cot_ref #cot).await;
            if let Err(e) = rst {
                break 'rst_block Err(Box::new(dysql::DySqlError::new(&e.to_string(), Some(Box::new(e)))));
            }
            let rst = rst.expect("Unexpected error");
            Ok(rst)
        ),
        None => quote!(
            let mut query = sqlx::query_as::<_, #ret_type>(&sql);
            let rst = query.fetch_all(#cot_ref #cot).await;
            if let Err(e) = rst {
                break 'rst_block Err(Box::new(dysql::DySqlError::new(&e.to_string(), Some(Box::new(e)))));
            }
            let rst = rst.expect("Unexpected error");
            Ok(rst)
        ),
    };

    ret
}

fn expand_fetch_one(st: &SqlClosure, param_strings: &Vec<String>, param_idents: &Vec<proc_macro2::Ident>) -> proc_macro2::TokenStream {
    let cot = &st.cot;
    let ret_type = &st.ret_type;
    let dto = &st.dto;

    let cot_ref = if st.is_cot_ref_mut {
        quote!(&mut )
    } else if st.is_cot_ref {
        quote!(&)
    } else {
        quote!()
    };

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
    
            let rst = query.fetch_one(#cot_ref #cot).await;
            if let Err(e) = rst {
                break 'rst_block Err(Box::new(dysql::DySqlError::new(&e.to_string(), Some(Box::new(e)))));
            }
            let rst = rst.expect("Unexpected error");
            Ok(rst)
        ),
        None => quote!(
            let mut query = sqlx::query_as::<_, #ret_type>(&sql);
            let rst = query.fetch_one(#cot_ref #cot).await;
            if let Err(e) = rst {
                break 'rst_block Err(Box::new(dysql::DySqlError::new(&e.to_string(), Some(Box::new(e)))));
            }
            let rst = rst.expect("Unexpected error");
            Ok(rst)
        ),
    };

    ret
}

fn expand_fetch_scalar(st: &SqlClosure, param_strings: &Vec<String>, param_idents: &Vec<proc_macro2::Ident>) -> proc_macro2::TokenStream {
    let cot = &st.cot;
    let ret_type = &st.ret_type;
    let dto = &st.dto;

    let cot_ref = if st.is_cot_ref_mut {
        quote!(&mut )
    } else if st.is_cot_ref {
        quote!(&)
    } else {
        quote!()
    };

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
                break 'rst_block Err(Box::new(dysql::DySqlError::new(&e.to_string(), Some(Box::new(e)))));
            }
            let rst = rst.expect("Unexpected error");
            Ok(rst)
        ),
        None => quote!(
            let mut query = sqlx::query_scalar::<_, #ret_type>(&sql);
            let rst = query.fetch_one(#cot_ref #cot).await;
            if let Err(e) = rst {
                break 'rst_block Err(Box::new(dysql::DySqlError::new(&e.to_string(), Some(Box::new(e)))));
            }
            let rst = rst.expect("Unexpected error");
            Ok(rst)
        ),
    };

    ret
}

fn expand_execute(st: &SqlClosure, param_strings: &Vec<String>, param_idents: &Vec<proc_macro2::Ident>) -> proc_macro2::TokenStream {
    let cot = &st.cot;
    let dto = &st.dto;

    let cot_ref = if st.is_cot_ref_mut {
        quote!(&mut )
    } else if st.is_cot_ref {
        quote!(&)
    } else {
        quote!()
    };

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
    
            let rst = query.execute(#cot_ref #cot).await;
            if let Err(e) = rst {
                break 'rst_block Err(Box::new(dysql::DySqlError::new(&e.to_string(), Some(Box::new(e)))));
            }
            let rst = rst.expect("Unexpected error");
            let af_rows = rst.rows_affected();
            Ok(af_rows)
        ),
        None => quote!(
            let mut query = sqlx::query(&sql);
            let rst = query.execute(#cot_ref #cot).await;
            if let Err(e) = rst {
                break 'rst_block Err(Box::new(dysql::DySqlError::new(&e.to_string(), Some(Box::new(e)))));
            }
            let rst = rst.expect("Unexpected error");
            let af_rows = rst.rows_affected();
            Ok(af_rows)
        ),
    };

    ret
}

fn expand_insert(st: &SqlClosure, param_strings: &Vec<String>, param_idents: &Vec<proc_macro2::Ident>) -> syn::Result<proc_macro2::TokenStream> {
    let cot = &st.cot;
    let dto = &st.dto;
    let dialect: SqlDialect = st.dialect.to_string().into();
    
    let cot_ref = if st.is_cot_ref_mut {
        quote!(&mut )
    } else if st.is_cot_ref {
        quote!(&)
    } else {
        quote!()
    };

    // gen return type fro postgres
    let i64_path = Some(gen_path("i64"));
    let ret_type = match &st.ret_type {
        Some(_) => &st.ret_type,
        None => &i64_path,
    };

    // build return token stream
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
            
                    let insert_id = query.fetch_one(#cot_ref #cot).await;
                    if let Err(e) = insert_id {
                        break 'rst_block Err(Box::new(dysql::DySqlError::new(&e.to_string(), Some(Box::new(e)))));
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
        
                    let _rst = query.execute(&mut #cot).await?;
                    let insert_id = sqlx::query_as::<_, (u64,)>("SELECT LAST_INSERT_ID();")
                        .fetch_one(#cot_ref #cot)
                        .await;
                    if let Err(e) = insert_id {
                        break 'rst_block Err(Box::new(dysql::DySqlError::new(&e.to_string(), Some(Box::new(e)))));
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
        
                    let _rst = query.execute(&mut #cot).await?;
                    let insert_id = sqlx::query_as::<_, (i32,)>("SELECT last_insert_rowid();")
                        .fetch_one(#cot_ref #cot)
                        .await;
                    if let Err(e) = insert_id {
                        break 'rst_block Err(Box::new(dysql::DySqlError::new(&e.to_string(), Some(Box::new(e)))));
                    }
                    let insert_id = insert_id.expect("Unexpected error").0;
                    Ok(insert_id)
                ),
            }
        },
        None => match dialect {
            SqlDialect::postgres => quote!(
                let mut query = sqlx::query_scalar::<_, #ret_type>(&sql);
                let insert_id = query.fetch_one(#cot_ref #cot).await;
                if let Err(e) = insert_id {
                    break 'rst_block Err(Box::new(dysql::DySqlError::new(&e.to_string(), Some(Box::new(e)))));
                }
                let insert_id = insert_id.expect("Unexpected error");
                Ok(insert_id)
            ),
            SqlDialect::mysql => quote!(
                let mut query = sqlx::query(&sql);
                let _rst = query.execute(&mut #cot).await?;
                let insert_id = sqlx::query_as::<_, (u64,)>("SELECT LAST_INSERT_ID();")
                    .fetch_one(#cot_ref #cot)
                    .await;
                if let Err(e) = insert_id {
                    break 'rst_block Err(Box::new(dysql::DySqlError::new(&e.to_string(), Some(Box::new(e)))));
                }
                let insert_id = insert_id.expect("Unexpected error").0;
                Ok(insert_id)
            ),
            SqlDialect::sqlite => quote!(
                let mut query = sqlx::query(&sql);
                let _rst = query.execute(&mut #cot).await?;
                let insert_id = sqlx::query_as::<_, (i32,)>("SELECT last_insert_rowid();")
                    .fetch_one(#cot_ref #cot)
                    .await;
                if let Err(e) = insert_id {
                    break 'rst_block Err(Box::new(dysql::DySqlError::new(&e.to_string(), Some(Box::new(e)))));
                }
                let insert_id = insert_id.expect("Unexpected error").0;
                Ok(insert_id)
            ),
        },
    };
    
    Ok(ret)
}

