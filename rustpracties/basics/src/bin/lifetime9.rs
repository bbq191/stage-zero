/* 添加合适的生命周期让下面代码工作 */
struct ImportantExcerpt {
    part: String,
}

impl ImportantExcerpt {
    fn level<'a>(&'a self) -> i32 {
        3
    }
}

fn main() {}
