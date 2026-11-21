// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

pub struct RvTimer<const CLOCK: u64, const TICK_HZ: u64 = 1_000_000u64> {
    inner: RawTimer,
}

impl<const CLOCK: u64, const TICK_HZ: u64> RvTimer<CLOCK, TICK_HZ> {
    #[inline]
    pub fn new(regs: zermio::rv_timer::RvTimer) -> Self {
        let mut inner = RawTimer { regs };
        inner.stop();
        inner.set_interrupt(false);
        let prescale: u32 = ((CLOCK + TICK_HZ) / TICK_HZ) as u32;
        inner.configure_tick(prescale);
        inner.start();
        Self { inner }
    }

    pub fn new_timer(&self) -> Timer<'_, CLOCK, TICK_HZ> {
        Timer::new(self)
    }

    pub fn into_raw(self) -> RawTimer {
        self.inner
    }
}

use core::ops::{Deref, DerefMut};

impl<const CLOCK: u64, const TICK_HZ: u64> Deref for RvTimer<CLOCK, TICK_HZ> {
    type Target = RawTimer;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<const CLOCK: u64, const TICK_HZ: u64> DerefMut for RvTimer<CLOCK, TICK_HZ> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub struct RawTimer {
    regs: zermio::rv_timer::RvTimer,
}

impl RawTimer {
    fn configure_tick(&mut self, prescale: u32) {
        self.regs.cfg0.write(|cfg| {
            cfg.prescale().write(prescale - 1);
            cfg.step().write(1);
        });
    }

    pub fn clear_interrupt(&mut self) {
        self.regs.intr_state0.is().assign(true).commit();
    }

    pub fn set_interrupt(&mut self, enable: bool) {
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

    pub fn get_ticks(&self) -> u64 {
        loop {
            let hi = self.regs.timer_v_upper0.fetch().v().get() as u64;
            let lo = self.regs.timer_v_lower0.fetch().v().get() as u64;
            if hi == self.regs.timer_v_upper0.fetch().v().get() as u64 {
                return hi << 32 | lo;
            }
        }
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

// ==============Virtual Timer==========================================

type Duration64<const TICK_HZ: u64> = fugit::Duration<u64, 1, TICK_HZ>;
type Instant64<const TICK_HZ: u64> = fugit::Instant<u64, 1, TICK_HZ>;

pub struct Timer<'a, const CLOCK: u64, const TICK_HZ: u64> {
    inner: &'a RvTimer<CLOCK, TICK_HZ>,
    duration: Duration64<TICK_HZ>,
    start_time: Instant64<TICK_HZ>,
    pub end_time: Instant64<TICK_HZ>,
}

impl<'a, const CLOCK: u64, const TICK_HZ: u64> Timer<'a, CLOCK, TICK_HZ> {
    fn new(timer: &'a RvTimer<CLOCK, TICK_HZ>) -> Self {
        let now = Instant64::from_ticks(0);
        Timer {
            inner: timer,
            start_time: now,
            end_time: now,
            duration: Duration64::from_ticks(0),
        }
    }
}

use embedtrait::timer;
impl<const CLOCK: u64, const TICK_HZ: u64> timer::Timer for Timer<'_, CLOCK, TICK_HZ> {
    type Duration = Duration64<TICK_HZ>;
    type Instant = Instant64<TICK_HZ>;

    fn start<T>(&mut self, duration: T)
    where
        T: Into<Self::Duration>,
    {
        self.duration = duration.into();
        self.reload();
    }

    fn reload(&mut self) {
        self.start_time = self.now();
        self.end_time = self.start_time + self.duration;
    }

    fn elapsed(&self) -> Self::Duration {
        self.now() - self.start_time
    }

    fn is_ready(&self) -> bool {
        self.end_time <= self.now()
    }

    fn wait_for_ready(&self) {
        while !self.is_ready() {}
    }

    fn now(&self) -> Self::Instant {
        Self::Instant::from_ticks(self.inner.get_ticks())
    }
}
