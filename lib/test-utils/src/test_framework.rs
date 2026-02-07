#![cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]

extern crate panic_halt as _;

use crate::test_status::TestStatus;
use embedded_hal::delay::DelayNs;
use riscv::delay::McycleDelay;
use riscv_rt::entry;
pub use test_utils_macros::test_main;

extern crate manifest as _;

pub const CPU_CLOCK_HZ: u32 = 500_000;
pub const PERIPHERAL_CLOCK_HZ: u32 = 125_000;
pub const HI_SPEED_PERIPHERAL_CLOCK_HZ: u32 = 500_000;
pub const BAUD: u32 = 7200;

#[entry]
fn main() -> ! {
    let peri = unsafe { zermio::earlgrey::Peripherals::steal() };
    let mut ibex = hal::rv_core_ibex::RvCoreIbex::new(peri.rv_core_ibex);
    TestStatus::InTest.set(&mut ibex);

    unsafe extern "Rust" {
        pub fn test_entry() -> bool;
    }
    let pass = unsafe { test_entry() };

    let peri = unsafe { zermio::earlgrey::Peripherals::steal() };
    let mut ibex = hal::rv_core_ibex::RvCoreIbex::new(peri.rv_core_ibex);

    let mut delay = McycleDelay::new(CPU_CLOCK_HZ);
    delay.delay_ms(50);
    if pass {
        TestStatus::Passed.set(&mut ibex);
    } else {
        TestStatus::Failed.set(&mut ibex);
    }
    loop {
        riscv::asm::wfi();
    }
}

pub fn abort() -> ! {
    let peri = unsafe { zermio::earlgrey::Peripherals::steal() };
    let mut ibex = hal::rv_core_ibex::RvCoreIbex::new(peri.rv_core_ibex);
    loop {
        TestStatus::Failed.set(&mut ibex);
        riscv::asm::wfi();
    }
}
