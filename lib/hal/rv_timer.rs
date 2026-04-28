// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

pub struct RvTimer {
    regs: zermio::rv_timer::RvTimer,
    clock: u64,
}

impl RvTimer {
    #[inline]
    pub fn new(regs: zermio::rv_timer::RvTimer, clock: u32) -> Self {
        let mut this = RvTimer {
            regs,
            clock: clock as u64,
        };
        this.stop();
        this.set_interrupt(false);
        this.configure_tick();
        this
    }

    fn configure_tick(&mut self) {
        let target_tick_hz = 1_000_000u64; // 1us
        let prescale: u32 = ((self.clock + target_tick_hz) / target_tick_hz) as u32;
        self.regs.cfg0.write(|cfg| {
            cfg.prescale().write(prescale - 1);
            cfg.step().write(1);
        });
    }

    fn set_interrupt(&mut self, enable: bool) {
        self.regs.intr_enable0.ie().assign(enable).commit();
    }

    pub fn start(&mut self) {
        self.regs.ctrl.active().set().commit();
    }

    pub fn stop(&mut self) {
        self.regs.ctrl.active().reset().commit();
    }

    pub fn is_running(&mut self) -> bool {
        self.regs.ctrl.fetch().active().is_set()
    }

    pub fn get_prescale(&mut self) -> (u32, u32) {
        (
            self.regs.cfg0.fetch().prescale().get(),
            self.regs.cfg0.step().get(),
        )
    }

    pub fn get_ticks(&mut self) -> u64 {
        loop {
            let hi = self.regs.timer_v_upper0.fetch().v().get() as u64;
            let lo = self.regs.timer_v_lower0.fetch().v().get() as u64;
            if hi == self.regs.timer_v_upper0.fetch().v().get() as u64 {
                return hi << 32 | lo;
            }
        }
    }

    pub fn get_time_ms(&mut self) -> u64 {
        let (prescale, _) = self.get_prescale();
        (self.get_ticks() * (prescale + 1) as u64 * 1000u64) / self.clock
    }

    pub fn set_counter(&mut self, counter: u64) {
        self.regs.timer_v_lower0.v().write(0).commit();
        let hi = (counter >> 32) as u32;
        self.regs.timer_v_upper0.v().write(hi).commit();
        self.regs.timer_v_lower0.v().write(counter as u32).commit();
    }

    pub fn get_compare(&mut self) -> u64 {
        let mut hi;
        let mut lo;

        loop {
            lo = self.regs.compare_lower0_0.fetch().v().get() as u64;
            hi = self.regs.compare_upper0_0.fetch().v().get() as u64;
            if hi == self.regs.compare_upper0_0.fetch().v().get() as u64 {
                break;
            }
        }

        hi << 32 | lo
    }

    pub fn set_compare(&mut self, counter: u64) {
        let hi = (counter >> 32) as u32;
        let lo = counter as u32;
        self.regs.compare_upper0_0.v().write(hi).commit();
        self.regs.compare_lower0_0.v().write(lo).commit();
    }
}
