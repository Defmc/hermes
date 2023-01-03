use std::{
    fmt,
    hint::black_box,
    time::{Duration, Instant},
};

pub enum BenchSize {
    Iters(usize),
    Time(Duration),
}

pub struct Bench<'a, T, U> {
    name: String,
    size: BenchSize,
    test: &'a dyn Fn(T) -> U,
    setup: &'a dyn Fn() -> T,
    post: &'a dyn Fn(U),
    dur: Duration,
    iters: usize,
}

impl<'a, T, U> fmt::Display for Bench<'a, T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)?;
        f.write_fmt(format_args!(": {:?}", self.dur))?;
        f.write_fmt(format_args!(" / {}", self.iters))?;
        f.write_fmt(format_args!(" = ± {:?}/iter", self.mean()))?;
        Ok(())
    }
}

impl<'a, T, U> Bench<'a, T, U> {
    #[must_use]
    pub fn new(setup: &'a impl Fn() -> T, test: &'a impl Fn(T) -> U) -> Bench<'a, T, U> {
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
    pub fn name(mut self, name: impl AsRef<str>) -> Self {
        self.name = name.as_ref().to_string();
        self
    }

    #[must_use]
    pub fn size(mut self, size: BenchSize) -> Self {
        self.size = size;
        self
    }

    #[must_use]
    pub fn post(mut self, post: &'a impl Fn(U)) -> Self {
        self.post = post;
        self
    }

    pub fn run(&mut self) {
        match self.size {
            BenchSize::Iters(n) => {
                for _ in 0..n {
                    self.step();
                }
                self.iters = n;
            }
            BenchSize::Time(dur) => {
                while self.dur < dur {
                    self.step();
                    self.iters += 1;
                }
            }
        }
    }

    pub fn step(&mut self) {
        let setup = black_box((self.setup)());
        let start = Instant::now();
        let res = (self.test)(setup);
        let elapsed = start.elapsed();
        (self.post)(res);
        self.dur += elapsed;
    }

    #[must_use]
    pub fn mean(&self) -> Duration {
        self.dur / self.iters.max(1) as u32
    }
}

impl<'a, U> Bench<'a, (), U> {
    #[must_use]
    pub fn no_setup(test: &'a impl Fn(()) -> U) -> Self {
        Self::new(&|| (), test)
    }
}
