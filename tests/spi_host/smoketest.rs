// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

#![cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
#![no_main]
#![no_std]

extern crate test_utils as _;
use bsp::Device;
use core::fmt::Write;
use embedtrait::spi_host::{Data, Operation, SpiBus, Width};
use test_utils::test_main;

#[test_main]
fn test_main() -> bool {
    let peri = unsafe { zermio::earlgrey::Peripherals::steal() };
    let mut uart = hal::uart::Uart::new(
        peri.uart0,
        bsp::Silicon::PERIPHERAL_CLOCK_HZ,
        bsp::Silicon::BAUD,
    );

    let _ = writeln!(uart, "Spi test");

    let config = hal::spi_host::SpiHostConfig {
        speed: 10000,
        clock_freq_hz: bsp::Silicon::HI_SPEED_PERIPHERAL_CLOCK_HZ,
        chip_select: hal::spi_host::ChipSelectTiming {
            idle: 1,
            trail: 1,
            lead: 1,
        },
        full_cycle: false,
        cpha: false,
        cpol: false,
        tx_watermark: 5,
        rx_watermark: 5,
    };

    let mut spih = hal::spi_host::SpiHost::new(peri.spi_host0);
    spih.configure(config)
        .unwrap_or_else(|e| {
            panic!("error to configure: {e:?}");
        })
        .enable(true)
        .enable_output(true);
    let _ = writeln!(uart, "Spi ok");

    let _ = spih
        .transfer(&mut [Operation {
            data: Data::Write(&[0xbe, 0xba, 0xca, 0xfe, 0x55]),
            width: Width::Single,
        }])
        .unwrap_or_else(|e| {
            panic!("Error in transaction: {e:?}");
        });

    let _ = writeln!(uart, "finished");
    true
}
