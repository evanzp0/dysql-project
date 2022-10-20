use dysql_macro::*;

#[test]
fn test_dynamic_sql() -> Result<(), String> {
    struct UserDto {
        name: String,
        id: i32,
    }

    impl UserDto {
        fn new(name: String, id: i32) -> Self {
            Self {
                name,
                id,
            }
        }
    }

    let dto = UserDto::new("name1".to_owned(), 12);
    let rst = sql!( |dto, "my_sql_name"| { select * from abc where id=:id and name=:name order by id});
    println!("{}, {:#?}", rst.0, rst.1);

    Ok(())
}