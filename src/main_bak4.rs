use std::ops::Deref;

trait Operation {
    fn operation(self) -> u32;
}

fn meth<T>(data: T) 
where
   T: std::ops::Deref + std::fmt::Debug,
{
    println!("{:?}", data);
}

#[derive(Debug)]
struct Sa {
    age: i32,
}

// impl Deref for Sa {
//     type Target = i32;

//     fn deref(&self) -> &Self::Target {
//         &self.age
//     }
// }

fn main() {
    let mut a = Sa { age: 1 };

    meth(&mut a);
}