use tokio::time::{sleep, Duration};

async fn perform_task() {
    println!("Task started.");
    // 模拟耗时操作
    sleep(Duration::from_secs(5)).await;
    println!("Task completed.");
}

#[tokio::main]
async fn main() {
    println!("Program started.");
    // let task = perform_task();
    perform_task().await;
    another_async_task().await;
    println!("Task is running in the background.");
    // task.await; // 等待任务完成
    println!("Program completed.");
}

async fn another_async_task() {
    let mut sum = 0;
    while sum != 5 {
        sum += 1;
        println!("sum:{}", sum);
        sleep(Duration::from_millis(5)).await;
    }
}
