// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

#![feature(exit_status_error)]

use clap::Parser;

pub mod command;
use crate::command::Command;

fn main() -> anyhow::Result<()> {
    let cmd: Command = Command::parse();
    cmd.run()
}
