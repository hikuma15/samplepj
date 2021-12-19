
use std::time::{Duration, Instant};

fn main() {
    println!("start");

    let start = Instant::now();

    let result = leibniz_f64powf(TERMS);

    let end = start.elapsed();
    let dur:f32 = end.as_nanos() as f32 / 1000_000 as f32;
    //let dur:f64 = end.as_nanos() as f64;    
    println!("end {}ms",dur);
    println!("result {}",result);
}



static TERMS: u32 = 2e8 as u32;

pub fn leibniz_f64powf(terms: u32) -> f64 {
    leibniz_common(terms, &|n| (-1f64).powf(n as f64))
}

fn leibniz_common<F>(terms: u32, pow: &F) -> f64
    where F: Fn(u32) -> f64
{
    let mut s = 0.0;
    for n in 0..(terms + 1) {
        s+= pow(n) / (2 * n + 1) as f64;
    }
    s * 4.0
}