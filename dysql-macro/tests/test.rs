use dysql_macro::*;

#[test]
fn test_plain_sql() -> Result<(), Box<dyn std::error::Error>> {
    struct UserDto {
        name: String,
        id: i32,
    }

    impl UserDto {
        fn new(name: String, id: i32) -> Self {
            Self { name, id }
        }
    }

    let dto = UserDto::new("name1".to_owned(), 12);
    let rst = sql!(|dto| -> postgres {
        r#"select * from abc 
        where id = :id "aa
          and name = :name  
        order by id"#
    });
    
    println!("{:?}", rst);

    Ok(())
}
