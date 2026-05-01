// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

pub struct Gpio {
    regs: zermio::gpio::Gpio,
}

impl Gpio {
    #[inline]
    pub fn new(regs: zermio::gpio::Gpio) -> Self {
        Self { regs }
    }

    #[inline]
    pub fn output_en(&mut self, pin: usize, en: bool) -> &mut Self {
        self.regs.direct_oe.update(|reg| {
            let mask = 0x01 << pin;
            let val = reg.direct_oe().get() & !mask;
            reg.direct_oe().write(val | (en as u32) << pin);
        });
        self
    }

    #[inline]
    pub fn pin_write(&mut self, pin: usize, set: bool) -> &mut Self {
        self.regs.direct_out.update(|reg| {
            let mask = 0x01 << pin;
            let val = reg.direct_out().get() & !mask;
            reg.direct_out().write(val | (set as u32) << pin);
        });
        self
    }

    #[inline]
    pub fn pin_read(&mut self, pin: usize) -> bool {
        let mask = 0x01 << pin;
        (self.regs.data_in.fetch().data_in().get() & mask) == mask
    }
}
