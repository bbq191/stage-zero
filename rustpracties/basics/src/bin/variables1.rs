// 修复下面代码的错误并尽可能少的修改
fn main() {
    // let x: i32; // 未初始化，但被使用
    // let y: i32; // 未初始化，也未被使用
    let x: i32 = 0;
    let _y: i32;
    println!("x is equal to {}", x);
}

