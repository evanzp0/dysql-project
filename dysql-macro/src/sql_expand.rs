use dysql_core::{save_sql_template, hash_str};
use dysql_tpl::Template;
use quote::quote;

use crate::DyClosure;

pub(crate) struct SqlExpand;

impl SqlExpand {

    /// expend fetch_one
    pub fn fetch_one(&self, st: &DyClosure) -> syn::Result<proc_macro2::TokenStream>{
        let dto_ident = &st.dto;
        let executor_ident = &st.executor;
        let executor_token = st.gen_executor_token();
        let ret_type = &st.ret_type;

        // declare named_sql at runtime
        let named_sql_declare = self.gen_named_sql_declare(st)?;

        let query_declare = if let Some(dto) = dto_ident {
            quote!(
                let rst = dysql::extract_params(&named_sql, #executor_ident.get_dialect());
                // println!("rst == {:?}", rst);
                if let Err(e) = rst {
                    break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::ExtractSqlParamterError, Some(Box::new(e)), None)))
                }
                let (sql, param_names) = rst.unwrap(); 
                let query = #executor_ident.create_query(&sql, param_names, Some(#dto));
            )
        } else {
            quote!(
                let query = #executor_ident.create_query::<dysql::EmptyObject>(&named_sql, Vec::<&str>::new(), None);
            )
        };

        let ret = quote!('rst_block: {
            use dysql::SqlxExecutorAdatper;
            #named_sql_declare  // let named_sql = ....;
            #query_declare      // let query = executor.create_query(....);
            query.fetch_one::<_, #ret_type>(#executor_token).await // all adapter must keep the same interface, let rst: ::std::result::Result<#ret_type, ::dysql::DySqlError>
        });

        Ok(ret)
    }

    /// expend fetch_all
    pub fn fetch_all(&self, st: &DyClosure) -> syn::Result<proc_macro2::TokenStream>{
        let dto_ident = &st.dto;
        let executor_ident = &st.executor;
        let executor_token = st.gen_executor_token();
        let ret_type = &st.ret_type;

        // declare named_sql at runtime
        let named_sql_declare = self.gen_named_sql_declare(st)?;

        let query_declare = if let Some(dto) = dto_ident {
            quote!(
                let rst = dysql::extract_params(&named_sql, #executor_ident.get_dialect());
                if let Err(e) = rst {
                    break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::ExtractSqlParamterError, Some(Box::new(e)), None)))
                }
                let (sql, param_names) = rst.unwrap(); 
                let query = #executor_ident.create_query(&sql, param_names, Some(#dto));
            )
        } else {
            quote!(
                let query = #executor_ident.create_query::<dysql::EmptyObject>(&named_sql, Vec::<&str>::new(), None);
            )
        };

        let ret = quote!('rst_block: {
            use dysql::SqlxExecutorAdatper;
            #named_sql_declare  // let named_sql = ....;
            #query_declare      // let query = executor.create_query(....);
            query.fetch_all::<_, #ret_type>(#executor_token).await
        });

        Ok(ret)
    }

    /// expend fetch_scalar
    pub fn fetch_scalar(&self, st: &DyClosure) -> syn::Result<proc_macro2::TokenStream>{
        let dto_ident = &st.dto;
        let executor_ident = &st.executor;
        let executor_token = st.gen_executor_token();
        let ret_type = &st.ret_type;

        // declare named_sql at runtime
        let named_sql_declare = self.gen_named_sql_declare(st)?;

        let query_declare = if let Some(dto) = dto_ident {
            quote!(
                let rst = dysql::extract_params(&named_sql, #executor_ident.get_dialect());
                if let Err(e) = rst {
                    break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::ExtractSqlParamterError, Some(Box::new(e)), None)))
                }
                let (sql, param_names) = rst.unwrap(); 
                let query = #executor_ident.create_query(&sql, param_names, Some(#dto));
            )
        } else {
            quote!(
                let query = #executor_ident.create_query::<dysql::EmptyObject>(&named_sql, Vec::<&str>::new(), None);
            )
        };

        let ret = quote!('rst_block: {
            use dysql::SqlxExecutorAdatper;
            #named_sql_declare  // let named_sql = ....;
            #query_declare      // let query = executor.create_query(....);
            query.fetch_scalar::<_, #ret_type>(#executor_token).await // all adapter must keep the same interface, let rst: ::std::result::Result<#ret_type, ::dysql::DySqlError>
        });

        Ok(ret)
    }

    /// expend execute
    pub fn execute(&self, st: &DyClosure) -> syn::Result<proc_macro2::TokenStream>{
        let dto_ident = &st.dto;
        let executor_ident = &st.executor;
        let executor_token = st.gen_executor_token();

        // declare named_sql at runtime
        let named_sql_declare = self.gen_named_sql_declare(st)?;

        let query_declare = if let Some(dto) = dto_ident {
            quote!(
                let rst = dysql::extract_params(&named_sql, #executor_ident.get_dialect());
                if let Err(e) = rst {
                    break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::ExtractSqlParamterError, Some(Box::new(e)), None)))
                }
                let (sql, param_names) = rst.unwrap(); 
                let query = #executor_ident.create_query(&sql, param_names, Some(#dto));
            )
        } else {
            quote!(
                let query = #executor_ident.create_query::<dysql::EmptyObject>(&named_sql, Vec::<&str>::new(), None);
            )
        };

        let ret = quote!('rst_block: {
            use dysql::SqlxExecutorAdatper;
            #named_sql_declare  // let named_sql = ....;
            #query_declare      // let query = executor.create_query(....);
            query.execute(#executor_token).await
        });

        Ok(ret)
    }

    /// expend insert
    pub fn insert(&self, st: &DyClosure) -> syn::Result<proc_macro2::TokenStream>{
        let dto_ident = &st.dto;
        let executor_ident = &st.executor;
        let executor_token = st.gen_executor_token();
        let ret_type = &st.ret_type;

        // declare named_sql at runtime
        let named_sql_declare = self.gen_named_sql_declare(st)?;

        let query_declare = if let Some(dto) = dto_ident {
            quote!(
                let rst = dysql::extract_params(&named_sql, #executor_ident.get_dialect());
                if let Err(e) = rst {
                    break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::ExtractSqlParamterError, Some(Box::new(e)), None)))
                }
                let (sql, param_names) = rst.unwrap(); 
                let query = #executor_ident.create_query(&sql, param_names, Some(#dto));
            )
        } else {
            quote!(
                let query = #executor_ident.create_query::<dysql::EmptyObject>(&named_sql, Vec::<&str>::new(), None);
            )
        };

        let ret = quote!('rst_block: {
            use dysql::SqlxExecutorAdatper;
            #named_sql_declare  // let named_sql = ....;
            #query_declare      // let query = executor.create_query(....);
            query.insert::<_, #ret_type>(#executor_token).await // all adapter must keep the same interface, let rst: ::std::result::Result<#ret_type, ::dysql::DySqlError>
        });

        Ok(ret)
    }

    /// expend page query
    pub fn page(&self, st: &DyClosure) -> syn::Result<proc_macro2::TokenStream>{
        let dto_ident = &st.dto;
        let executor_ident = &st.executor;
        let executor_token = st.gen_executor_token();
        let ret_type = &st.ret_type;

        // declare named_sql whith template at runtime 
        let named_sql_declare = self.gen_named_sql_declare(st)?;

        let query_declare = if let Some(dto) = dto_ident {
            quote!(
                let rst = dysql::extract_params(&named_sql, #executor_ident.get_dialect());
                if let Err(e) = rst {
                    break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::ExtractSqlParamterError, Some(Box::new(e)), None)))
                }
                let (sql, param_names) = rst.unwrap(); 
                let query = #executor_ident.create_query(&sql, param_names, Some(#dto));
            )
        } else {
            quote!(
                let query = #executor_ident.create_query::<dysql::EmptyObject>(&named_sql, Vec::<&str>::new(), None);
            )
        };

        let ret = quote!('rst_block: {
            use dysql::SqlxExecutorAdatper;
            #named_sql_declare  // let named_sql = ....;
            #query_declare      // let query = executor.create_query(....);
            query.fetch_all::<_, #ret_type>(#executor_token).await
        });

        Ok(ret)
    }

    /// 在编译时根据 dto 生成运行时需要使用的 named_sql
    /// 
    /// st: 在编译时生成的包含 sql 的结构体;
    fn gen_named_sql_declare(&self, st: &crate::DyClosure) -> syn::Result<proc_macro2::TokenStream> {
        let dto_ident = &st.dto;

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


// macro_rules! impl_sqlspand_types {
//     ($( $name:ident),*) => {
//         $(
//             impl SqlExpand for $name {}
//         )*
//     }
// }

// pub struct FetchAll;
// pub struct Execute;
// pub struct FetchOne;

// pub struct FetchScalar;
// pub struct Insert;

// impl_sqlspand_types!(FetchAll, Execute, FetchOne, FetchScalar, Insert);