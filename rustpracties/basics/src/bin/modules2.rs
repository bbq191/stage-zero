#![allow(unused)]
fn main() {
    // in lib.rs

    // 填空并修复错误

    // 提示：你需要通过 `pub` 将一些项标记为公有的，这样模块 `front_of_house` 中的项才能被模块外的项访问
    mod front_of_house {
        /* ...snip... */
    }

    pub fn eat_at_restaurant() {
        // 使用绝对路径调用
        crate::front_of_house::hosting::add_to_waitlist();

        // 使用相对路径调用
        front_of_house::hosting::add_to_waitlist();
    }
}
