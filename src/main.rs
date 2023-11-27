use dysql::Template;


fn main() {
    let dto = UserDto::new(None, Some("z".to_owned()), Some(13), Some(vec![1, 2, 3,]));
    let sort_model = vec![
        dysql::SortModel {field: "id".to_owned(), sort: "desc".to_owned()}
    ];
    let mut pg_dto = dysql::PageDto::new_with_sort(3, 10, Some(&dto), sort_model);
    pg_dto.init(9);
    println!("{:?}", pg_dto.start);
    let s = "{{#sort_model}} {{field}} {{sort}} {{/sort_model}} LIMIT";

    let tpl = Template::new(s).unwrap();
    let rst = tpl.render_sql(&pg_dto);

    println!("{}", rst);
}

#[derive(dysql::Content, Clone)]
pub struct UserDto {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub age: Option<i32>,
    pub id_rng: Option<Vec<i32>>,
}

impl UserDto {
    pub fn new(id: Option<i64>, name: Option<String>, age: Option<i32>, id_rng: Option<Vec<i32>>) -> Self {
        Self { id, name, age, id_rng }
    }
}
