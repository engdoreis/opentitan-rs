// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

pub trait ReisFuture {
    type Output;
    fn poll(&mut self, id: usize) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Pending,
    Sleep(u64),
    Ready(T),
}
