use dysql_core::{save_sql_template, hash_str};
use dysql_tpl::Template;
use quote::quote;

use crate::DyClosure;

pub(crate) struct SqlExpand;

impl SqlExpand {

    /// expend fetch_one
    pub fn fetch_one(&self, st: &DyClosure) -> syn::Result<proc_macro2::TokenStream>{
        let dto_ident = &st.dto_info.src;
        let executor_ident = &st.executor_info.src;
        let executor_token = st.executor_info.gen_token();
        let ret_type = &st.ret_type;
        
        // declare named_sql at runtime
        let named_sql_declare = self.gen_named_sql_declare(st)?;

        // 生成 QueryAdapter 对象
        let query_declare = quote!(
            let query = #executor_ident.create_query();
        );

        let execute = match dto_ident {
            Some(_) => quote!(
                query.fetch_one::<_, _, #ret_type>(#executor_token, &named_sql, Some(&#dto_ident)).await 
            ),
            None => quote!(
                query.fetch_one::<_, dysql::EmptyObject, #ret_type>(#executor_token, &named_sql, None).await 
            ),
        };
        
        let ret = quote!('rst_block: {
            use dysql::SqlxExecutorAdatper;
            #named_sql_declare  // let named_sql = ....;
            #query_declare      // let query = executor.create_query(....);
            #execute
        });

