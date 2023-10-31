use dysql_core::{extract_params, SqlDialect, md5};
use quote::{ToTokens, quote};

use crate::SqlClosure;

pub(crate) trait SqlExpand {
    /// get (param_strings, params_idents) at compile time
    /// 在编译时获取 sql body 中的 :named_parameter 
    fn extra_params(&self, st: &crate::SqlClosure) -> syn::Result<(Vec<String>, Vec<proc_macro2::TokenStream>)> {
        let dto = &st.dto;
        let sql = &st.body;
        let dialect = &st.dialect.to_string();

        // check the template syntax is ok
        dysql_tpl::Template::new(sql.clone()).unwrap(); 
    
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
    /// sql: 如果有值，它表示分页查询时在 st.body 基础上添加的 count sql 和 order sql;
    fn gen_declare_rt(&self, st: &crate::SqlClosure, sql: Option<&str>) -> syn::Result<proc_macro2::TokenStream> {
        let dto = &st.dto;
        
        // 如果不是分页查询，则使用 st.body
        let body = if let Some(bd) = sql {
            bd
        } else {
            &st.body
        };

        let dialect = &st.dialect.to_string();

        // 根据 sql body 生成唯一 hash 标识
        let template_id = md5(body);

        let is_dto_ref = &st.is_dto_ref;
        let is_dto_ref_mut = &st.is_dto_ref_mut;
        let dto_ref = if *is_dto_ref { quote!(&) }  else if *is_dto_ref_mut { quote!(&mut) } else { quote!() }; 
        
        let rst = match dto {
            Some(_) => quote!(
                let sql_tpl = match dysql::get_sql_template(#template_id) {
                    Some(tpl) => tpl,
                    None => dysql::put_sql_template(#template_id, #body).expect("Unexpected error when put_sql_template"),
                };
        
                let sql_rendered = sql_tpl.render(#dto_ref #dto);
                let sql_rendered = dysql::SqlNodeLinkList::new(&sql_rendered).trim().to_string();
                // println!("!!! {}", sql_rendered);
                let extract_rst = dysql::extract_params(&sql_rendered, dysql::SqlDialect::from(#dialect.to_owned()));
                if let Err(e) = extract_rst {
                    break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::ExtractSqlParamterError, Some(Box::new(e)), None)))
                }
                let (sql, param_names) = extract_rst.unwrap();
            ),
            // 没有 dto 则 sql 参数绑定列表为空
            None => quote!(
                // todo!, sql 也需要用 dysql::get_sql_template(#template_id) 获取
                let sql = #body;
                let param_names: Vec<String> = vec![];
            ),
        };

        Ok(rst)
    }

    fn expand(&self, st: &SqlClosure) -> syn::Result<proc_macro2::TokenStream>;
}