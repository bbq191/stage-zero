fn main() {
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];
    // 填空，让代码工作
    // assert_eq!([x, y], __);
    assert_eq!([x, y], [3, 2]);
}