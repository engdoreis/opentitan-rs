// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

#![no_std]

pub mod pinmux;
pub mod rv_core_ibex;
pub mod rv_timer;
pub mod spi_host;
pub mod uart;

use spi_host::SpiError;

pub type HalResult<T> = Result<T, HalError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HalError {
    Spi(SpiError),
    ConfigInvalid,
    BadClock,
}

impl From<SpiError> for HalError {
    fn from(err: SpiError) -> Self {
        HalError::Spi(err)
    }
}
