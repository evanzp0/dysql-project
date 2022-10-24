use dysql::{QueryType, SqlDialect};
use quote::quote;
use syn::punctuated::Punctuated;

use crate::SqlClosure;

pub (crate) fn expand(st: &SqlClosure, query_type: QueryType) -> syn::Result<proc_macro2::TokenStream> {
    let dto = &st.dto;
    let body = &st.body;
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
        QueryType::FetchAll => expand_fetch_all(st, &param_strings, &param_idents),
        QueryType::FetchOne => expand_fetch_one(st, &param_strings, &param_idents),
        QueryType::FetchScalar => expand_fetch_scalar(st, &param_strings, &param_idents),
        QueryType::Execute => expand_execute(st, &param_strings, &param_idents),
        QueryType::Insert => expand_insert(st, &param_strings, &param_idents)?,
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
            
            #expend_inner
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
    } else {
        quote!(&)
    };

    let ret = quote!(
        let mut query = sqlx::query_as::<_, #ret_type>(&sql);
        for i in 0..param_names.len() {
            #(
                if param_names[i] == #param_strings {
                    query = query.bind(&#dto.#param_idents);
                }
            )*
        }

        let rst = query.fetch_all(#cot_ref #cot).await?;
        rst
    );

    ret
}

fn expand_fetch_one(st: &SqlClosure, param_strings: &Vec<String>, param_idents: &Vec<proc_macro2::Ident>) -> proc_macro2::TokenStream {
    let cot = &st.cot;
    let ret_type = &st.ret_type;
    let dto = &st.dto;

    let cot_ref = if st.is_cot_ref_mut {
        quote!(&mut )
    } else {
        quote!(&)
    };

    let ret = quote!(
        let mut query = sqlx::query_as::<_, #ret_type>(&sql);
        for i in 0..param_names.len() {
            #(
                if param_names[i] == #param_strings {
                    query = query.bind(&#dto.#param_idents);
                }
            )*
        }

        let rst = query.fetch_one(#cot_ref #cot).await?;
        rst
    );

    ret
}

fn expand_fetch_scalar(st: &SqlClosure, param_strings: &Vec<String>, param_idents: &Vec<proc_macro2::Ident>) -> proc_macro2::TokenStream {
    let cot = &st.cot;
    let ret_type = &st.ret_type;
    let dto = &st.dto;

    let cot_ref = if st.is_cot_ref_mut {
        quote!(&mut )
    } else {
        quote!(&)
    };

    let ret = quote!(
        let mut query = sqlx::query_scalar::<_, #ret_type>(&sql);
        for i in 0..param_names.len() {
            #(
                if param_names[i] == #param_strings {
                    query = query.bind(&#dto.#param_idents);
                }
            )*
        }

        let rst = query.fetch_one(#cot_ref #cot).await?;
        rst
    );

    ret
}

fn expand_execute(st: &SqlClosure, param_strings: &Vec<String>, param_idents: &Vec<proc_macro2::Ident>) -> proc_macro2::TokenStream {
    let cot = &st.cot;
    let dto = &st.dto;

    let cot_ref = if st.is_cot_ref_mut {
        quote!(&mut )
    } else {
        quote!(&)
    };

    let ret = quote!(
        let mut query = sqlx::query(&sql);
        for i in 0..param_names.len() {
            #(
                if param_names[i] == #param_strings {
                    query = query.bind(&#dto.#param_idents);
                }
            )*
        }

        let rst = query.execute(#cot_ref #cot).await?;
        let af_rows = rst.rows_affected();
        af_rows
    );

    ret
}

fn expand_insert(st: &SqlClosure, param_strings: &Vec<String>, param_idents: &Vec<proc_macro2::Ident>) -> syn::Result<proc_macro2::TokenStream> {
    let cot = &st.cot;
    let dto = &st.dto;
    let dialect: SqlDialect = st.dialect.to_string().into();
    
    let cot_ref = if st.is_cot_ref_mut {
        quote!(&mut )
    } else {
        quote!(&)
    };

    // gen return type fro postgres
    let ret_type_seg = syn::PathSegment {
        ident: syn::Ident::new("i64", proc_macro2::Span::call_site()),
        arguments: syn::PathArguments::None,
    };
    let mut ret_type_punct: Punctuated<syn::PathSegment, syn::Token![::]> = Punctuated::new();
    ret_type_punct.push_value(ret_type_seg);
    let ret_type_path = Some(syn::Path{ leading_colon: None, segments: ret_type_punct });
    let ret_type = match &st.ret_type {
        Some(_) => &st.ret_type,
        None => &ret_type_path,
    };

    // build return token stream
    let ret = match dialect {
        SqlDialect::postgres => quote!(
            let mut query = sqlx::query_scalar::<_, #ret_type>(&sql);
            for i in 0..param_names.len() {
                #(
                    if param_names[i] == #param_strings {
                        query = query.bind(&#dto.#param_idents);
                    }
                )*
            }
    
            let insert_id = query.fetch_one(#cot_ref #cot).await?;
            insert_id
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
                .await?
                .0;
            insert_id
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
                .await?
                .0;
            insert_id
        ),
    };

    Ok(ret)
}

