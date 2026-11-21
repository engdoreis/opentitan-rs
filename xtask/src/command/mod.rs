// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

pub mod build;
pub mod generate;
pub mod lint;
pub mod test;

#[derive(clap::Parser, Debug)]
pub enum Command {
    Build,
    Test,
    Lint,
    GenerateZermio,
    GenerateRawBmp565 {
        input: std::path::PathBuf,
        output: std::path::PathBuf,
        /// Target width in pixels (preserves aspect ratio)
        #[arg(long, default_value_t = 160)]
        width: u32,
        /// Target height in pixels (preserves aspect ratio)
        #[arg(long, default_value_t = 128)]
        height: u32,
    },
}

impl Command {
    pub fn run(self) -> anyhow::Result<()> {
        match self {
            Command::Build => build::build(),
            Command::Test => test::test(),
            Command::Lint => lint::lint(),
            Command::GenerateZermio => generate::generate_zermio(),
            Command::GenerateRawBmp565 {
                input,
                output,
                width,
                height,
            } => generate::generate_raw_bmp565(input, output, width, height),
        }
    }
}
