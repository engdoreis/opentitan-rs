// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

pub fn lint() -> anyhow::Result<()> {
    println!("[Xtask] Linting license headers");
    std::process::Command::new("reuse")
        .args(["lint"])
        .status()?;

    println!("[Xtask] Linting Rust format");
    std::process::Command::new("cargo")
        .args(["fmt", "--check"])
        .status()?;

    println!("[Xtask] Linting clippy");
    std::process::Command::new("cargo")
        .args(["clippy"])
        .status()?;
    Ok(())
}
