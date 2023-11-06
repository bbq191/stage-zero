fn main() {
    let tup = (1, 6.4, "hello");

    // 填空
    let (x, y, z) = tup;

    assert_eq!(x, 1);
    assert_eq!(z, "hello");
    assert_eq!(y, 6.4);
}
