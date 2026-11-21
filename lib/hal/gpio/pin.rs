// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

// Define the basic operations of a pin.
pub trait Pin: Sized {
    fn id(&self) -> usize;

    fn port(&self) -> crate::Gpio;

    #[inline(always)]
    fn write(&self, val: bool) {
        self.port().pin_write(self.id(), val);
    }

    #[inline(always)]
    fn read(&self) -> bool {
        self.port().pin_read(self.id())
    }
}

pub trait GpioExt: Sized {
    type Pins;
    fn pins(self) -> Self::Pins;
    fn split(self) -> Self::Pins {
        self.pins()
    }
}

// Used for the pin type state, to check if the pin is Out or In at compile time.
pub struct OutputState;
pub struct InputState;
