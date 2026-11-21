// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use crate::gpio::pin::{self, Pin};
use crate::gpio::static_pin::StaticPin;
use embedded_hal::digital as embedded_hal;

impl<const N: usize, T> embedded_hal::ErrorType for StaticPin<N, T> {
    type Error = crate::HalError;
}

impl<const N: usize> embedded_hal::OutputPin for StaticPin<N, pin::OutputState> {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.write(false);
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.write(true);
        Ok(())
    }

    fn set_state(&mut self, state: embedded_hal::PinState) -> Result<(), Self::Error> {
        self.write(match state {
            embedded_hal::PinState::Low => false,
            embedded_hal::PinState::High => true,
        });
        Ok(())
    }
}

impl<const N: usize> embedded_hal::InputPin for StaticPin<N, pin::InputState> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.read())
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(!self.read())
    }
}
