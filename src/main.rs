use dysql::{Content, Template};

fn main() {
    let v1 = Sa {
        bb: Sb {
            name: "my_name".to_owned(),
        },
    };

    let s = "{{bb.name}}";
    let tpl = Template::new(s).unwrap();
    let rst = tpl.render(&v1);
    println!("rst = {}", rst)
}

#[derive(Content)]
struct Sa {
    bb: Sb,
}

#[derive(Content)]
struct Sb {
    name: String,
}
