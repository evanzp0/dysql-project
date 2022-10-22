use proc_macro::TokenStream;
use quote::quote;

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
    
    let ret = quote!(
        {
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

            (sql, param_values)
        }
    );

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
