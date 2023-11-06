use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
enum Event {
    Update,
    Delete,
    Unknow,
}
fn main() {
    let file = File::open("cargo.toml").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let (event, message) = parse_log(line);
        match event {
            Event::Unknow => {}
            Event::Delete => println!("{:?}, {}", event, message),
            Event::Update => println!("{:?}, {}", event, message),
        }
    }
}
type Message = String;
fn parse_log(line: String) -> (Event, Message) {
    let parts: Vec<&str> = line.splitn(2, ' ').collect();
    if parts.len() == 1 {
        return (Event::Unknow, "format error".to_string());
    }

    let event = parts[0];
    let rest = String::from(parts[1]);
    match event {
        "UPDATE" | "update" => (Event::Update, rest),
        "DELETE" | "delete" => (Event::Delete, rest),
        _ => (Event::Unknow, "unknow erroe".to_string()),
    }
}
