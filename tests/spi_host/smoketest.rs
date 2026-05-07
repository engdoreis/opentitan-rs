// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

#![cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
#![no_main]
#![no_std]

extern crate test_utils as _;
use bsp::earlgrey_es::*;
use bsp::prelude::*;
use core::fmt::Write;
use embedtrait::spi_host::{Data, Operation, SpiBus, Width};
use test_utils::test_main;

#[test_main]
fn test_main() -> bool {
    let peri = unsafe { zermio::earlgrey::Peripherals::steal() };
    let mut uart = hal::uart::Uart::new(peri.uart0, Silicon::PERIPHERAL_CLOCK_HZ, Silicon::BAUD);

    let _ = writeln!(uart, "Spi test");

    let config = hal::spi_host::Config {
        speed: 1_000_000,
        clock_freq_hz: Silicon::HI_SPEED_PERIPHERAL_CLOCK_HZ,
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

    let mut res = jedec_test(&mut uart, &mut spih);
    res &= page_test(&mut uart, &mut spih);

    let _ = writeln!(uart, "finished");
    res
}

fn jedec_test(uart: &mut hal::uart::Uart, spih: &mut hal::spi_host::SpiHost) -> bool {
    let mut jedec_id = [0x00; 4];
    let _ = spih
        .transfer(&mut [
            Operation {
                data: Data::Write(&[0x9f]),
                width: Width::Single,
            },
            Operation {
                data: Data::Read(&mut jedec_id),
                width: Width::Single,
            },
        ])
        .unwrap_or_else(|e| {
            panic!("Error in transaction: {e:?}");
        });
    let _ = writeln!(uart, "jedec id: {:?}", jedec_id);

    jedec_id == [0xef, 0x40, 0x18, 0x00]
}

fn page_test(uart: &mut hal::uart::Uart, spih: &mut hal::spi_host::SpiHost) -> bool {
    let page = [0x55; 257];
    let mut read = [0x00; 256];

    let _ = spih
        .transfer(&mut [Operation {
            data: Data::Write(&[0x06]),
            width: Width::Single,
        }])
        .unwrap_or_else(|e| {
            panic!("Error in transaction: {e:?}");
        });

    let _ = spih
        .transfer(&mut [Operation {
            data: Data::Write(&[0x20, 0x00, 0x00, 0x00]),
            width: Width::Single,
        }])
        .unwrap_or_else(|e| {
            panic!("Error in transaction: {e:?}");
        });

    let mut status = [1u8, 1];
    while status[0] != 0 {
        let _ = spih
            .transfer(&mut [
                Operation {
                    data: Data::Write(&[0x05]),
                    width: Width::Single,
                },
                Operation {
                    data: Data::Read(&mut status),
                    width: Width::Single,
                },
            ])
            .unwrap_or_else(|e| {
                panic!("Error in transaction: {e:?}");
            });
    }

    let _ = spih
        .transfer(&mut [Operation {
            data: Data::Write(&[0x06]),
            width: Width::Single,
        }])
        .unwrap_or_else(|e| {
            panic!("Error in transaction: {e:?}");
        });

    let _ = spih
        .transfer(&mut [
            Operation {
                data: Data::Write(&[0x02, 0x00, 0x00, 0x00]),
                width: Width::Single,
            },
            Operation {
                data: Data::Write(&page[1..]),
                width: Width::Single,
            },
        ])
        .unwrap_or_else(|e| {
            panic!("Error in transaction: {e:?}");
        });

    let mut status = [1u8, 1];
    while status[0] != 0 {
        let _ = spih
            .transfer(&mut [
                Operation {
                    data: Data::Write(&[0x05]),
                    width: Width::Single,
                },
                Operation {
                    data: Data::Read(&mut status),
                    width: Width::Single,
                },
            ])
            .unwrap_or_else(|e| {
                panic!("Error in transaction: {e:?}");
            });
    }

    let _ = spih
        .transfer(&mut [
            Operation {
                data: Data::Write(&[0x03, 0x00, 0x00, 0x00]),
                width: Width::Single,
            },
            Operation {
                data: Data::Read(&mut read),
                width: Width::Single,
            },
        ])
        .unwrap_or_else(|e| {
            panic!("Error in transaction: {e:?}");
        });
    let _ = writeln!(uart, "page: {:?}", read);

    read == &page[1..]
}
