// 修复错误
// fn main() {
// println!("{}, world", x);
// }
// fn define_x() {
// let x = "hello";
// }

fn main() {
    let x = define_x();
    println!("{}, world", x);
}
fn define_x() -> &'static str {
    "hello"
}
