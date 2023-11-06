// 定义在 impl 语句块中的函数被称为关联函数，因为它们跟当前类型关联在一起。关联函数与方法最大的区别就是它第一个参数不是 self ，原因是它们不需要使用当前的实例，因此关联函数往往可以用于构造函数：初始化一个实例对象。

#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    // 1. 实现下面的关联函数 `new`,
    // 2. 该函数返回一个 TrafficLight 实例，包含 `color` "red"
    // 3. 该函数必须使用 `Self` 作为类型，不能在签名或者函数体中使用 `TrafficLight`
    pub fn new() -> TrafficLight {
        TrafficLight {
            color: "red".to_string(),
        }
    }

    pub fn get_state(&self) -> &str {
        &self.color
    }
}

fn main() {
    let light = TrafficLight::new();
    assert_eq!(light.get_state(), "red");
}
