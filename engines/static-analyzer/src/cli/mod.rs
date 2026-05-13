use std::path::PathBuf;

use anyhow::Result;
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
    },
    Ci {
        path: PathBuf,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
}

pub fn run() -> Result<i32> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path, format } => {
            let report = analyze_path(path)?;
            println!("{}", report.render(format)?);
            Ok(0)
        }
        Commands::Ci { path, format } => {
            let report = analyze_path(path)?;
            println!("{}", report.render(format)?);
            Ok(if report.findings.is_empty() { 0 } else { 1 })
        }
    }
}
