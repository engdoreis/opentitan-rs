#![cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
#![no_main]
#![no_std]

extern crate panic_halt as _;

use core::fmt::Write;
use embedded_hal::delay::DelayNs;
use riscv::delay::McycleDelay;
use riscv_rt::entry;

const CPU_CLOCK_HZ: u32 = 500_000;
const PERIPHERAL_CLOCK_HZ: u32 = 125_000;
const BAUD: u32 = 7200;

#[entry]
fn main() -> ! {
    let mut delay = McycleDelay::new(CPU_CLOCK_HZ);
    let peri = zermio::earlgrey::Peripherals::take().unwrap();
    let mut uart = hal::uart::Uart::new(peri.uart0, PERIPHERAL_CLOCK_HZ, BAUD);

    loop {
        let _ = writeln!(uart, "Hello World!!");
        delay.delay_ms(50);
    }
}
