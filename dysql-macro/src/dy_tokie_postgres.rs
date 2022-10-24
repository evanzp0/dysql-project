use dysql::QueryType;
use quote::quote;

use crate::{SqlClosure, gen_path};

pub (crate) fn expand(st: &SqlClosure, query_type: QueryType) -> syn::Result<proc_macro2::TokenStream> {
    let dto = &st.dto;
    let body = &st.body;
    let cot = &st.cot;
    let dialect = &st.dialect.to_string();
    let template_id = dysql::md5(body);
    
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
    let expend_sql_bind_params_inner = match dto {
        Some(dto) => quote!(
            let sql_tpl = ramhorns::Template::new(#body).unwrap();
            let sql_tpl = match dysql::get_sql_template(#template_id) {
                Some(tpl) => tpl,
                None => dysql::put_sql_template(#template_id, #body).expect("Unexpected error when put_sql_template"),
            };
    
            let sql_rendered = unsafe{(*sql_tpl).render(&#dto)};
            let extract_rst = dysql::extract_params(&sql_rendered, dysql::SqlDialect::from(#dialect.to_owned()))?;
            let (sql, param_names) = extract_rst;

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
    let expend_query_inner = match query_type {
        QueryType::FetchAll => expand_fetch_all(st),
        QueryType::FetchOne => expand_fetch_one(st),
        QueryType::FetchScalar => expand_fetch_scalar(st),
        QueryType::Execute => expand_execute(st),
        QueryType::Insert => expand_fetch_insert(st),
    };

    let ret = quote!(
        {
            let mut param_values: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();

            #expend_sql_bind_params_inner

            let stmt = #cot.prepare(&sql).await?;
            let params = param_values.into_iter();
            let params = params.as_slice();
            
            #expend_query_inner
        }
    );

    Ok(ret)
}

fn expand_fetch_all(st: &SqlClosure) -> proc_macro2::TokenStream {
    let cot = &st.cot;
    let ret_type = &st.ret_type;

    let ret = quote!(
        let rows = #cot.query(&stmt, &params).await?;
        let rst = rows
            .iter()
            .map(|row| #ret_type::from_row_ref(row).expect("query unexpected error"))
            .collect::<Vec<#ret_type>>();

        rst
    );

    ret
}

fn expand_fetch_one(st: &SqlClosure) -> proc_macro2::TokenStream {
    let cot = &st.cot;
    let ret_type = &st.ret_type;

    let ret = quote!(
        let row = #cot.query_one(&stmt, &params).await?;
        let rst = #ret_type::from_row(row)?;
        rst
    );

    ret
}

fn expand_fetch_scalar(st: &SqlClosure) -> proc_macro2::TokenStream {
    let cot = &st.cot;
    let ret_type = &st.ret_type;

    let ret = quote!(
        let row = #cot.query_one(&stmt, &params).await?;
        let rst: #ret_type = row.get(0);

        rst
    );

    ret
}

fn expand_execute(st: &SqlClosure) -> proc_macro2::TokenStream {
    let cot = &st.cot;

    let ret = quote!(
        let affect_count = #cot.execute(&stmt, &params).await?;
        affect_count
    );

    ret
}

fn expand_fetch_insert(st: &SqlClosure) -> proc_macro2::TokenStream {
    let cot = &st.cot;
    let i64_path = Some(gen_path("i64"));
    let ret_type = match &st.ret_type {
        Some(_) => &st.ret_type,
        None => &i64_path,
    };

    let ret = quote!(
        let row = #cot.query_one(&stmt, &params).await?;
        let rst: #ret_type = row.get(0);

        rst
    );

    ret
}