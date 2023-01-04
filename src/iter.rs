use core::fmt;
use std::time::{Duration, Instant};

use crate::{BenchSize, Bencher};

#[allow(clippy::module_name_repetitions)]
pub struct IterBench<'a, I, U>
where
    I: Iterator,
{
    name: String,
    size: BenchSize,
    test: &'a dyn Fn(<I as Iterator>::Item) -> U,
    setup: I,
    post: &'a dyn Fn(U),
    dur: Duration,
    iters: u32,
}

impl<'a, I, U> IterBench<'a, I, U>
where
    I: Iterator,
{
    #[must_use]
    pub fn new(setup: I, test: &'a dyn Fn(<I as Iterator>::Item) -> U) -> Self {
        Self {
            name: String::new(),
            size: BenchSize::Iters(1_000),
            setup,
            test,
            post: &std::mem::drop,
            dur: Duration::ZERO,
            iters: 0,
        }
    }

    #[must_use]
    pub fn with_name(mut self, name: impl AsRef<str>) -> Self {
        self.name = name.as_ref().to_string();
        self
    }

    #[must_use]
    pub const fn with_size(mut self, size: BenchSize) -> Self {
        self.size = size;
        self
    }

    #[must_use]
    pub fn with_post(mut self, post: &'a dyn Fn(U)) -> Self {
        self.post = post;
        self
    }
}

impl<'a, I, U> Bencher for IterBench<'a, I, U>
where
    I: Iterator,
{
    fn iters(&self) -> &u32 {
        &self.iters
    }

    fn iters_mut(&mut self) -> &mut u32 {
        &mut self.iters
    }

    fn elapsed(&self) -> &Duration {
        &self.dur
    }

    fn elapsed_mut(&mut self) -> &mut Duration {
        &mut self.dur
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn size(&self) -> BenchSize {
        self.size
    }

    fn step(&mut self) -> Duration {
        let setup = self.setup.next().expect("iterator is not enough");
        let start = Instant::now();
        let out = (self.test)(std::hint::black_box(setup));
        let elapsed = start.elapsed();
        (self.post)(out);
        elapsed
    }
}

impl<'a, I, U> fmt::Display for IterBench<'a, I, U>
where
    I: Iterator,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(f)
    }
}
