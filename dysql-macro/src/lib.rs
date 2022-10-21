use proc_macro::TokenStream;
use quote::{quote, ToTokens};

#[allow(dead_code)]
#[derive(Debug)]
struct SqlClosure {
    dto: syn::Ident,
    dialect: syn::Ident,
    body: String,
}

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as SqlClosure);

    match expand(&st) {
        Ok(ret) => ret.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

fn expand(st: &SqlClosure) -> syn::Result<proc_macro2::TokenStream> {
    let mut ret = proc_macro2::TokenStream::new();
    let dto = &st.dto;
    let body = &st.body;
    let dialect = &st.dialect.to_string();

    // get raw sql and all params as both string and ident type at compile time!
    let (tmp_sql, param_strings) = dysql::extract_params(&body, dysql::SqlDialect::from(dialect.to_owned()));
    if tmp_sql == "".to_owned() {
        return Err(syn::Error::new(proc_macro2::Span::call_site(), format!("Parse sql error: {} ", body)))
    }
    
    let param_idents: Vec<_> = param_strings.iter().map( |p| proc_macro2::Ident::new(p, proc_macro2::Span::call_site()) ).collect();

    // gen inner expr token stream
    let mut expr = proc_macro2::TokenStream::new();
    let template_id = dysql::md5(body);
    let expr_def = quote!(
        // let sql_tpl = ramhorns::Template::new(#body)?;
        let sql_tpl = match dysql::get_sql_template(#template_id) {
            Some(tpl) => tpl,
            None => dysql::put_sql_template(#template_id, #body).expect("Unexpected error when put_sql_template"),
        };

        let sql_rendered = unsafe{(*sql_tpl).render(&#dto)};
        let rst = dysql::extract_params(&sql_rendered, dysql::SqlDialect::from(#dialect.to_owned()));
        let (sql, param_names) = rst;
        let mut param_values: Vec<&(dyn dysql::ToSql + Sync)> = Vec::new();
    );
    expr.extend(expr_def);

    let expr_for = quote!(
        for i in 0..param_names.len() 
    );
    expr.extend(expr_for);

    let mut expr_block_inner = proc_macro2::TokenStream::new();
    let params = param_strings.iter().zip(param_idents);
    for (param_string, ref param_ident) in params {
        let expr_if = quote!(
            if param_names[i] == #param_string {
                param_values.push(&#dto.#param_ident);
            }
        );
        expr_block_inner.extend(expr_if);
    }
    let expr_for_block = proc_macro2::Group::new(proc_macro2::Delimiter::Brace, expr_block_inner);
    expr.extend(expr_for_block.into_token_stream());

    expr.extend(quote!((sql, param_values)));

    ret.extend(proc_macro2::Group::new(proc_macro2::Delimiter::Brace, expr).into_token_stream());

    // let ret_fn = quote!(
    //     {
    //         fn _dysql_tmp_fn() -> DySqlResult<(String, Vec<&(dyn ToSql + Sync)>)> {
    //             #expr
    //         }
    //         _dysql_tmp_fn()
    //     }
    // );

    Ok(ret)
}

impl syn::parse::Parse for SqlClosure {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // parse closure parameters
        input.parse::<syn::Token!(|)>()?;
        let dto: syn::Ident = input.parse()?;
        input.parse::<syn::Token!(|)>()?;

        // parse closure returning sql dialect
        let dialect: syn::Ident = match input.parse::<syn::Token!(->)>() {
            Ok(_) => input.parse()?,
            Err(_) => syn::Ident::new(&dysql::SqlDialect::postgres.to_string(), input.span()),
        };

        // parse closure sql body
        let body_buf;
        syn::braced!(body_buf in input);
        let body: syn::LitStr = body_buf.parse()?;
        let body = body.value();
        let body:Vec<_> = body.split("\n").map(|f| f.trim()).collect();
        let body = body.join(" ");
        // eprintln!("{:#?}", body);
        Ok(SqlClosure { dto, dialect, body })
    }
}
