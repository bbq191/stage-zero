use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("befor reading files");
    let file1 = read_form_file1();
    println!("{:?}", file1);
    println!("after reading file 1");
    let file2 = read_form_file2();
    println!("{:?}", file2);
    println!("after reading file 2");
}

fn read_form_file1() -> String {
    sleep(Duration::new(4, 0));
    String::from("file 1")
}

fn read_form_file2() -> String {
    sleep(Duration::new(2, 0));
    String::from("file 2")
}
