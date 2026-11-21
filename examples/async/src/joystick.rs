// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use bsp::earlgrey_es::*;
use bsp::voyager;

use core::{
    cell::RefCell,
    task::{Poll, Waker},
};

use critical_section::Mutex;
pub static WAKER: Mutex<RefCell<Option<Waker>>> = Mutex::new(RefCell::new(None));

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Event {
    Press,
    Release(voyager::Direction),
}

pub struct Button {
    js: voyager::Joystick,
}

impl Button {
    pub fn new(js: voyager::Joystick) -> Self {
        Self { js }
    }

    pub async fn wait_for_event(&mut self, evt: Event) -> voyager::Direction {
        core::future::poll_fn(|ctx| {
            match evt {
                Event::Press => {
                    if let Some(direction) = self.js.scan() {
                        return Poll::Ready(direction);
                    }
                }
                Event::Release(dir) => {
                    let Some(direction) = self.js.scan() else {
                        return Poll::Ready(dir);
                    };
                    if dir != direction {
                        return Poll::Ready(dir);
                    }
                }
            }
            critical_section::with(|cs| {
                WAKER.borrow(cs).replace(Some(ctx.waker().clone()));
            });
            Poll::Pending
        })
        .await
    }
}

fn joystick_handler(pin: usize) {
    let peri = unsafe { zermio::earlgrey::Peripherals::steal() };
    let mut gpio = hal::Gpio::new(peri.gpio);

    critical_section::with(|cs| {
        gpio.interrupt_clear(pin);
        if let Some(waker) = WAKER.borrow(cs).borrow_mut().as_ref() {
            waker.wake_by_ref();
        }
    });
}

#[riscv_rt::external_interrupt(PlicIrqId::Gpio5)]
fn joystick_up_irs() {
    joystick_handler(5);
}

#[riscv_rt::external_interrupt(PlicIrqId::Gpio6)]
fn joystick_down_irs() {
    joystick_handler(6);
}

#[riscv_rt::external_interrupt(PlicIrqId::Gpio7)]
fn joystick_left_irs() {
    joystick_handler(7);
}

#[riscv_rt::external_interrupt(PlicIrqId::Gpio8)]
fn joystick_right_irs() {
    joystick_handler(8);
}

#[riscv_rt::external_interrupt(PlicIrqId::Gpio9)]
fn joystick_ok_irs() {
    joystick_handler(9);
}
