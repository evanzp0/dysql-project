use std::io::{Cursor, Write};

fn main() {
    let buffer_size = 300 + 40;
    let mut buf = Vec::<u8>::with_capacity(buffer_size);
    // let mut cursor = Cursor::new(&mut buf[..]);
    write!(buf, "SELECT count(*) FROM ({}) as _tmp", "named_sql").unwrap();
    // let len = cursor.position() as usize;
    let a = std::str::from_utf8(&buf).unwrap();
    println!("{} {}", a, buf.len());
}