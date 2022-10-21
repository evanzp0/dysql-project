use dysql_macro::*;
pub use ramhorns::Content;

#[test]
fn test_plain_sql() -> Result<(), Box<dyn std::error::Error>> {
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
          {{#age}}
          and age = :age
          {{/age}}
        order by id"#
    });

    println!("{:?}", rst);

    Ok(())
}
