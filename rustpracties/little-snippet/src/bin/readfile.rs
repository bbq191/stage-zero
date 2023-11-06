use std::error::Error;
use std::io::prelude::*;
use std::{fs::File, io::BufReader};

fn read_string_from_file() -> Result<String, std::io::Error> {
    let mut content = String::new();
    File::open("Cargo.toml")?.read_to_string(&mut content)?;
    Ok(content)
}
fn main() -> Result<(), Box<dyn Error>> {
    let s = read_string_from_file()?;
    println!("{}", s);
    Ok(())
}

// 不推荐的写法，会 panic，推荐上面的写法
// fn main() {
// let file = File::open("application-prod.properties").unwrap();
// let reader = BufReader::new(file);
//
// for line_rs in reader.lines() {
// let line = line_rs.unwrap();
// println!("{}, {} bytes length", line, line.len());
// }
// }
