use quote::{ToTokens, quote};

use crate::SqlClosure;

pub(crate) trait SqlExpand {
    /// get (param_strings, params_idents) at compile time
    fn extra_params(&self, st: &crate::SqlClosure) -> syn::Result<(Vec<String>, Vec<proc_macro2::TokenStream>)> {
        let dto = &st.dto;
        let sql = &st.body;
        let dialect = &st.dialect.to_string();

        // check the template syntax is ok
        ramhorns::Template::new(sql.clone()).unwrap(); 
    
        // get raw sql and all params as both string and ident type at compile time!
        let param_strings = match dto {
            Some(_) => dysql::extract_params(&sql, dysql::SqlDialect::from(dialect.to_owned()))
                .map_err(|_| syn::Error::new(proc_macro2::Span::call_site(), format!("Parse sql error: {} ", sql)))?
                .1,
            None => vec![],
        };
    
        let param_idents: Vec<_> = param_strings.iter().map(|p| {
                let mut rst = proc_macro2::TokenStream::new();
                let arr: Vec<&str> = p.split(".").collect();
                let mut comm_count = arr.len() as i32 - 1;
                for s in arr {
                    let idt = proc_macro2::Ident::new(s, proc_macro2::Span::call_site());
                    rst.extend(idt.to_token_stream());
    
                    if comm_count >= 1 {
                        let punct = proc_macro2::Punct::new('.', proc_macro2::Spacing::Joint);
                        rst.extend(punct.to_token_stream());
                    }
                    comm_count -= 1;
                }
                rst
            }
        ).collect();

        Ok((param_strings, param_idents))
    }
    
    /// declare sql and bind params at runtime
    fn gen_declare_rt(&self, st: &crate::SqlClosure) -> syn::Result<proc_macro2::TokenStream> {
        let dto = &st.dto;
        let body = &st.body;
        let dialect = &st.dialect.to_string();
        let template_id = dysql::md5(body);
        let is_dto_ref = &st.is_dto_ref;
        let dto_ref = if *is_dto_ref { quote!(&) }  else { quote!() }; 
        
        let rst = match dto {
            Some(_) => quote!(
                let sql_tpl = ramhorns::Template::new(#body).unwrap();
                let sql_tpl = match dysql::get_sql_template(#template_id) {
                    Some(tpl) => tpl,
                    None => dysql::put_sql_template(#template_id, #body).expect("Unexpected error when put_sql_template"),
                };
        
                let sql_rendered = sql_tpl.render(#dto_ref #dto);
                let extract_rst = dysql::extract_params(&sql_rendered, dysql::SqlDialect::from(#dialect.to_owned()));
                if let Err(e) = extract_rst {
                    break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::ExtractSqlParamterError, Some(Box::new(e)))))
                }
                let (sql, param_names) = extract_rst.unwrap();
            ),
            None => quote!(
                let sql = #body;
                let param_names: Vec<String> = vec![];
            ),
        };
        
        Ok(rst)
    }

    fn expand(&self, st: &SqlClosure) -> syn::Result<proc_macro2::TokenStream>;
}