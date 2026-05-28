// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

#![no_std]

pub mod embedded_hal;
pub mod gpio;
pub mod pinmux;
pub mod plic;
pub mod rv_core_ibex;
pub mod rv_timer;
pub mod spi_host;
pub mod uart;

pub use gpio::*;
pub use pinmux::*;
pub use plic::*;
pub use rv_core_ibex::*;
pub use rv_timer::*;
pub use spi_host::*;
pub use uart::*;

pub type HalResult<T> = Result<T, HalError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HalError {
    Spi(SpiError),
    ConfigInvalid,
    BadClock,
    BadSize,
    NotSupported,
    InvalidVariant,
}

impl From<SpiError> for HalError {
    fn from(err: SpiError) -> Self {
        HalError::Spi(err)
    }
}
