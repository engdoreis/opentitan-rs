use clap::Parser;

pub mod command;
use crate::command::Command;

fn main() -> anyhow::Result<()> {
    let cmd: Command = Command::parse();
    cmd.run()
}
