use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use clap::Args;
use epub::doc::EpubDoc;

/// Get info about an epub file.
///
/// This just lists the values for the various metadata keys on the epub file.
#[derive(Debug, Args)]
pub(crate) struct Info {
    /// The file to inspect.
    file: PathBuf,
}

impl Info {
    pub fn run(&self) -> Result<()> {
        if !self.file.is_file() {
            bail!("File '{}' does not exist", self.file.to_string_lossy());
        }

        let doc = EpubDoc::new(self.file.to_string_lossy().to_string())
            .context("Failed to open as epub file")?;

        for (k, v) in doc.metadata.iter() {
            println!("{}", k);
            for entry in v.iter() {
                println!("  {}", entry);
            }
        }

        Ok(())
    }
}
