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
}

impl Command {
    pub fn run(self) -> anyhow::Result<()> {
        match self {
            Command::Build => build::build(),
            Command::Test => test::test(),
            Command::Lint => lint::lint(),
            Command::GenerateZermio => generate::generate_zermio(),
        }
    }
}
