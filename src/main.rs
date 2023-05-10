use anyhow::Result;
use cli::Cli;

mod cli;

fn main() -> Result<()> {
    Cli::run()
}
