// 使用两种方法让代码工作起来
fn main() {
    let v = {
        let mut x = 1;
        x += 2
    };

    assert_eq!(v, ());
    //assert_eq!(v, 3);
}

