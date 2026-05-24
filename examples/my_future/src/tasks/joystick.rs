// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use crate::channel::Sender;
use bsp::earlgrey_es::*;
use bsp::prelude::*;
use bsp::voyager::{self, Direction};
use core::fmt::Write;

use core::sync::atomic::AtomicUsize;
const INVALID_TASK_ID: usize = 0xFF;
const DEFAULT_TASK_ID: AtomicUsize = AtomicUsize::new(INVALID_TASK_ID);
static TASK_ID: AtomicUsize = DEFAULT_TASK_ID;

#[derive(PartialEq, Debug, Clone, Copy)]
enum State {
    WaitForPress,
    Debounce(Direction),
    WaitForRelease(Direction),
}

pub struct Button<'a> {
    js: voyager::Joystick,
    state: State,
    sender: Sender<'a, Direction>,
}

impl<'a> Button<'a> {
    pub fn new(js: voyager::Joystick, sender: Sender<'a, Direction>) -> Self {
        Self {
            js,
            state: State::WaitForPress,
            sender,
        }
    }
}

use crate::my_future::{Poll, ReisFuture};
impl<'a> ReisFuture for Button<'a> {
    type Output = ();
    fn poll(&mut self, id: usize) -> Poll<Self::Output> {
        loop {
            match self.state {
                State::WaitForPress => {
                    if let Some(direction) = self.js.scan() {
                        self.state = State::Debounce(direction);
                        return Poll::Sleep(25);
                    }
                    break;
                }
                State::Debounce(direction) => {
                    if let Some(new_direction) = self.js.scan()
                        && direction == new_direction
                    {
                        self.sender.send(direction);
                        self.state = State::WaitForRelease(direction);
                        continue;
                    }
                    self.state = State::WaitForPress;
                }
                State::WaitForRelease(direction) => {
                    if let Some(new_direction) = self.js.scan()
                        && direction == new_direction
                    {
                        break;
                    }
                    self.state = State::WaitForPress;
                }
            }
        }
        TASK_ID.store(id, core::sync::atomic::Ordering::Relaxed);
        Poll::Pending
    }
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

fn joystick_handler(pin: usize) {
    let peri = unsafe { zermio::earlgrey::Peripherals::steal() };
    let mut uart = hal::uart::Uart::new(peri.uart0, Silicon::PERIPHERAL_CLOCK_HZ, Silicon::BAUD);

    let mut gpio = hal::Gpio::new(peri.gpio);
    let _ = writeln!(uart, "interrupt gpio {}", pin);
    gpio.interrupt_clear(pin);

    crate::my_executor::wake_task(TASK_ID.load(core::sync::atomic::Ordering::Relaxed));
    riscv::interrupt::disable();
}
