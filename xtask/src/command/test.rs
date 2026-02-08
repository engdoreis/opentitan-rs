// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

pub fn test() -> anyhow::Result<()> {
    println!("[Xtask] Running unittests...");
    std::process::Command::new("cargo")
        .args(["test"])
        .current_dir("lib/zermio/earlgrey")
        .status()?
        .exit_ok()?;

    println!("[Xtask] Running integration tests...");
    std::process::Command::new("cargo")
        .args(["test"])
        .current_dir("tests")
        .status()?
        .exit_ok()?;

    Ok(())
}
