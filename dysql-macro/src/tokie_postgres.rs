use dysql::QueryType;
use quote::quote;

use crate::SqlClosure;

pub (crate) fn expand(st: &SqlClosure, query_type: QueryType) -> syn::Result<proc_macro2::TokenStream> {
    let dto = &st.dto;
    let body = &st.body;
    let cot = &st.cot;
    let dialect = &st.dialect.to_string();
    let template_id = dysql::md5(body);
    
    // check the template syntax is ok
    ramhorns::Template::new(body.clone()).unwrap(); 

    // get raw sql and all params as both string and ident type at compile time!
    let (tmp_sql, param_strings) = dysql::extract_params(&body, dysql::SqlDialect::from(dialect.to_owned()));
    if tmp_sql == "".to_owned() {
        return Err(syn::Error::new(proc_macro2::Span::call_site(), format!("Parse sql error: {} ", body)))
    }
    let param_idents: Vec<_> = param_strings.iter().map( |p| proc_macro2::Ident::new(p, proc_macro2::Span::call_site()) ).collect();
    
    let expend_inner = match query_type {
        QueryType::FetchAll => expand_fetch_all(st),
        QueryType::FetchOne => expand_fetch_one(st),
        QueryType::FetchScalar => expand_fetch_scalar(st),
        QueryType::Execute => expand_execute(st),
    };

    let ret = quote!(
        {
            let _empty_dto = dysql::EmptyDto{ _empty: 0 };
            let mut param_values: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
            let sql_tpl = ramhorns::Template::new(#body).unwrap();
            let sql_tpl = match dysql::get_sql_template(#template_id) {
                Some(tpl) => tpl,
                None => dysql::put_sql_template(#template_id, #body).expect("Unexpected error when put_sql_template"),
            };
    
            let sql_rendered = unsafe{(*sql_tpl).render(&#dto)};
            let rst = dysql::extract_params(&sql_rendered, dysql::SqlDialect::from(#dialect.to_owned()));
            let (sql, param_names) = rst;

            for i in 0..param_names.len() {
                #(
                    if param_names[i] == #param_strings {
                        param_values.push(&#dto.#param_idents);
                    }
                )*
            }

            let stmt = #cot.prepare(&sql).await?;
            let params = param_values.into_iter();
            let params = params.as_slice();
            
            #expend_inner
        }
    );

    Ok(ret)
}

pub (crate) fn expand_fetch_all(st: &SqlClosure) -> proc_macro2::TokenStream {
    let cot = &st.cot;
    let ret_type = &st.ret_type;

    let ret = quote!(
        let rows = #cot.query(&stmt, params).await?;
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
        let row = #cot.query_one(&stmt, params).await?;
        let rst = #ret_type::from_row(row)?;
        rst
    );

    ret
}

fn expand_fetch_scalar(st: &SqlClosure) -> proc_macro2::TokenStream {
    let cot = &st.cot;
    let ret_type = &st.ret_type;

    let ret = quote!(
        let row = #cot.query_one(&stmt, params).await?;
        let rst: #ret_type = row.get(0);

        rst
    );

    ret
}

fn expand_execute(st: &SqlClosure) -> proc_macro2::TokenStream {
    let cot = &st.cot;

    let ret = quote!(
        let affect_count = #cot.execute(&stmt, params).await?;
        affect_count
    );

    ret
}
