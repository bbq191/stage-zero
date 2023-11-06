#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

///构造函数方式二 - 不推荐
fn new(width: i32, height: i32) -> Rectangle {
    Rectangle { width, height }
}
impl Rectangle {
    ///关联函数
    fn area(width: i32, height: i32) -> i32 {
        width * height
    }
    ///实例函数
    fn area_with_self(&self) -> i32 {
        self.width * self.height
    }
    ///构造函数方式一
    fn new(width: i32, height: i32) -> Self {
        Rectangle { width, height }
    }
}
fn main() {
    // 关联函数调用
    println!("{}", Rectangle::area(12, 18));
    // 实例函数调用 - 先申明实例对象
    let cube = Rectangle {
        width: 13,
        height: 13,
    };
    println!("{}", cube.area_with_self());
    // 构造函数 -- 方式一
    println!("{:#?}", Rectangle::new(18, 22));
    // 构造函数 -- 方式一
    println!("{:#?}", new(18, 20));
}
