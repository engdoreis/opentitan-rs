// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

pub fn generate_raw_bmp565(
    input: std::path::PathBuf,
    output: std::path::PathBuf,
    width: u32,
    height: u32,
) -> anyhow::Result<()> {
    let img = image::open(&input)?;
    let img = img
        .resize(width, height, image::imageops::FilterType::Lanczos3)
        .into_rgb8();

    let pixels: Vec<u8> = img
        .pixels()
        .flat_map(|p| {
            let r = (p[0] as u16) >> 3;
            let g = (p[1] as u16) >> 2;
            let b = (p[2] as u16) >> 3;
            let rgb565: u16 = (r << 11) | (g << 5) | b;
            rgb565.to_le_bytes()
        })
        .collect();

    std::fs::write(&output, &pixels)?;

    println!(
        "[Xtask] Converted {} -> {} ({}x{}, {} bytes)",
        input.display(),
        output.display(),
        img.width(),
        img.height(),
        pixels.len()
    );

    Ok(())
}

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
