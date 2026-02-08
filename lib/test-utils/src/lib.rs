// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

#![no_std]

pub mod test_framework;
pub mod test_status;

// Re-export
pub use test_framework::abort;
pub use test_utils_macros::test_main;
