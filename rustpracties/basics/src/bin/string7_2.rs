// 使用至少两种方法来修复错误
fn main() {
    let s = String::from("hello, world");
    greetings(s)
}

fn greetings(s: String) {
    println!("{}", s)
}
