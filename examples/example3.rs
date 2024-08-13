use std::thread::sleep;
use std::time::Duration;

fn perform_task() {
    println!("Task started.");
    // 模拟耗时操作
    sleep(Duration::from_secs(5));
    println!("Task completed.");
}

fn main() {
    println!("Program started.");
    perform_task();
    println!("Task is running in the background.");
    println!("Program completed.");
}
