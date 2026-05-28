// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use core::result::Result;

pub trait PinMuxNumber {
    type Error;
    fn number(&self) -> usize;
    fn from_number(value: usize) -> Result<Self, Self::Error>
    where
        Self: Sized;
}
