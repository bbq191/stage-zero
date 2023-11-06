// in lib.rs

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // 使用三种方式填空
        //1. 使用关键字 `super`
        //2. 使用绝对路径
        super::front_of_house::serving::serve_order();
        crate::front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}
