// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

#![cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
#![no_main]
#![no_std]

use bsp::earlgrey_es::*;
use bsp::prelude::*;

use core::fmt::Write;
use embedded_hal::delay::DelayNs;
use riscv::delay::McycleDelay;
use riscv_rt::entry;

extern crate manifest as _;

#[entry]
fn main() -> ! {
    let mut delay = McycleDelay::new(Silicon::CPU_CLOCK_HZ);
    let peri = zermio::earlgrey::Peripherals::take().unwrap();
    let mut uart = hal::uart::Uart::new(peri.uart0, Silicon::PERIPHERAL_CLOCK_HZ, Silicon::BAUD);

    loop {
        let _ = writeln!(uart, "Hello World!!");
        delay.delay_ms(50);
    }
}
