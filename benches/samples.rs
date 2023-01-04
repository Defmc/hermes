use hermes::{BenchSize, Bencher, ClassicBench, IterBench};
use std::{collections::HashSet, time::Duration};

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

fn nat_pow(base: u64, mut exp: u32) -> u64 {
    if exp == 0 {
        return 1;
    }
    let mut base = base;
    let mut acc = 1;

    while exp > 1 {
        if (exp & 1) == 1 {
            acc = acc * base;
        }
        exp /= 2;
        base = base * base;
    }

    acc * base
}

fn main() {
    let iter: Vec<_> = (0..100)
        .map(|i| (1..=100).map(move |x| (i, x)))
        .flatten()
        .collect();

    let anwsers: HashSet<u64> = iter.iter().map(|(b, e)| pow(*b, *e)).collect();

    let inputs = &iter.iter().cycle();

    let size = BenchSize::Time(Duration::from_secs(1));

    let assert_exists = |x| assert!(anwsers.contains(&x));

    let mut rec_bench =
        IterBench::new(inputs.clone().copied(), &|(b, e): (u64, u32)| rec_pow(b, e))
            .with_name("recursive pow function")
            .with_size(size)
            .with_post(&assert_exists);

    let mut linear_bench = IterBench::new(inputs.clone().copied(), &|(b, e): (u64, u32)| pow(b, e))
        .with_name("linear pow function")
        .with_size(size)
        .with_post(&assert_exists);

    let mut native_bench =
        IterBench::new(inputs.clone().copied(), &|(b, e): (u64, u32)| nat_pow(b, e))
            .with_name("native pow function")
            .with_size(size)
            .with_post(&assert_exists);

    let mut empty = ClassicBench::new(&|| (), &|_| ())
        .with_name("empty")
        .with_size(BenchSize::Iters(1))
        .with_post(&|x| assert_eq!(x, ()));

    rec_bench.run();
    linear_bench.run();
    native_bench.run();
    empty.run();

    println!("{rec_bench}\n{linear_bench}\n{native_bench}");
    println!("{empty}");
}
