use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Result};

pub fn collect_rust_files(path: &Path) -> Result<Vec<PathBuf>> {
    if path.is_file() {
        if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
            bail!("expected a Rust source file, got `{}`", path.display());
        }

        return Ok(vec![path.to_path_buf()]);
    }

    if !path.exists() {
        bail!("path `{}` does not exist", path.display());
    }

    let mut files = Vec::new();
    visit_directory(path, &mut files)?;
    files.sort();

    if files.is_empty() {
        bail!("no Rust source files found under `{}`", path.display());
    }

    Ok(files)
}

fn visit_directory(path: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            visit_directory(&entry_path, files)?;
            continue;
        }

        if entry_path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
            files.push(entry_path);
        }
    }

    Ok(())
}
