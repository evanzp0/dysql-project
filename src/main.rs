
fn main() {
    let a = {
        fn m() -> Result<i32, String> {
            Ok(1)
        }

        m()

    }.unwrap();

    println!("{:?}", a)
}