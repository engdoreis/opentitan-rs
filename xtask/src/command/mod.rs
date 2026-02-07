pub mod build;

#[derive(clap::Parser, Debug)]
pub enum Command {
    Build,
}

impl Command {
    pub fn run(self) -> anyhow::Result<()> {
        match self {
            Command::Build => build::build(),
        }
    }
}
