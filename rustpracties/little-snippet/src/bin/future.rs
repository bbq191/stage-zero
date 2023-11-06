// future
// 对照查看 synchronized 版本
use std::thread::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("before reading files");
    let h1 = tokio::spawn(async {
        // 异步函数需要用 await 调用
        let _file1 = read_form_file1().await;
    });
    let h2 = tokio::spawn(async {
        let _file2 = read_form_file2().await;
    });

    let _ = tokio::join!(h1, h2);
}

// 用 future 替换了 async/await 版本
// async 和 await 只不过是 future 的语法糖
use std::future::Future;

fn read_form_file1() -> impl Future<Output = String> {
    async {
        sleep(Duration::new(4, 0));
        println!("{:?}", "future file 1");
        String::from("h1")
    }
}

fn read_form_file2() -> impl Future<Output = String> {
    async {
        sleep(Duration::new(2, 0));
        println!("{:?}", "future file 2");
        String::from("h2")
    }
}
