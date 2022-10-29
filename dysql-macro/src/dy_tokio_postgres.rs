//! tokio-postegres feature implement
use dysql::QueryType;
use quote::quote;

use crate::{SqlClosure, gen_path};

pub (crate) fn expand(st: &SqlClosure, query_type: QueryType) -> syn::Result<proc_macro2::TokenStream> {
    let dto = &st.dto;
    let is_dto_ref = &st.is_dto_ref;
    let body = &st.body;
    let dialect = &st.dialect.to_string();
    let template_id = dysql::md5(body);
    
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

    // gen sql render and bind params statement
    let sql_bind_params_ts = match dto {
        Some(dto) => quote!(
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

            for i in 0..param_names.len() {
                #(
                    if param_names[i] == #param_strings {
                        param_values.push(&#dto.#param_idents);
                    }
                )*
            }
        ),
        None => quote!(let sql = #body;),
    };

    // gen query statement
    let rst = match query_type {
        QueryType::FetchAll => expand_fetch_all(st, sql_bind_params_ts),
        QueryType::FetchOne => expand_fetch_one(st, sql_bind_params_ts),
        QueryType::FetchScalar => expand_fetch_scalar(st, sql_bind_params_ts),
        QueryType::Execute => expand_execute(st, sql_bind_params_ts),
        QueryType::Insert => expand_fetch_insert(st, sql_bind_params_ts),
    };

    let ret = quote!('rst_block: {
        #rst
    });

    Ok(ret)
}

fn expand_fetch_all(st: &SqlClosure, sql_bind_params_ts: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let cot = &st.cot;
    let ret_type = &st.ret_type;

    let ret = quote!(
        let mut param_values: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();

        #sql_bind_params_ts

        let stmt = #cot.prepare(&sql).await;
        if let Err(e) = stmt {
            break 'rst_block Err(Box::new(dysql::DySqlError::new(&e.to_string(), Some(Box::new(e)))));
        }
        let stmt = stmt.expect("Unexpected error");

        let params = param_values.into_iter();
        let params = params.as_slice();

        let rows = #cot.query(&stmt, &params).await;
        if let Err(e) = rows {
            break 'rst_block Err(Box::new(dysql::DySqlError::new(&e.to_string(), Some(Box::new(e)))));
        }
        let rows = rows.expect("Unexpected error");

        let rst = rows
            .iter()
            .map(|row| #ret_type::from_row_ref(row).expect("query unexpected error"))
            .collect::<Vec<#ret_type>>();

        Ok(rst)
    );

    ret
}

fn expand_fetch_one(st: &SqlClosure, sql_bind_params_ts: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let cot = &st.cot;
    let ret_type = &st.ret_type;

    let ret = quote!(
        let mut param_values: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();

        #sql_bind_params_ts

        let stmt = #cot.prepare(&sql).await;
        if let Err(e) = stmt {
            break 'rst_block Err(Box::new(dysql::DySqlError::new(&e.to_string(), Some(Box::new(e)))));
        }
        let stmt = stmt.expect("Unexpected error");

        let params = param_values.into_iter();
        let params = params.as_slice();

        let row = #cot.query_one(&stmt, &params).await;
        if let Err(e) = row {
            break 'rst_block Err(Box::new(dysql::DySqlError::new(&e.to_string(), Some(Box::new(e)))));
        }
        let row = row.expect("Unexpected error");

        let rst = #ret_type::from_row(row);
        if let Err(e) = rst {
            break 'rst_block Err(Box::new(dysql::DySqlError::new(&e.to_string(), Some(Box::new(e)))));
        }
        let rst = rst.expect("Unexpected error");

        Ok(rst)
    );

    ret
}

fn expand_fetch_scalar(st: &SqlClosure, sql_bind_params_ts: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let cot = &st.cot;
    let ret_type = &st.ret_type;

    let ret = quote!(
        let mut param_values: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();

        #sql_bind_params_ts

        let stmt = #cot.prepare(&sql).await;
        if let Err(e) = stmt {
            break 'rst_block Err(Box::new(dysql::DySqlError::new(&e.to_string(), Some(Box::new(e)))));
        }
        let stmt = stmt.expect("Unexpected error");

        let params = param_values.into_iter();
        let params = params.as_slice();

        let row = #cot.query_one(&stmt, &params).await;
        if let Err(e) = row {
            break 'rst_block Err(Box::new(dysql::DySqlError::new(&e.to_string(), Some(Box::new(e)))));
        }
        let row = row.expect("Unexpected error");

        let rst: #ret_type = row.get(0);

        Ok(rst)
    );

    ret
}

fn expand_execute(st: &SqlClosure, sql_bind_params_ts: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let cot = &st.cot;

    let ret = quote!(
        let mut param_values: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();

        #sql_bind_params_ts

        let stmt = #cot.prepare(&sql).await;
        if let Err(e) = stmt {
            break 'rst_block Err(Box::new(dysql::DySqlError::new(&e.to_string(), Some(Box::new(e)))));
        }
        let stmt = stmt.expect("Unexpected error");

        let params = param_values.into_iter();
        let params = params.as_slice();

        let affect_count = #cot.execute(&stmt, &params).await;
        if let Err(e) = affect_count {
            break 'rst_block Err(Box::new(dysql::DySqlError::new(&e.to_string(), Some(Box::new(e)))));
        }
        let affectrst_count = affect_count.expect("Unexpected error");

        let rst = affectrst_count;

        Ok(rst)
    );

    ret
}

fn expand_fetch_insert(st: &SqlClosure, sql_bind_params_ts: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let cot = &st.cot;
    let i64_path = Some(gen_path("i64"));
    let ret_type = match &st.ret_type {
        Some(_) => &st.ret_type,
        None => &i64_path,
    };

    let ret = quote!(
        let mut param_values: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();

        #sql_bind_params_ts

        let stmt = #cot.prepare(&sql).await;
        if let Err(e) = stmt {
            break 'rst_block Err(Box::new(dysql::DySqlError::new(&e.to_string(), Some(Box::new(e)))));
        }
        let stmt = stmt.expect("Unexpected error");

        let params = param_values.into_iter();
        let params = params.as_slice();

        let row = #cot.query_one(&stmt, &params).await;
        if let Err(e) = row {
            break 'rst_block Err(Box::new(dysql::DySqlError::new(&e.to_string(), Some(Box::new(e)))));
        }
        let row = row.expect("Unexpected error");
        let rst: #ret_type = row.get(0);

        Ok(rst)
    );

    ret
}