use crate::{Bencher, Logs};
use core::fmt;
use std::time::{Duration, Instant};

/// An `Iterator`-based benchmarking struct.
/// Iterates over an iterator to get tests inputs.
/// It's highly recommended to use an infinity (or cyclic) iterator, once its can't crash.
#[allow(clippy::module_name_repetitions)]
pub struct IterBench<'a, I, U>
where
    I: Iterator,
{
    logs: Logs,
    test: &'a dyn Fn(<I as Iterator>::Item) -> U,
    setup: I,
    post: &'a dyn Fn(U),
}

impl<'a, I, U> IterBench<'a, I, U>
where
    I: Iterator,
{
    #[must_use]
    pub fn new(setup: I, test: &'a dyn Fn(<I as Iterator>::Item) -> U) -> Self {
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

impl<'a, I, U> Bencher for IterBench<'a, I, U>
where
    I: Iterator,
{
    fn logs(&self) -> &Logs {
        &self.logs
    }

    fn logs_mut(&mut self) -> &mut Logs {
        &mut self.logs
    }

    /// # Panics
    /// When there's no next item on the passed iterator.
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
