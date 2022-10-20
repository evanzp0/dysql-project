use dysql::SqlDialect;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    let rst = {
        let rst = dysql::extract_params(
            "\"select * from abc where id = :id and name = :name order by id\"",
            SqlDialect::from("postgres".to_owned()),
        )?;
        let (sql, param_names) = rst;
        let mut param_values: Vec<&(dyn dysql::ToSql + Sync)> = Vec::new();
        for i in 0..param_names.len() {
            if param_names[i] == "id" {
                param_values.push(&dto.id);
            }
            if param_names[i] == "name" {
                param_values.push(&dto.name);
            }
        }
        (
            "\"\\\"select * from abc where id = $1 and name = $2 order by id\\\"\"",
            param_values,
        )
    };

    println!("{:?}", rst);

    Ok(())
}