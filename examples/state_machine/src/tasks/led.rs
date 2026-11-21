// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use super::Task;
use super::screen::LedAction;
use crate::channel::Receiver;

use embedded_hal::digital::OutputPin;
use embedtrait::timer::Timer;

#[derive(PartialEq, Debug, Clone, Copy)]
enum State {
    Blinking(bool),
    On,
    Off,
}

pub struct Led<'a, T, P> {
    timer: T,
    state: State,
    receiver: Receiver<'a, LedAction>,
    led: P,
}

impl<'a, T: Timer, P: OutputPin> Led<'a, T, P> {
    pub fn new(timer: T, led: P, receiver: Receiver<'a, LedAction>) -> Self {
        Self {
            timer,
            state: State::Blinking(true),
            receiver,
            led,
        }
    }
}

impl<'a, T: Timer, P: OutputPin> Task for Led<'a, T, P> {
    fn poll(&mut self) {
        self.state = match self.receiver.receive() {
            Some(LedAction::On) => State::On,
            Some(LedAction::Off) => State::Off,
            Some(LedAction::Blink) => State::Blinking(true),
            _ => self.state,
        };

        match self.state {
            State::On => {
                self.led.set_high().unwrap();
            }
            State::Off => {
                self.led.set_low().unwrap();
            }
            State::Blinking(on) => {
                if self.timer.is_ready() {
                    if on {
                        self.led.set_high().unwrap();
                    } else {
                        self.led.set_low().unwrap();
                    }
                    self.state = State::Blinking(!on);
                    self.timer.reload();
                }
            }
        }
    }
}
