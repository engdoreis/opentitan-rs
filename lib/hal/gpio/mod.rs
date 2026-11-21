// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

pub mod pin;
pub mod static_pin;

pub enum InterruptMode {
    LevelLow,
    LevelHigh,
    EdgeRising,
    EdgeFalling,
}

pub struct Gpio {
    regs: zermio::gpio::Gpio,
}

impl Gpio {
    #[inline]
    pub fn new(regs: zermio::gpio::Gpio) -> Self {
        Self { regs }
    }

    #[inline]
    pub fn from_addr(addr: usize) -> Self {
        Self::new(zermio::gpio::Gpio::new(addr as u32))
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

    #[inline]
    pub fn interrupt_set_enable(&mut self, pin: usize, en: bool) -> &mut Self {
        self.regs.intr_enable.update(|reg| {
            let mask = 0x01 << pin;
            let val = reg.gpio().get() & !mask;
            reg.gpio().write(val | (en as u32) << pin);
        });
        self
    }

    #[inline]
    pub fn interrupt_pending(&mut self, pin: usize) -> bool {
        let mask = 0x01 << pin;
        self.regs.intr_state.fetch().gpio().get() & mask == mask
    }

    #[inline]
    pub fn interrupt_clear(&mut self, pin: usize) -> &mut Self {
        let mask = 0x01 << pin;
        self.regs.intr_state.gpio().write(mask).commit();
        self
    }

    #[inline]
    pub fn interrupt_force(&mut self, pin: usize) -> &mut Self {
        self.regs.intr_test.write(|reg| {
            let mask = 0x01 << pin;
            reg.gpio().write(mask);
        });
        self
    }

    #[inline]
    pub fn interrupt_set_mode(
        &mut self,
        pin: usize,
        en: bool,
        modes: &[InterruptMode],
    ) -> &mut Self {
        for mode in modes {
            match mode {
                InterruptMode::LevelLow => {
                    self.regs.intr_ctrl_en_lvllow.update(|reg| {
                        let mask = 0x01 << pin;
                        let val = reg.intr_ctrl_en_lvllow().get() & !mask;
                        reg.intr_ctrl_en_lvllow().write(val | (en as u32) << pin);
                    });
                }
                InterruptMode::LevelHigh => {
                    self.regs.intr_ctrl_en_lvlhigh.update(|reg| {
                        let mask = 0x01 << pin;
                        let val = reg.intr_ctrl_en_lvlhigh().get() & !mask;
                        reg.intr_ctrl_en_lvlhigh().write(val | (en as u32) << pin);
                    });
                }
                InterruptMode::EdgeFalling => {
                    self.regs.intr_ctrl_en_falling.update(|reg| {
                        let mask = 0x01 << pin;
                        let val = reg.intr_ctrl_en_falling().get() & !mask;
                        reg.intr_ctrl_en_falling().write(val | (en as u32) << pin);
                    });
                }
                InterruptMode::EdgeRising => {
                    self.regs.intr_ctrl_en_rising.update(|reg| {
                        let mask = 0x01 << pin;
                        let val = reg.intr_ctrl_en_rising().get() & !mask;
                        reg.intr_ctrl_en_rising().write(val | (en as u32) << pin);
                    });
                }
            }
        }
        self
    }
}

impl pin::GpioExt for zermio::gpio::Gpio {
    type Pins = static_pin::GpioPins;
    fn pins(self) -> Self::Pins {
        Self::Pins::new(self.destroy() as usize)
    }
}
