// 修复错误并实现缺失的代码
fn main() {
    let mut v = Vec::from([1, 2, 3]);
    for i in 0..3 {
        println!("{:?}", v[i])
    }

    v.clear();
    for i in 0..5 {
        // 实现这里的代码...
        v.push(i + 2)
    }

    assert_eq!(v, vec![2, 3, 4, 5, 6]);

    println!("Success!")
}
