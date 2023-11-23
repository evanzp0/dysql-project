use dysql_core::{save_sql_template, hash_str};
use dysql_tpl::Template;
use quote::quote;

use crate::DyClosure;

pub(crate) struct SqlExpand;

impl SqlExpand {

    /// expend fetch_one
    pub fn fetch_one(&self, st: &DyClosure) -> syn::Result<proc_macro2::TokenStream>{
        let dto_ident = &st.dto_info.src;
        let executor_token = st.executor_info.gen_token();
        let ret_type = &st.ret_type;
        
        // declare named_template at runtime
        let named_template_declare = self.gen_named_template_declare(st)?;

        let dto_token = st.dto_info.gen_token();
        let execute_query = match dto_ident {
            Some(_) => quote!(
                query.fetch_one::<_, _, #ret_type>(#executor_token, named_template, Some(#dto_token)).await 
            ),
            None => quote!(
                query.fetch_one::<_, dysql::EmptyObject, #ret_type>(#executor_token, named_template, None).await 
            ),
        };
        
        let ret = quote!('rst_block: {
            #[cfg(feature = "tokio-postgres")]
            use dysql::TokioPgExecutorAdatper;

            #[cfg(feature="sqlx")]
            use dysql::SqlxExecutorAdatper;

            #named_template_declare  // let named_template = ....;
            let query = (#executor_token).create_query();
            #execute_query
        });

        Ok(ret)
    }

    /// expend fetch_all
    pub fn fetch_all(&self, st: &DyClosure) -> syn::Result<proc_macro2::TokenStream>{
        let dto_ident = &st.dto_info.src;
        let executor_token = st.executor_info.gen_token();
        let ret_type = &st.ret_type;

        // declare named_template at runtime
        let named_template_declare = self.gen_named_template_declare(st)?;

        let dto_token = st.dto_info.gen_token();
        let execute_query = match dto_ident {
            Some(_) => quote!(
                query.fetch_all::<_, _, #ret_type>(#executor_token, named_template, Some(#dto_token)).await 
            ),
            None => quote!(
                query.fetch_all::<_, dysql::EmptyObject, #ret_type>(#executor_token, named_template, None).await 
            ),
        };

        let ret = quote!('rst_block: {
            #[cfg(feature = "tokio-postgres")]
            use dysql::TokioPgExecutorAdatper;

            #[cfg(feature="sqlx")]
            use dysql::SqlxExecutorAdatper;

            #named_template_declare  // let named_sql = ....;
            let query = (#executor_token).create_query();
            #execute_query
        });

        Ok(ret)
    }

    /// expend fetch_scalar
    pub fn fetch_scalar(&self, st: &DyClosure) -> syn::Result<proc_macro2::TokenStream>{
        let dto_ident = &st.dto_info.src;
        let executor_token = st.executor_info.gen_token();
        let ret_type = &st.ret_type;

        // declare named_template at runtime
        let named_template_declare = self.gen_named_template_declare(st)?;

        let dto_token = st.dto_info.gen_token();
        let execute_query = match dto_ident {
            Some(_) => quote!(
                query.fetch_scalar::<_, _, #ret_type>(#executor_token, named_template, Some(#dto_token)).await 
            ),
            None => quote!(
                query.fetch_scalar::<_, dysql::EmptyObject, #ret_type>(#executor_token, named_template, None).await 
            ),
        };

        let ret = quote!('rst_block: {
            #[cfg(feature = "tokio-postgres")]
            use dysql::TokioPgExecutorAdatper;

            #[cfg(feature="sqlx")]
            use dysql::SqlxExecutorAdatper;
            
            #named_template_declare  // let named_sql = ....;
            let query = (#executor_token).create_query();
            #execute_query
        });

        Ok(ret)
    }

