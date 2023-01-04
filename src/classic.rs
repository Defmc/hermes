use crate::{Bencher, Logs};
use std::{
    fmt,
    time::{Duration, Instant},
};

#[allow(clippy::module_name_repetitions)]
pub struct ClassicBench<'a, T, U> {
    logs: Logs,
    test: &'a dyn Fn(T) -> U,
    setup: &'a dyn Fn() -> T,
    post: &'a dyn Fn(U),
}

impl<'a, T, U> ClassicBench<'a, T, U> {
    #[must_use]
    pub fn new(setup: &'a dyn Fn() -> T, test: &'a dyn Fn(T) -> U) -> Self {
        Self {
            logs: Logs::default(),
            setup,
            test,
            post: &std::mem::drop,
        }
    }

    #[must_use]
    pub fn with_post(mut self, post: &'a dyn Fn(U)) -> Self {
        self.post = post;
        self
    }
}

impl<'a, T, U> Bencher for ClassicBench<'a, T, U> {
    fn logs(&self) -> &Logs {
        &self.logs
    }

    fn logs_mut(&mut self) -> &mut Logs {
        &mut self.logs
    }

    fn step(&mut self) -> Duration {
        let setup = (self.setup)();
        let start = Instant::now();
        let out = (self.test)(std::hint::black_box(setup));
        let elapsed = start.elapsed();
        (self.post)(out);
        elapsed
    }
}

impl<'a, T, U> fmt::Display for ClassicBench<'a, T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(f)
    }
}
