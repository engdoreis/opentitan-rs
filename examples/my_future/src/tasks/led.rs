// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use super::screen::LedAction;
use crate::channel::Receiver;

use embedded_hal::digital::OutputPin;

#[derive(PartialEq, Debug, Clone, Copy)]
enum State {
    Blinking(bool),
    On,
    Off,
}

pub struct Led<'a, P> {
    state: State,
    receiver: Receiver<'a, LedAction>,
    led: P,
}

impl<'a, P: OutputPin> Led<'a, P> {
    pub fn new(led: P, receiver: Receiver<'a, LedAction>) -> Self {
        Self {
            state: State::Blinking(true),
            receiver,
            led,
        }
    }
}

use crate::my_future::{Poll, ReisFuture};
impl<'a, P: OutputPin> ReisFuture for Led<'a, P> {
    type Output = ();
    fn poll(&mut self, id: usize) -> Poll<Self::Output> {
        // Register task id to enable the sender to wakeup task.
        self.state = match self.receiver.receive(id) {
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
                if on {
                    self.led.set_high().unwrap();
                } else {
                    self.led.set_low().unwrap();
                }
                self.state = State::Blinking(!on);
                return Poll::Sleep(400);
            }
        }
        Poll::Pending
    }
}