    /// expend execute
    pub fn execute(&self, st: &DyClosure) -> syn::Result<proc_macro2::TokenStream>{
        let dto_ident = &st.dto_info.src;
        let executor_token = st.executor_info.gen_token();

        // declare named_template at runtime
        let named_template_declare = self.gen_named_template_declare(st)?;

        let dto_token = st.dto_info.gen_token();
        let execute_query = match dto_ident {
            Some(_) => quote!(
                query.execute(#executor_token, named_template, Some(#dto_token)).await
            ),
            None => quote!(
                query.execute::<_, dysql::EmptyObject>(#executor_token, named_template, None).await 
            ),
        };

        let ret = quote!('rst_block: {
            #[cfg(feature = "tokio-postgres")]
            use dysql::TokioPgExecutorAdatper;

            #[cfg(feature="sqlx")]
            use dysql::SqlxExecutorAdatper;

            #named_template_declare  // let named_sql = ....;
            let query = (#executor_token).create_query();
            #execute_query
        });

        Ok(ret)
    }

    /// expend insert
    pub fn insert(&self, st: &DyClosure) -> syn::Result<proc_macro2::TokenStream>{
        let dto_ident = &st.dto_info.src;
        let executor_token = st.executor_info.gen_token();
        let ret_type = &st.ret_type;

        // declare named_template at runtime
        let named_template_declare = self.gen_named_template_declare(st)?;

        let dto_token = st.dto_info.gen_token();
        let execute_query = match dto_ident {
            Some(_) => quote!(
                let insert_rst = query.insert::<_, _, #ret_type>(#executor_token, named_template, Some(#dto_token)).await;
            ),
            None => quote!(
                let insert_rst = query.insert::<_, dysql::EmptyObject, #ret_type>(#executor_token, named_template, None).await;
            ),
        };

        let ret = quote!('rst_block: {
            #[cfg(feature = "tokio-postgres")]
            use dysql::TokioPgExecutorAdatper;

            #[cfg(feature="sqlx")]
            use dysql::SqlxExecutorAdatper;

            #named_template_declare  // let named_sql = ....;
            let query = (#executor_token).create_query();
            #execute_query
            
            let rst = match insert_rst {
                Ok(Some(insert_id)) => Ok(insert_id),
                Ok(None) => {
                    let query = tran.create_query();
                    query.fetch_insert_id(#executor_token).await
                }
                Err(e) => {
                    break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::QueryError, Some(Box::new(e)), None)));
                }
            };
            let rst = rst.map(|v| v as #ret_type);
            rst
        });

        Ok(ret)
    }

    /// expend page query
    pub fn page(&self, st: &DyClosure) -> syn::Result<proc_macro2::TokenStream>{
        let dto_ident = &st.dto_info.src;
        let executor_token = st.executor_info.gen_token();
        let ret_type = &st.ret_type;
        let dto_token = st.dto_info.gen_token();

        // declare named_template at runtime
        let named_template_declare = self.gen_named_template_declare(st)?;

        // 生成 count 查询的调用
        let execute_count_query = match dto_ident {
            Some(_) => quote!(
                let count_rst = query.page_count::<_, _, i64>(#executor_token, named_template.clone(), Some(&#dto_token)).await;
            ),
            None => quote!(
                let count_rst = query.page_count::<_, dysql::EmptyObject, i64>(#executor_token, named_template.clone(), None).await;
            ),
        };

        let ret = quote!('rst_block: {
            #[cfg(feature = "tokio-postgres")]
            use dysql::TokioPgExecutorAdatper;

            #[cfg(feature="sqlx")]
            use dysql::SqlxExecutorAdatper;

            #named_template_declare  // let named_sql = ....;
            let query = (#executor_token).create_query();

            #execute_count_query
            if let Err(e) = count_rst {
                break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::QueryError, Some(Box::new(e)), None)))
            }
            let count = count_rst.expect("Unexpected error");
            #dto_ident.init(count as u64);

            // execute page_all query
            let query = (#executor_token).create_query();
            query.page_all::<_, _, #ret_type>(#executor_token, named_template, &#dto_token).await 
        });

        Ok(ret)
    }

    /// 在编译时生成运行时根据 dto 进行 render 后得到的 named_template
    /// 
    /// st: 在编译时生成的包含 sql 的结构体;
    fn gen_named_template_declare(&self, st: &crate::DyClosure) -> syn::Result<proc_macro2::TokenStream> {
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
        let rst = quote!(
            // 优先从 cache 中加载 sql 模板，如果 cache 中没有，则直接从序列化的二进制变量中加载并缓存 sql 模板
            let named_template = match dysql::get_sql_template(#template_id) {
                Some(tpl) => tpl,
                None => {
                    let serd_template =  [#(#serd_template,)*];
                    dysql::put_sql_template(#template_id, &serd_template).expect("Unexpected error when put_sql_template")
                },
            };
        );
        Ok(rst)
    }
}
