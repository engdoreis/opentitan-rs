// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

// use crate::test_status::TestStatus;
use bsp::earlgrey_es::*;
use bsp::prelude::*;
use riscv_rt::entry;
pub use test_utils_macros::test_main;

use core::fmt::Write;

#[entry]
fn main() -> ! {
    // let peri = unsafe { zermio::earlgrey::Peripherals::steal() };
    // let mut ibex = hal::rv_core_ibex::RvCoreIbex::new(peri.rv_core_ibex);
    // TestStatus::InTest.set(&mut ibex);

    unsafe extern "Rust" {
        pub fn test_entry() -> bool;
    }
    let pass = unsafe { test_entry() };

    let peri = unsafe { zermio::earlgrey::Peripherals::steal() };
    let mut uart = hal::uart::Uart::new(peri.uart0, Silicon::PERIPHERAL_CLOCK_HZ, Silicon::BAUD);

    // let mut ibex = hal::rv_core_ibex::RvCoreIbex::new(peri.rv_core_ibex);
    if pass {
        // TestStatus::Passed.set(&mut ibex);
        let _ = writeln!(uart, "TEST RESULT: PASS");
    } else {
        // TestStatus::Failed.set(&mut ibex);
        let _ = writeln!(uart, "TEST RESULT: FAIL");
    }
    loop {
        riscv::asm::wfi();
    }
}
