pub mod build;
pub mod test;

#[derive(clap::Parser, Debug)]
pub enum Command {
    Build,
    Test,
}

impl Command {
    pub fn run(self) -> anyhow::Result<()> {
        match self {
            Command::Build => build::build(),
            Command::Test => test::test(),
        }
    }
}
