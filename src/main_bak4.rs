use std::sync::Arc;

use dysql::{SimpleTemplate, Content};


#[tokio::main]
async fn main() {
    let sb = Sb {
        age: "121".to_owned()
    };
    let dto = Sa {
        bb: Arc::new(
            Some (
                &sb
            )
        )
    };

    let rst = SimpleTemplate::new("bb.age").apply(&dto);

    println!("{:?}", rst.unwrap().as_string())
}

#[derive(Debug, Content)]
struct Sa<'a> {
    bb: Arc::<Option<&'a Sb>>
}

#[derive(Debug, Content)]
struct Sb {
    age: String,
}