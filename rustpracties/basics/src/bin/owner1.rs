fn main() {
    // 使用尽可能多的方法来通过编译
    let x = String::from("hello, world");
    let y = x.clone();
    let y = x.as_str();
    let x = &String::from("hello, world");
    let x = "hello, world";
    let y = x;
    println!("{},{}", x, y);
}
