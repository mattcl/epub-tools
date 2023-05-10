use anyhow::Result;
use clap::{Parser, Subcommand};

mod info;
mod rename;

/// A collection of tools for manipulating epub files.
#[derive(Debug, Parser)]
#[command(name = "epub-tools", version, about)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn run() -> Result<()> {
        let command = Self::parse().command;
        command.run()
    }
}

#[derive(Debug, Subcommand)]
enum Commands {
    Info(info::Info),
    Rename(rename::Rename),
}

impl Commands {
    pub fn run(&self) -> Result<()> {
        match self {
            Self::Info(cmd) => cmd.run(),
            Self::Rename(cmd) => cmd.run(),
        }
    }
}

#[macro_export]
macro_rules! attention {
    ($msg:expr) => {
        console::Style::new().magenta().apply_to($msg)
    };
}

#[macro_export]
macro_rules! highlight {
    ($msg:expr) => {
        console::Style::new().yellow().apply_to($msg)
    };
}

#[macro_export]
macro_rules! success {
    ($msg:expr) => {
        console::Style::new().green().apply_to($msg)
    };
}

#[macro_export]
macro_rules! failure {
    ($msg:expr) => {
        console::Style::new().red().apply_to($msg)
    };
}
