
// 修改以下结构体让代码工作
struct Point<T, S> {
    x: T,
    y: S,
}

fn main() {
    // 不要修改这行代码！
    let p = Point{x: 5, y : "hello".to_string()};
}

