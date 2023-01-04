use std::{fmt, time::Duration};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy)]
pub enum BenchSize {
    Iters(u32),
    Time(Duration),
}

pub trait Bencher {
    fn iters(&self) -> &u32;
    fn iters_mut(&mut self) -> &mut u32;

    fn elapsed(&self) -> &Duration;
    fn elapsed_mut(&mut self) -> &mut Duration;

    fn name(&self) -> &str;
    fn size(&self) -> BenchSize;

    fn mean(&self) -> Duration {
        *self.elapsed() / *self.iters()
    }

    fn step(&mut self) -> Duration;

    fn run(&mut self) {
        match self.size() {
            BenchSize::Iters(n) => {
                for _ in 0..n {
                    let elapsed = self.step();
                    *self.elapsed_mut() += elapsed;
                }
                *self.iters_mut() += n;
            }
            BenchSize::Time(t) => {
                while *self.elapsed() < t {
                    let elapsed = self.step();
                    *self.iters_mut() += 1;
                    *self.elapsed_mut() += elapsed;
                }
            }
        }
    }

    /// # Errors
    /// When `write_str` or `write_fmt` fails
    fn display(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())?;
        f.write_fmt(format_args!(": {:?}", self.elapsed()))?;
        f.write_fmt(format_args!(" / {}", self.iters()))?;
        f.write_fmt(format_args!(" = ± {:?}/iter", self.mean()))?;
        Ok(())
    }
}
