use quote::quote;

use crate::sql_expand::SqlExpand;

pub struct Page;

impl SqlExpand for Page {

    fn expand(&self, st: &crate::SqlClosure) -> syn::Result<proc_macro2::TokenStream> {
        let dto = &st.dto;
        let cot = &st.cot;
        let ret_type = &st.ret_type;

        let is_dto_ref = &st.is_dto_ref;
        let is_dto_ref_mut = &st.is_dto_ref_mut;
        let dto_ref = if *is_dto_ref { quote!(&) } else if *is_dto_ref_mut { quote!(&mut) } else { quote!() }; 

        let cot_ref = if st.is_cot_ref_mut {
            quote!(&mut )
        } else if st.is_cot_ref {
            quote!(&)
        } else {
            quote!()
        };

        let (param_strings, param_idents) = self.extra_params(st)?;

        // count query ----------------------

        // declare sql and bind params at runtime
        let count_sql = format!("SELECT count(*) FROM ({}) as _tmp", &st.body);
        let declare_rt = self.gen_declare_rt(st, Some(&count_sql))?;

        let rst_count = match dto {
            Some(_) => quote!(
                #declare_rt
                
                let mut query = sqlx::query_scalar::<_, i64>(&sql);

                for i in 0..param_names.len() {
                    #(
                        if param_names[i] == #param_strings {
                            query = query.bind(&#dto.#param_idents);
                        }
                    )*
                }
        
                let rst = query.fetch_one(#cot_ref #cot).await;
                if let Err(e) = rst {
                    break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::QueryError, Some(Box::new(e)), None)))
                }
                let count = rst.expect("Unexpected error");

                {
                    let _tmp_pg_dto: &mut PageDto<_> = #dto_ref #dto;
                    _tmp_pg_dto.init(count as u64);
                }
            ),
            None => quote!(
                #declare_rt
                let mut query = sqlx::query_scalar::<_, #ret_type>(&sql);

                let rst = query.fetch_one(#cot_ref #cot).await;
                if let Err(e) = rst {
                    break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::QueryError, Some(Box::new(e)), None)))
                }
                let count = rst.expect("Unexpected error");

                {
                    let _tmp_pg_dto: &mut PageDto<_> = #dto_ref #dto;
                    _tmp_pg_dto.init(count as u64);
                }
            ),
        };
    
        // page query ----------------------

        // declare sql and bind params at runtime
        let mut page_sql = st.body.to_owned();
        page_sql.push_str(
            " {{#is_sort}} ORDER BY {{#sort_model}} {{field}} {{sort}}, {{/sort_model}} ![B_DEL(,)] {{/is_sort}} LIMIT {{page_size}} OFFSET {{start}} "
        );
        let declare_rt = self.gen_declare_rt(st, Some(&page_sql))?;

        let rst_page = match dto {
            Some(_) => quote!(
                #declare_rt

                let mut query = sqlx::query_as::<_, #ret_type>(&sql);
                for i in 0..param_names.len() {
                    #(
                        if param_names[i] == #param_strings {
                            query = query.bind(&#dto.#param_idents);
                        }
                    )*
                }
    
                let rst = query.fetch_all(#cot_ref #cot).await;
                if let Err(e) = rst {
                    break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::QueryError, Some(Box::new(e)), None)))
                }
                let rst = rst.expect("Unexpected error");
                let pg_data = dysql::Pagination::from_dto(#dto_ref #dto, rst);

                Ok(pg_data)
            ),
            None => quote!(
                #declare_rt

                let mut query = sqlx::query_as::<_, #ret_type>(&sql);
                let rst = query.fetch_all(#cot_ref #cot).await;
                if let Err(e) = rst {
                    break 'rst_block  Err(dysql::DySqlError(dysql::ErrorInner::new(dysql::Kind::QueryError, Some(Box::new(e)), None)))
                }
                let rst = rst.expect("Unexpected error");
                let pg_data = dysql::Pagination::from_dto(#dto_ref #dto, rst);
                
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