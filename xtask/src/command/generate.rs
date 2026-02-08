// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

pub fn generate_zermio() -> anyhow::Result<()> {
    println!("[Xtask] Processing RDL...");
    std::process::Command::new("rdl2ot")
        .args(["export-rtl", "--soc", "rdl/earlgrey.rdl", "build/"])
        .status()?;

    println!("[Xtask] Generating linker script ...");
    std::process::Command::new("python")
        .args([
            "tools/rdlgenerator.py",
            "gen-linker-script",
            "build/rdl.json",
            "lib/linker/memory.ld",
        ])
        .status()?;

    println!("[Xtask] Generating zermio ...");
    std::process::Command::new("zermio-cli")
        .args([
            "import-rdl-json",
            "--rdl",
            "build/rdl.json",
            "--header-file",
            "rdl/license_header.md",
            "export-rust",
            "--dir",
            "lib/zermio/earlgrey",
        ])
        .status()?;

    println!("[Xtask] Formating ...");
    std::process::Command::new("cargo").args(["fmt"]).status()?;

    Ok(())
}
