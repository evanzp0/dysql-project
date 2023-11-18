use dysql::Content;

fn main() {
    let dto = UserDto {
        id: None //Some("ab".to_owned())
    };

    let rst = dto.id.apply_unescaped();
    println!("{:?}", rst.unwrap());

}

#[derive(Content)]
struct UserDto {
    id: Option<String>,
}