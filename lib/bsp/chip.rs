// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

pub trait Clocks {
    const CPU_CLOCK_HZ: u32 = 500_000;
    const PERIPHERAL_CLOCK_HZ: u32 = 125_000;
    const HI_SPEED_PERIPHERAL_CLOCK_HZ: u32 = 500_000;
    const BAUD: u32 = 7200;
}
