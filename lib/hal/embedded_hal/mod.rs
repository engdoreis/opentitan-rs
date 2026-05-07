// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

/// This module implements the crate embedded hal traits.
pub mod gpio;
use crate::HalError;

impl embedded_hal::digital::Error for HalError {
    fn kind(&self) -> embedded_hal::digital::ErrorKind {
        embedded_hal::digital::ErrorKind::Other
    }
}
