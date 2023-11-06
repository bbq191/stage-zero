use std::io::Error;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Person { name, age }
    }
    // 类似 this 指针，指向当前不可变对象
    fn greeting(&self) -> String {
        format!("Hello {}", self.name)
    }
    // 类似 this 指针，指向当前可变对象
    fn age_up(&mut self, arg: u8) {
        self.age += arg
    }
    // 使用后被销毁
    fn drop_self(self) {
        println!("drop")
    }
}
fn main() {
    let mut me = Person::new("vinci".to_string(), 89);
    println!("{:#?}", me);
    println!("{}", me.greeting());
    me.age_up(8);
    println!("my age now: {}", me.age);
    println!("Now: {:#?}", me);
    me.drop_self();
    // println!("{:#?}", me); // borrow of moved value `me`
}
