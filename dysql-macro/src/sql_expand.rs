use dysql_core::{extract_params, SqlDialect, save_sql_template, hash_str};
use dysql_tpl::Template;
use quote::{ToTokens, quote};

use crate::{DySqlFragmentContext, sqlx_fragment::gen_dto_quote};

pub(crate) trait SqlExpand {
    /// get (param_strings, params_idents) at compile time
    /// 在编译时获取 sql body 中的 :named_parameter 
    fn extra_params(&self, st: &crate::DySqlFragmentContext) -> syn::Result<(Vec<String>, Vec<proc_macro2::TokenStream>)> {
        let dto = &st.dto;
        let sql = &st.body;
        let dialect = &st.dialect.to_string();

        // check the template syntax is ok
        dysql_tpl::Template::new(sql).unwrap(); 
    
        // get raw sql and all params as both string and ident type at compile time!
        let param_strings = match dto {
            Some(_) => extract_params(&sql, SqlDialect::from(dialect.to_owned()))
                .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), e.0))?
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
    
    /// 生成运行时的 sql 和 param_names 两个变量的声明语句
    /// 
    /// st: 在编译时生成的包含 sql 的结构体;\
    /// page_sql: 如果有值，它表示分页查询时在 st.body 基础上添加的 count sql 和 order sql;
    fn gen_declare_rt(&self, st: &crate::DySqlFragmentContext, page_sql: Option<&str>, is_page_count: bool) -> syn::Result<proc_macro2::TokenStream> {
        let dto_ident = &st.dto;
        
        // 如果不是分页查询，则使用 st.body
        let body = if let Some(bd) = page_sql {
            bd
        } else {
            &st.body
        };

        let dialect = &st.dialect.to_string();

        // 根据 sql body 生成唯一 hash 标识
        let template_id = hash_str(body);
        
        let source_file = if let Some(path) = st.source_file.to_str() {
            path
        } else {
            Err(syn::Error::new(proc_macro2::Span::call_site(), format!("source_file path can not convert to string: {:?}", st.source_file)))?
        };
        
        match std::env::var("DYSQL_PESIST_SQL") {
            Ok(val) if val.to_ascii_uppercase() == "TRUE" => {
                // 持久化 sql
                let sql_name = st.sql_name
                    .clone()
                    .map(|val|                    
                        if is_page_count {
                            "count_".to_owned() + &val
                        } else {
                            val.to_owned()
                        }
                    );
                save_sql_template(source_file, template_id, body, sql_name).unwrap();
            },
            _ => (),
        }
        
        let template = Template::new(body).expect("error: generate template from sql failed");
        let serd_template = template.serialize();

        let rst = match dto_ident {
            Some(dto_ident) => {
                let dto = gen_dto_quote(st, dto_ident);
                
                quote!(
                    let sql_tpl = match dysql::get_sql_template(#template_id) {
                        Some(tpl) => tpl,
                        None => {
                            let serd_template =  [#(#serd_template,)*];
                            dysql::put_sql_template(#template_id, &serd_template).expect("Unexpected error when put_sql_template")
                        },
                    };
            
                    let sql_rendered = sql_tpl.render(#dto);
                    let sql_rendered = dysql::SqlNodeLinkList::new(&sql_rendered).trim().to_string();
                    // println!("!!! {}", sql_rendered);
                    let extract_rst = dysql::extract_params(&sql_rendered, dysql::SqlDialect::from(#dialect.to_owned()));
                    if let Err(e) = extract_rst {
                        break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::ExtractSqlParamterError, Some(Box::new(e)), None)))
                    }
                    let (sql, param_names) = extract_rst.unwrap();
                )
            },
            // 没有 dto 则 sql 参数绑定列表为空
            None => quote!(
                let sql_tpl = match dysql::get_sql_template(#template_id) {
                    Some(tpl) => tpl,
                    None => {
                        let serd_template =  [#(#serd_template,)*];
                        dysql::put_sql_template(#template_id, &serd_template).expect("Unexpected error when put_sql_template")
                    },
                };
                let sql = sql_tpl.source();
                let param_names: Vec<String> = vec![];
            ),
        };

        Ok(rst)
    }

    fn expand(&self, st: &DySqlFragmentContext) -> syn::Result<proc_macro2::TokenStream>;
}