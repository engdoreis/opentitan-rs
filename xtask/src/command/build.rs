// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

pub fn build() -> anyhow::Result<()> {
    println!("[Xtask] Building all...");
    std::process::Command::new("cargo")
        .args(["build", "--release"])
        .status()?;
    Ok(())
}
