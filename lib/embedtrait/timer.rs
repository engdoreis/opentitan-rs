// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

pub trait Timer {
    type Duration;
    type Instant;

    fn start<T>(&mut self, time: T)
    where
        T: Into<Self::Duration>;
    fn reload(&mut self);
    fn elapsed(&self) -> Self::Duration;
    fn is_ready(&self) -> bool;
    fn wait_for_ready(&self);
    fn now(&self) -> Self::Instant;
}
