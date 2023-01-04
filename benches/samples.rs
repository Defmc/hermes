use std::time::Duration;

use hermes::{BenchSize, Bencher, IterBench};

fn rec_pow(base: u64, exp: u32) -> u64 {
    match exp {
        0 => 1,
        1 => base,
        _ => base * rec_pow(base, exp - 1),
    }
}

fn pow(base: u64, exp: u32) -> u64 {
    (0..exp).fold(1, |b, _| b * base)
}

fn main() {
    let iter: Vec<_> = (0..100)
        .map(|i| (1..=100).map(move |x| (i, x)))
        .flatten()
        .collect();

    let inputs = &iter.iter().cycle();

    let size = BenchSize::Time(Duration::from_secs(1));

    let mut rec_bench =
        IterBench::new(inputs.clone().copied(), &|(b, e): (u64, u32)| rec_pow(b, e))
            .with_name("recursive pow function")
            .with_size(size);

    let mut linear_bench = IterBench::new(inputs.clone().copied(), &|(b, e): (u64, u32)| pow(b, e))
        .with_name("linear pow function")
        .with_size(size);

    rec_bench.run();
    linear_bench.run();

    println!("{rec_bench}\n{linear_bench}");
}