        Ok(ret)
    }

    /// expend fetch_all
    pub fn fetch_all(&self, st: &DyClosure) -> syn::Result<proc_macro2::TokenStream>{
        let dto_ident = &st.dto_info.src;
        let executor_ident = &st.executor_info.src;
        let executor_token = st.executor_info.gen_token();
        let ret_type = &st.ret_type;

        // declare named_sql at runtime
        let named_sql_declare = self.gen_named_sql_declare(st)?;

        // 生成 QueryAdapter 对象
        let query_declare = quote!(
            let query = #executor_ident.create_query();
        );

        let execute = match dto_ident {
            Some(_) => quote!(
                query.fetch_all::<_, _, #ret_type>(#executor_token, &named_sql, Some(&#dto_ident)).await 
            ),
            None => quote!(
                query.fetch_all::<_, dysql::EmptyObject, #ret_type>(#executor_token, &named_sql, None).await 
            ),
        };

        let ret = quote!('rst_block: {
            use dysql::SqlxExecutorAdatper;
            #named_sql_declare  // let named_sql = ....;
            #query_declare      // let query = executor.create_query(....);
            #execute
        });

        Ok(ret)
    }

    /// expend fetch_scalar
    pub fn fetch_scalar(&self, st: &DyClosure) -> syn::Result<proc_macro2::TokenStream>{
        let dto_ident = &st.dto_info.src;
        let executor_ident = &st.executor_info.src;
        let executor_token = st.executor_info.gen_token();
        let ret_type = &st.ret_type;

        // declare named_sql at runtime
        let named_sql_declare = self.gen_named_sql_declare(st)?;

        // 生成 QueryAdapter 对象
        let query_declare = quote!(
            let query = #executor_ident.create_query();
        );

        let execute = match dto_ident {
            Some(_) => quote!(
                query.fetch_scalar::<_, _, #ret_type>(#executor_token, &named_sql, Some(&#dto_ident)).await 
            ),
            None => quote!(
                query.fetch_scalar::<_, dysql::EmptyObject, #ret_type>(#executor_token, &named_sql, None).await 
            ),
        };

        let ret = quote!('rst_block: {
            use dysql::SqlxExecutorAdatper;
            #named_sql_declare  // let named_sql = ....;
            #query_declare      // let query = executor.create_query(....);
            #execute
        });

        Ok(ret)
    }

    /// expend execute
    pub fn execute(&self, st: &DyClosure) -> syn::Result<proc_macro2::TokenStream>{
        let dto_ident = &st.dto_info.src;
        let executor_ident = &st.executor_info.src;
        let executor_token = st.executor_info.gen_token();

        // declare named_sql at runtime
        let named_sql_declare = self.gen_named_sql_declare(st)?;

        // 生成 QueryAdapter 对象
        let query_declare = quote!(
            let query = #executor_ident.create_query();
        );

        let execute = match dto_ident {
            Some(_) => quote!(
                query.execute(#executor_token, &named_sql, Some(&#dto_ident)).await
            ),
            None => quote!(
                query.execute::<_, dysql::EmptyObject>(#executor_token, &named_sql, None).await 
            ),
        };

        let ret = quote!('rst_block: {
            use dysql::SqlxExecutorAdatper;
            #named_sql_declare  // let named_sql = ....;
            #query_declare      // let query = executor.create_query(....);
            #execute
        });

        Ok(ret)
    }

    /// expend insert
    pub fn insert(&self, st: &DyClosure) -> syn::Result<proc_macro2::TokenStream>{
        let dto_ident = &st.dto_info.src;
        let executor_ident = &st.executor_info.src;
        let executor_token = st.executor_info.gen_token();
        let ret_type = &st.ret_type;

        // declare named_sql at runtime
        let named_sql_declare = self.gen_named_sql_declare(st)?;

        // 生成 QueryAdapter 对象
        let query_declare = quote!(
            let query = #executor_ident.create_query();
        );

        let execute = match dto_ident {
            Some(_) => quote!(
                query.insert::<_, _, #ret_type>(#executor_token, &named_sql, Some(&#dto_ident)).await 
            ),
            None => quote!(
                query.insert::<_, dysql::EmptyObject, #ret_type>(#executor_token, &named_sql, None).await 
            ),
        };

        let ret = quote!('rst_block: {
            use dysql::SqlxExecutorAdatper;
            #named_sql_declare  // let named_sql = ....;
            #query_declare      // let query = executor.create_query(....);
            #execute
        });

        Ok(ret)
    }

    /// expend page query
    pub fn page(&self, st: &DyClosure) -> syn::Result<proc_macro2::TokenStream>{
        let dto_ident = &st.dto_info.src;
        let executor_ident = &st.executor_info.src;
        let executor_token = st.executor_info.gen_token();
        let ret_type = &st.ret_type;

        // declare named_sql whith template at runtime 
        let named_sql_declare = self.gen_named_sql_declare(st)?;

        // page_dto 通过 QueryAdapter.page() 方法传递，所以这里只要生成没有 dto 的 QueryAdapter 对象就可以了
        let query_declare = quote!(
            let query = #executor_ident.create_query();
        );

        let buf_count_named_sql_declare = quote!(
            let buffer_size = named_sql.len() + 200;
            let mut sql_buf = Vec::<u8>::with_capacity(buffer_size);
    
            // count query
            let count_named_sql = {
                use std::io::Write;
                write!(sql_buf, "SELECT count(*) FROM ({}) as _tmp", named_sql).unwrap();
                std::str::from_utf8(&sql_buf).unwrap()
            };
        );

        let count_exexute = match dto_ident {
            Some(_) => quote!(
                #buf_count_named_sql_declare
                let count_rst = query.fetch_scalar::<_, _, i64>(#executor_token, &count_named_sql, Some(&#dto_ident)).await;
            ),
            None => quote!(
                #buf_count_named_sql_declare
                let count_rst = query.fetch_scalar::<_, dysql::EmptyObject, i64>(#executor_token, &count_named_sql, None).await;
            ),
        };

        let ret = quote!('rst_block: {
            use dysql::SqlxExecutorAdatper;
            #named_sql_declare  // let named_sql = ....;
            #query_declare      // let query = executor.create_query(....);

            #count_exexute
            if let Err(e) = count_rst {
                break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::QueryError, Some(Box::new(e)), None)))
            }
            let count = count_rst.expect("Unexpected error");
            #dto_ident.init(count as u64);

            let page_named_sql = {
                use std::io::Write;
    
                sql_buf.clear();
                
                let sort_fragment = "{{#is_sort}} ORDER BY {{#sort_model}} {{field}} {{sort}}, {{/sort_model}} ![B_DEL(,)] {{/is_sort}} LIMIT {{page_size}} OFFSET {{start}}";
                let template = dysql::Template::new(sort_fragment).expect("unexpected error: generate template from sql failed");
                let sort_fragment = template.render(&#dto_ident);
                let sort_fragment = dysql::SqlNodeLinkList::new(&sort_fragment).trim().to_string();
                
                write!(sql_buf, "{} {} ", named_sql, sort_fragment).unwrap();
                std::str::from_utf8(&sql_buf).unwrap()
            };

            #query_declare
            query.page::<_, _, #ret_type>(#executor_token, &page_named_sql, &#dto_ident).await 
        });

        Ok(ret)
    }

    /// 在编译时生成运行时根据 dto 进行 render 后得到的 named_sql
    /// 
    /// st: 在编译时生成的包含 sql 的结构体;
    fn gen_named_sql_declare(&self, st: &crate::DyClosure) -> syn::Result<proc_macro2::TokenStream> {
        let dto_ident = &st.dto_info.src;

        // 根据 sql body 生成唯一 hash 标识
        let template_id = hash_str(&st.body);
        
        // 根据配置决定是否持久化 sql
        let source_file = if let Some(path) = st.source_file.to_str() {
            path
        } else {
            Err(syn::Error::new(proc_macro2::Span::call_site(), format!("source_file path can not convert to string: {:?}", st.source_file)))?
        };
        
        match std::env::var("DYSQL_PESIST_SQL") {
            Ok(val) if val.to_ascii_uppercase() == "TRUE" => {
                save_sql_template(source_file, template_id, &st.body, st.sql_name.clone()).unwrap();
            },
            _ => (),
        }
        
        // 根据 sql 生成模板
        let template = Template::new(&st.body).expect("error: generate template from sql failed");
        // 将模板序列化，接下来通过 TokenSteam 放在编译后的文件里，可以加快加载速度
        let serd_template = template.serialize();

        // 生成 TokenStream
        let mut rst = quote!(
            // 优先从 cache 中加载 sql 模板，如果 cache 中没有，则直接从序列化的二进制变量中加载并缓存 sql 模板
            let sql_tpl = match dysql::get_sql_template(#template_id) {
                Some(tpl) => tpl,
                None => {
                    let serd_template =  [#(#serd_template,)*];
                    dysql::put_sql_template(#template_id, &serd_template).expect("Unexpected error when put_sql_template")
                },
            };
        );
        if let Some(dto_ident) = dto_ident {
            rst.extend(quote!( let named_sql: String = sql_tpl.render(&#dto_ident); ));
        } else {
            rst.extend(quote!( let named_sql: String = sql_tpl.source().to_owned(); ));
        }
        rst.extend(
            quote!(
                // 格式化 sql 并解析 BDEL 和 FDEL 指令
                let named_sql = dysql::SqlNodeLinkList::new(&named_sql).trim().to_string();
                // println!("!!! {}", named_sql);
            )
        );
        Ok(rst)
    }
}
