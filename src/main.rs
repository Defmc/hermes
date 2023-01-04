use std::time::{Duration, Instant};

use hermes::{Bench, BenchSize, Bencher, NewBench};

fn main() {
    let mut bench = Bench::no_setup(&|_| println!("a"))
        .name("foo")
        .size(BenchSize::Time(Duration::from_secs(1)));
    bench.run();

    let mut new_bench = NewBench::new(&|| (), &|_| println!("a"))
        .name("bar")
        .size(BenchSize::Time(Duration::from_secs(1)));
    new_bench.run();

    let start = Instant::now();
    let mut amount = 0;
    while start.elapsed() < Duration::from_secs(1) {
        println!("a");
        amount += 1;
    }

    println!("old: {bench}");
    println!("new: {new_bench}");
    println!("native: {amount}");
}
