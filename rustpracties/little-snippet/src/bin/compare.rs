use std::convert::TryInto;

// rust 不允许不同类型进行比较
fn main() {
    // 对于整型的比较可以用 TryInto
    let a: i32 = 10;
    let b: u32 = 1030;
    let b = b.try_into().unwrap();
    if a < b {
        println!("{} < {}", a, b);
    }

    // 对于浮点数的比较
    let a: f64 = 0.1 + 0.22;
    let b: f64 = 0.32;
    println!("{}", a - b);
    let abs_diff = (a - b).abs();
    assert!(abs_diff <= f64::EPSILON); // 只比较相减后的合理区间

    // NaN 与 infinite
    let a = (-4.0_f32).sqrt();
    println!("{}", a);
    // assert_eq!(a, a); // nan 不等于任何值包括其自身，并且 nan 断言会使程序 panic

    let a: f32 = 99.0 / 0.0;
    println!("{}", a);
    assert_eq!(a, a); // nan 不等于任何值包括其自身
}
