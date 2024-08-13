use anyhow::Result;
use rand::{thread_rng, Rng};
use rayon::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    println!("start task sum");
    let handle1 = tokio::task::spawn(sum());
    println!("start task concat");
    let _concat1 = concat();
    let handle2 = tokio::task::spawn(concat());

    let (r1, r2) = tokio::join!(handle1, handle2);
    r1?;
    r2?;

    Ok(())
}

async fn sum() -> u16 {
    let now = std::time::Instant::now();
    let sum = (0u8..u8::MAX).into_par_iter().map(|x| x as u16).sum();
    println!("sum elapsed:{}", now.elapsed().as_secs_f32());
    println!("sum:{}", sum);
    sum
}

async fn concat() -> String {
    let now = std::time::Instant::now();
    let string: String = (0u8..u8::MAX)
        .into_par_iter()
        .map(|_| {
            let mut rng = thread_rng();
            rng.gen_range('a'..='z')
        })
        .collect();
    println!("concat elapsed:{}", now.elapsed().as_secs_f32());
    println!("string:{}", string.len());
    string
}
