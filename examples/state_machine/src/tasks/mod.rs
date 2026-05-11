// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0
pub mod joystick;
pub mod led;
pub mod screen;

pub trait Task {
    fn poll(&mut self);
}
