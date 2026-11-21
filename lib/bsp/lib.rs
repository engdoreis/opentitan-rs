// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0
#![feature(linkage)]
#![no_std]

pub mod earlgrey_es;
pub mod voyager;

pub mod chip;

pub use chip::*;

pub use earlgrey_es::*;
pub use voyager::*;

pub mod prelude {
    pub use crate::Clocks;
    pub use crate::PadInput;
    pub use crate::PadOutput;
    pub use crate::PeripheralInput;
    pub use crate::PeripheralOutput;
    pub use crate::earlgrey_es::PlicIrqId;
}

extern crate manifest as _;

use core::fmt::Write;
use core::panic::PanicInfo;
pub use earlgrey_es::Silicon;

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

    let _ = writeln!(uart, "looping",);
    loop {
        riscv::asm::wfi();
    }
}
