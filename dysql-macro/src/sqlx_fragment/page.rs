use quote::quote;

use crate::{sql_expand::SqlExpand, sqlx_fragment::gen_dto_quote};

pub struct Page;

impl SqlExpand for Page {

    fn expand(&self, st: &crate::DySqlFragmentContext) -> syn::Result<proc_macro2::TokenStream> {
        let dto_ident = &st.dto;
        let cot = &st.cot;
        let ret_type = &st.ret_type;

        let cot = super::gen_cot_quote(st, cot);
        let (param_strings, param_idents) = self.extra_params(st)?;

        // count query ----------------------

        // declare sql and bind params at runtime
        let count_sql = format!("SELECT count(*) FROM ({}) as _tmp", &st.body);
        let declare_rt = self.gen_declare_rt(st, Some(&count_sql), true)?;

        let rst_count = match dto_ident {
            Some(dto_ident) => {
                let dto = gen_dto_quote(st, dto_ident);
                quote!(
                    #declare_rt
                    
                    let mut query = sqlx::query_scalar::<_, i64>(&sql);
    
                    for i in 0..param_names.len() {
                        #(
                            if param_names[i] == #param_strings {
                                query = query.bind(&#dto_ident.#param_idents);
                            }
                        )*
                    }
            
                    let rst = query.fetch_one(#cot).await;
                    if let Err(e) = rst {
                        break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::QueryError, Some(Box::new(e)), None)))
                    }
                    let count = rst.expect("Unexpected error");
    
                    {
                        let _tmp_pg_dto: &mut PageDto<_> = #dto;
                        _tmp_pg_dto.init(count as u64);
                    }
                )
            },
            None => return Err(syn::Error::new(proc_macro2::Span::call_site(), "missing PageDto object in page query")),
        };
    
        // page query ----------------------

        // declare sql and bind params at runtime
        let mut page_sql = st.body.to_owned();
        page_sql.push_str(
            " {{#is_sort}} ORDER BY {{#sort_model}} {{field}} {{sort}}, {{/sort_model}} ![B_DEL(,)] {{/is_sort}} LIMIT {{page_size}} OFFSET {{start}} "
        );
        let declare_rt = self.gen_declare_rt(st, Some(&page_sql), false)?;

        let rst_page = match dto_ident {
            Some(dto_ident) => quote!(
                #declare_rt

                let mut query = sqlx::query_as::<_, #ret_type>(&sql);
                for i in 0..param_names.len() {
                    #(
                        if param_names[i] == #param_strings {
                            query = query.bind(&#dto_ident.#param_idents);
                        }
                    )*
                }
    
                let rst = query.fetch_all(#cot).await;
                if let Err(e) = rst {
                    break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::QueryError, Some(Box::new(e)), None)))
                }
                let rst = rst.expect("Unexpected error");
                let pg_data = dysql::Pagination::from_dto(&#dto_ident, rst);

                Ok(pg_data)
            ),
            None => quote!(
                #declare_rt

                let mut query = sqlx::query_as::<_, #ret_type>(&sql);
                let rst = query.fetch_all(#cot).await;
                if let Err(e) = rst {
                    break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::QueryError, Some(Box::new(e)), None)))
                }
                let rst = rst.expect("Unexpected error");
                let pg_data = dysql::Pagination::from_dto(&#dto_ident, rst);
                
                Ok(pg_data)
            ),
        };

        let ret = quote!('rst_block: {
            #rst_count
            #rst_page
        });

        Ok(ret)
    }
}