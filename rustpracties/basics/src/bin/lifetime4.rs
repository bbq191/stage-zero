/* 使用三种方法修复下面的错误  */
fn invalid_output1<'a>() -> String {
    String::from("foo")
}
fn invalid_output2<'a>() -> &'a str {
    "foo"
}
fn main() {}
