pub struct Benchmark<'a, T> {
    start: u64,
    clock_hz: u64,
    log: &'a core::cell::RefCell<T>,
}

impl<'a, T: core::fmt::Write> Benchmark<'a, T> {
    pub fn new(log: &'a core::cell::RefCell<T>, clock_hz: u64) -> Self {
        Self {
            start: 0,
            clock_hz,
            log,
        }
    }

    pub fn start(&mut self) {
        self.start = riscv::register::mcycle::read64();
    }

    pub fn finish(&mut self, msg: &str) {
        let cycles = riscv::register::mcycle::read64().wrapping_sub(self.start);
        let time = cycles * 1000 / self.clock_hz;
        let _ = writeln!(
            self.log.borrow_mut(),
            "{} in {} cycles, or {} ms @ {} Mhz",
            msg,
            cycles,
            time,
            self.clock_hz / 1_000_000
        );
    }
}
