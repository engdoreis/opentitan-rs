// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

pub fn test() -> anyhow::Result<()> {
    println!("[Xtask] testing all...");
    std::process::Command::new("cargo")
        .args(["test"])
        .current_dir("tests")
        .status()?;
    Ok(())
}
