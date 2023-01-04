use std::{fmt, time::Duration};

pub mod classic;
pub use classic::*;

pub mod iter;
pub use iter::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BenchSize {
    Iters(u32),
    Time(Duration),
}

impl Default for BenchSize {
    fn default() -> Self {
        Self::Time(Duration::from_secs(1))
    }
}

pub trait Bencher {
    #[must_use]
    fn logs(&self) -> &Logs;

    #[must_use]
    fn logs_mut(&mut self) -> &mut Logs;

    #[must_use]
    fn with_name(mut self, name: impl AsRef<str>) -> Self
    where
        Self: Sized,
    {
        self.logs_mut().name = name.as_ref().to_owned();
        self
    }

    #[must_use]
    fn with_size(mut self, size: BenchSize) -> Self
    where
        Self: Sized,
    {
        self.logs_mut().size = size;
        self
    }

    #[must_use]
    fn mean(&self) -> Duration {
        self.logs().elapsed / self.logs().iters
    }

    #[must_use]
    fn step(&mut self) -> Duration;

    /// Runs tests and stores the results
    fn run(&mut self) {
        match self.logs().size {
            BenchSize::Iters(n) => {
                for _ in 0..n {
                    let elapsed = self.step();
                    self.logs_mut().elapsed += elapsed;
                }
                self.logs_mut().iters += n;
            }
            BenchSize::Time(t) => {
                while self.logs().elapsed < t {
                    let elapsed = self.step();
                    self.logs_mut().iters += 1;
                    self.logs_mut().elapsed += elapsed;
                }
            }
        }
    }

    /// # Errors
    /// When `write_str` or `write_fmt` fails
    fn display(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.logs().name)?;
        f.write_fmt(format_args!(": {:?}", self.logs().elapsed))?;
        f.write_fmt(format_args!(" / {}", self.logs().iters))?;
        f.write_fmt(format_args!(" = ± {:?}/iter", self.mean()))?;
        Ok(())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Logs {
    pub name: String,
    pub iters: u32,
    pub elapsed: Duration,
    pub size: BenchSize,
}
