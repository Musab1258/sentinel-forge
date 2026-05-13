use std::path::PathBuf;
use std::{fs, path::Path};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

use crate::analyze_path;
use crate::reporting::OutputFormat;

#[derive(Debug, Parser)]
#[command(name = "sentinel-forge", version, about = "Static security analysis for Soroban contracts")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Scan {
        path: PathBuf,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
        #[arg(long)]
        output: Option<PathBuf>,
    },
    Ci {
        path: PathBuf,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
        #[arg(long)]
        output: Option<PathBuf>,
    },
}

pub fn run() -> Result<i32> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan {
            path,
            format,
            output,
        } => {
            let report = analyze_path(path)?;
            emit_report(&report.render(format)?, output.as_deref())?;
            Ok(0)
        }
        Commands::Ci {
            path,
            format,
            output,
        } => {
            let report = analyze_path(path)?;
            emit_report(&report.render(format)?, output.as_deref())?;
            Ok(if report.findings.is_empty() { 0 } else { 1 })
        }
    }
}

fn emit_report(rendered: &str, output: Option<&Path>) -> Result<()> {
    if let Some(path) = output {
        fs::write(path, rendered)
            .with_context(|| format!("failed to write report to `{}`", path.display()))?;
        println!("report written to {}", path.display());
        return Ok(());
    }

    println!("{rendered}");
    Ok(())
}
