// synchronized
// 对照查看 future 版本 - 那里有揭秘
use std::thread::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("befor reading files");
    let h1 = tokio::spawn(async {
        // 异步函数需要用 await 调用
        let _file1 = read_form_file1().await;
    });
    let h2 = tokio::spawn(async {
        let _file2 = read_form_file2().await;
    });

    let _ = tokio::join!(h1, h2);
}

// 加入 async 以后函数就成了异步的惰性函数
async fn read_form_file1() -> String {
    sleep(Duration::new(4, 0));
    println!("{:?}", "file 1");
    String::from("h1")
}

async fn read_form_file2() -> String {
    sleep(Duration::new(2, 0));
    println!("{:?}", "file 2 ");
    String::from("h2")
}
