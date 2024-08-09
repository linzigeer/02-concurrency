use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use anyhow::{anyhow, Result};
use rand::random;

const PRODUCER_NUM: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

impl Msg {
    pub fn new(idx: usize, value: usize) -> Self {
        Msg { idx, value }
    }
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    for i in 0..PRODUCER_NUM {
        let rx = tx.clone();
        thread::spawn(move || producer(i, rx));
    }

    drop(tx);

    let consumer = thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(msg) => {
                    println!("received msg:{:?}", msg);
                }
                Err(e) => {
                    eprintln!("No more msg or an error occurred while receive msg:{:?}", e);
                    break;
                }
            }
        }
        println!("consumer exit");
        42
    });

    let result = consumer
        .join()
        .map_err(|e| anyhow!("Error occurred while receive msg:{:?}", e))?;
    println!("result:{}", result);

    Ok(())
}

fn producer(idx: usize, sender: Sender<Msg>) -> Result<()> {
    loop {
        let value = random::<usize>();
        sender.send(Msg::new(idx, value))?;
        let sleep_time = random::<u8>() as u64 * 10;
        //sleep函数会短暂地让出cpu的执行权，系统会把当前线程挂起，并调度其他线程，让其他线程有机会得到执行
        //如果没有这个sleep的调用，那么for循环将永远地处于第一轮循环，而无法继续进行其他轮次的循环
        thread::sleep(Duration::from_millis(sleep_time));
        //让线程随机地退出
        if random::<u8>() % 5 == 0 {
            println!("producer {} exit", idx);
            break Ok(());
        }
    }
}
