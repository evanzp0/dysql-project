use dysql::{Content, SimpleTemplate, SimpleValue};

fn main() {
    let son = Son {
        id: 1,
        name: "maomao",
    };

    let dto = UserDto {
        id: 12,
        name: "evan",
        job: Some("it".to_owned()),
        descp: None,
        son,
    };

    // let param_names = vec!["id", "name", "job", "descp", "not_exist", "son.id", "son.name" ];
    let param_names = vec!["son.id", "son.name" ];
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
struct UserDto<'a> {
    id: i32,
    name: &'a str,
    job: Option<String>,
    descp: Option<&'a str>,
    son: Son<'a>,
}


#[derive(Content)]
struct Son<'a> {
    id: i32,
    name: &'a str,
}