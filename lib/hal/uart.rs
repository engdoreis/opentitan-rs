use core::fmt;

pub struct Uart {
    regs: zermio::uart::Uart,
}

impl Uart {
    #[inline]
    pub fn new(mut regs: zermio::uart::Uart, cpu_clock_hz: u32, baud: u32) -> Self {
        let nco = ((baud as u64) << 20) / cpu_clock_hz as u64;
        regs.ctrl.write(|ctrl| {
            ctrl.nco().write(nco as u32);
            ctrl.tx().set();
            ctrl.rx().set();
        });
        Self { regs }
    }

    #[inline]
    pub fn write(&mut self, buf: &[u8]) {
        for c in buf {
            while self.regs.status.fetch().txfull().is_set() {}
            self.regs.wdata.wdata().write((*c).into()).commit();
        }
    }
}

// Implementing this trait will allow we use the `writeln!` macro to format log messages.
impl core::fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s.as_bytes());
        Ok(())
    }
}
