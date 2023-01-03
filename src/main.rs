use std::time::{Duration, Instant};

use hermes::{Bench, BenchSize};

fn main() {
    let mut bench = Bench::no_setup(&|_| println!("a"))
        .name("foo")
        .size(BenchSize::Time(Duration::from_secs(1)));
    bench.run();

    let start = Instant::now();
    let mut amount = 0;
    while start.elapsed() < Duration::from_secs(1) {
        println!("a");
        amount += 1;
    }

    println!("{bench}");
    println!("{amount}");
}
