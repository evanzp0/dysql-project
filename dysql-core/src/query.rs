use dysql_tpl::Content;

use crate::InstanceOf;

pub enum QueryCmd {
    Execute(String),
    FetchAll(String),
    FetchOne(String),
    FetchScalar(String),
    Insert(String),
    Page {count_sql: String, page_sql: String},
}

pub struct Query<D> {
    pub query_type: QueryCmd,
    pub dto: Option<D>,
}

impl<D> Query<D> 
where 
    D: Content + 'static
{
    pub fn new(query_type: QueryCmd,dto: Option<D>) -> Self {
        Self {
            query_type,
            dto,
        }
    }

    pub fn execute<N>(&mut self, cot: &N)
    where 
        N: 'static + Sized,
    {
        // self.dto.take()
        cot.instance_of::<i32>();


        todo!()
    }

    pub fn gen_page_count_sql(raw_sql: &str) -> String {
        format!("SELECT count(*) FROM ({}) as _tmp", raw_sql)
    }
    
    pub fn gen_page_sql(raw_sql: &str) -> String {
        let mut page_sql = raw_sql.to_owned();
        page_sql.push_str(
            " {{#is_sort}} ORDER BY {{#sort_model}} {{field}} {{sort}}, {{/sort_model}} ![B_DEL(,)] {{/is_sort}} LIMIT {{page_size}} OFFSET {{start}} "
        );
    
        page_sql
    }
}
