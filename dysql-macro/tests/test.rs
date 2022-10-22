use dysql_macro::*;
pub use ramhorns::Content;

#[test]
fn test_plain_sql() -> dysql::DySqlResult<()>{
    #[derive(Content)]
    struct UserDto {
        name: String,
        id: i32,
        age: Option<i32>
    }

    impl UserDto {
        fn new(name: String, id: i32, age: Option<i32>) -> Self {
            Self { name, id, age }
        }
    }

    let dto = UserDto::new("name1".to_owned(), 12, Some(13));
    let rst = sql!(|dto| -> postgres {
        r#"select * from abc 
        where id = :id
          and name = :name
          {{#age}}and age = :age{{/age}}
        order by id"#
    });

    
    let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = vec![];
    params.push(&dto.id);
    params.push(&dto.name);
    params.push(&dto.age);
    assert_eq!("select * from abc where id = $1 and name = $2 and age = $3 order by id".to_string(), rst.0);
    let params = format!("{:?}", params);
    let rst1 = format!("{:?}", rst.1);
    assert_eq!(params, rst1);

    Ok(())
}
