// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

#![no_std]

//export
// pub use abort as _;
extern crate manifest as _;

pub trait Device {
    const CPU_CLOCK_HZ: u32 = 500_000;
    const PERIPHERAL_CLOCK_HZ: u32 = 125_000;
    const HI_SPEED_PERIPHERAL_CLOCK_HZ: u32 = 500_000;
    const BAUD: u32 = 7200;
}

pub struct Verilator;
impl Device for Verilator {}

pub struct Silicon;
impl Device for Silicon {
    const CPU_CLOCK_HZ: u32 = 100_000_000;
    const PERIPHERAL_CLOCK_HZ: u32 = 24_000_000;
    const HI_SPEED_PERIPHERAL_CLOCK_HZ: u32 = 96_000_000;
    const BAUD: u32 = 115200;
}

use core::fmt::Write;
use core::panic::PanicInfo;

pub fn abort() -> ! {
    panic!("Aborting");
}

#[panic_handler]
fn custom_panic_handler(info: &PanicInfo) -> ! {
    let peri = unsafe { zermio::earlgrey::Peripherals::steal() };
    let mut uart = hal::uart::Uart::new(peri.uart0, Silicon::PERIPHERAL_CLOCK_HZ, Silicon::BAUD);
    if let Some(location) = info.location() {
        let _ = writeln!(
            uart,
            "{}, Panicked at {}:{}",
            info.message(),
            location.file(),
            location.line()
        );
    }

    loop {
        riscv::asm::wfi();
    }
}
