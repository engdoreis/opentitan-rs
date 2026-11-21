// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use super::Task;
use crate::channel::Sender;
use bsp::voyager::{self, Direction};

use embedtrait::timer::Timer;

#[derive(PartialEq, Debug, Clone, Copy)]
enum State {
    WaitForPress,
    Debounce(Direction),
    WaitForRelease(Direction),
}

pub struct Button<'a, T> {
    timer: T,
    js: voyager::Joystick,
    state: State,
    sender: Sender<'a, Direction>,
}

impl<'a, T: Timer> Button<'a, T> {
    pub fn new(timer: T, js: voyager::Joystick, sender: Sender<'a, Direction>) -> Self {
        Self {
            timer,
            js,
            state: State::WaitForPress,
            sender,
        }
    }
}

impl<'a, T: Timer> Task for Button<'a, T> {
    fn poll(&mut self) {
        match self.state {
            State::WaitForPress => {
                if let Some(direction) = self.js.scan() {
                    self.state = State::Debounce(direction);
                    self.timer.reload()
                }
            }
            State::Debounce(direction) => {
                if self.timer.is_ready() {
                    if let Some(new_direction) = self.js.scan()
                        && direction == new_direction
                    {
                        self.sender.send(direction);
                        self.state = State::WaitForRelease(direction);
                        return;
                    }
                    self.state = State::WaitForPress;
                }
            }
            State::WaitForRelease(direction) => {
                if let Some(new_direction) = self.js.scan()
                    && direction == new_direction
                {
                    return;
                }
                self.state = State::WaitForPress;
            }
        }
    }
}
