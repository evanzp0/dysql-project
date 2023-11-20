trait Operation {
    fn operation(self) -> u32;
}

struct Sa(u32);

impl<'a> Operation for &'a mut Sa {
    fn operation(self) -> u32 {
        (*self).0 = 10;
        (*self).0
    }
}

// impl Operation for u32 {
//     fn operation(self) -> u32 {
//         self
//     }
// }

impl<'a> Operation for &'a u32 {
    fn operation(self) -> u32 {
        *self
    }
}
struct Container<T>
    // where for<'a> &'a T: Sized + Operation
{
    x: T,
}

impl<T> Container<T>
    where for<'a> &'a T: Sized + Operation
    // where T: Sized + Operation
{
    fn do_thing(&self) -> u32 {
        // let b = &self.x;
        // b.operation();
        self.x.operation();
        Operation::operation(&self.x)
    }
}

fn main() {
    let mut sa = Sa(15);
    // (&sa).operation();
    let x = sa.operation();
    println!("x = {x}");
    
    // Operation::operation(&sa);

    let x  = (&(3 as u32)).operation();
    let y = (3 as u32).operation();
    // Operation::operation(y);

    println!("y = {}", y);
    let container = Container { x: 1 as u32 };
    println!("{}", container.do_thing());
}