// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

#![cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
#![no_main]
#![no_std]

extern crate test_utils as _;
use core::fmt::Write;
use embedded_hal::delay::DelayNs;
use riscv::delay::McycleDelay;
use test_utils::abort;
use test_utils::test_framework;
use test_utils::test_main;

#[test_main]
fn test_main() -> bool {
    let peri = unsafe { zermio::earlgrey::Peripherals::steal() };
    let mut uart = hal::uart::Uart::new(
        peri.uart0,
        test_framework::PERIPHERAL_CLOCK_HZ,
        test_framework::BAUD,
    );

    let _ = writeln!(uart, "Timer smoketest");
    let mut delay = McycleDelay::new(test_framework::CPU_CLOCK_HZ);
    let mut timer = hal::rv_timer::RvTimer::new(peri.timer, test_framework::PERIPHERAL_CLOCK_HZ);

    timer.start();
    let start = timer.get_time_ms();
    delay.delay_ms(20);
    let end = timer.get_time_ms();
    let _ = writeln!(uart, "elapsed {} ms", end);
    let (prescale, step) = timer.get_prescale();
    let _ = writeln!(uart, "prescale: {}, step: {}", prescale, step);
    start - end >= 20
}
