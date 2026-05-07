// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0
use crate::chip::*;

pub struct Verilator;
impl Clocks for Verilator {}

pub struct Silicon;
impl Clocks for Silicon {
    const CPU_CLOCK_HZ: u32 = 100_000_000;
    const PERIPHERAL_CLOCK_HZ: u32 = 24_000_000;
    const HI_SPEED_PERIPHERAL_CLOCK_HZ: u32 = 96_000_000;
    const BAUD: u32 = 115200;
}

pub struct EarlgreyEs;
impl PeripheralInput for EarlgreyEs {}
impl PeripheralOutput for EarlgreyEs {}

impl PadInput for EarlgreyEs {}
impl PadOutput for EarlgreyEs {}
