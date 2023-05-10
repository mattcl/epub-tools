use std::{collections::HashMap, path::PathBuf};

use anyhow::{bail, Result};
use clap::Args;
use epub::doc::EpubDoc;
use walkdir::WalkDir;

use crate::{attention, failure, highlight, success};

/// Renames one or more epub files using info from the metadata.
///
/// You can specify multiple files and directories. Files that are explicitly
/// specified are permitted to not end with the epub extension, but when
/// recursing through directories only .epub files will be processed.
///
/// This will not actually make changes unless --execute is specified.
#[derive(Debug, Args)]
pub struct Rename {
    /// The paths to epub files or directories of epub files to rename.
    paths: Vec<PathBuf>,

    /// Actually perform the renaming operations.
    #[clap(long)]
    execute: bool,
}

impl Rename {
    pub fn run(&self) -> Result<()> {
        let mut files: HashMap<PathBuf, PathBuf> = HashMap::default();

        for path in self.paths.iter() {
            if path.is_file() {
                self.process_file_path(path, &mut files)?;
            } else if path.is_dir() {
                for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
                    let path = entry.into_path();
                    if path.is_file()
                        && path
                            .extension()
                            .map(|ext| ext == "epub")
                            .unwrap_or_default()
                    {
                        self.process_file_path(&path, &mut files)?;
                    }
                }
            } else {
                bail!(
                    "{} is neither a file nor a directory",
                    path.to_string_lossy()
                );
            }
        }

        if files.is_empty() {
            println!("\n  {}", highlight!("No eligible files found to rename."));
            return Ok(());
        }

        if self.execute {
            for (renamed, original) in files.iter() {
                println!(
                    "  Renaming {} to {}",
                    highlight!(original.to_string_lossy()),
                    highlight!(renamed.to_string_lossy()),
                );

                std::fs::rename(original, renamed)?;
            }

            println!("\n  {}", success!("Done"));
        } else {
            for (renamed, original) in files.iter() {
                println!(
                    "  Would rename {} to {}",
                    highlight!(original.to_string_lossy()),
                    highlight!(renamed.to_string_lossy()),
                );
            }

            println!(
                "\n  {}",
                attention!("Rerun with --execute to actually make changes.")
            );
        }

        Ok(())
    }

    fn process_file_path(
        &self,
        path: &PathBuf,
        files: &mut HashMap<PathBuf, PathBuf>,
    ) -> Result<()> {
        if !path.exists() {
            println!("  {}: {}", failure!("INVALID PATH"), path.to_string_lossy());
        }

        let renamed = {
            let doc = EpubDoc::new(path.to_string_lossy().to_string());
            match doc {
                Ok(doc) => doc.mdata("title").map(|v| {
                    let mut new_path = path.clone();
                    new_path.set_file_name(format!("{}.epub", v.trim()));
                    new_path
                }),
                Err(_) => None,
            }
        };

        match renamed {
            Some(new_name) => {
                if &new_name == path {
                    println!(
                        "  Skipping {} as its name already matches the title",
                        highlight!(path.to_string_lossy())
                    );
                    return Ok(());
                }

                // we have a file that would be renamed to match a file that
                // currently exists
                if new_name.exists() {
                    bail!(
                        "{} would be overwritten by {}",
                        new_name.to_string_lossy(),
                        path.to_string_lossy()
                    );
                }

                // At least of our files being renamed would collide.
                if files.contains_key(&new_name) {
                    // we know this exists
                    let collision = files.get(&new_name).unwrap().to_string_lossy();
                    bail!(
                        "Collision detected: {} and {} resolve to {}",
                        collision,
                        path.to_string_lossy(),
                        new_name.to_string_lossy()
                    );
                }

                files.insert(new_name, path.clone());
            }
            None => {
                println!(
                    "{}",
                    highlight!(format!(
                        "  Cannot rename: {}. The file is invalid or does not have title metadata",
                        path.to_string_lossy()
                    ))
                );
            }
        }

        Ok(())
    }
}
