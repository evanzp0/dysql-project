use dysql::{Content, SimpleTemplate, SimpleValue};

fn main() {
    let dto = Some(UserDto{ id: Some(1), name: None, age: Some(13) , id_rng: None });
    let param_names = vec!["id"];
    for name in param_names {
        let stpl = SimpleTemplate::new(name);
        let rst = stpl.apply(&dto);
        match &rst {
            Ok(val) => match val{
                SimpleValue::t_str(_) => {
                    println!("{:?}", val.as_str())
                }
                _ => {},
            },
            Err(_) => {},
        }

        println!("{:?}", rst)
    }
}

#[derive(Content)]
struct UserDto {
    id: Option<i64>,
    name: Option<String>,
    age: Option<i32>,
    id_rng: Option<Vec<i32>>,
}