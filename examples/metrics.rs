use anyhow::Result;
use concurrecny::Metrics;
use rand::{thread_rng, Rng};
use std::thread;
use std::time::Duration;

const M: usize = 2;
const N: usize = 4;
fn main() -> Result<()> {
    let metrics = Metrics::default();
    println!("{:?}", metrics.snapshot());

    for idx in 0..M {
        let metrics = metrics.clone();
        task_worker(idx, metrics);
    }

    for _ in 0..N {
        let metrics = metrics.clone();
        request_worker(metrics);
    }

    loop {
        println!("{:?}", metrics);
        let random_no = rand::random::<u8>();
        if random_no % 5 == 0 {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(200));
    }
}

fn task_worker(idx: usize, metrics: Metrics) {
    thread::spawn(move || loop {
        let mut rng = thread_rng();
        let key = format!("call.thread.worker.{}", idx);
        metrics.increase(key).unwrap();
        let sleep_time = rng.gen_range(200..500);
        thread::sleep(Duration::from_millis(sleep_time));
    });
}

fn request_worker(metrics: Metrics) {
    thread::spawn(move || loop {
        let mut rng = thread_rng();
        let idx = rng.gen_range(0..5);
        let key = format!("req.page.{}", idx);
        metrics.increase(key).unwrap();
        let sleep_time = rng.gen_range(200..500);
        thread::sleep(Duration::from_millis(sleep_time));
    });
}
