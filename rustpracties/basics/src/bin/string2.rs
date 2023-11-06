use std::ops::Deref;

// 使用至少两种方法来修复错误
fn main() {
    let s: Box<str> = "hello, world".into();
    greetings(s.deref());
    greetings(s.as_ref());
}

fn greetings(s: &str) {
    println!("{}", s)
}
