use tokio::task;
use tokio::time::{sleep, Duration};

async fn perform_task() {
    println!("Task started.");
    // 模拟耗时操作
    sleep(Duration::from_secs(5)).await;
    println!("Task completed.");
}

async fn another_async_task() {
    let mut sum = 0;
    while sum != 5 {
        sum += 1;
        println!("sum:{}", sum);
        sleep(Duration::from_millis(5)).await;
    }
}

#[tokio::main]
async fn main() {
    println!("Program started.");

    // 启动两个任务并发执行
    let task1 = task::spawn(perform_task());
    let task2 = task::spawn(another_async_task());

    // 等待两个任务完成
    let _ = tokio::join!(task1, task2);

    println!("Program completed.");
}
