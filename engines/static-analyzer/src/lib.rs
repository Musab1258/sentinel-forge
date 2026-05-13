pub mod ast;
pub mod cli;
pub mod detectors;
pub mod ir;
pub mod reporting;
pub mod rules;
pub mod tracing;
pub mod utils;

use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use ast::parse_contract;
use ir::IRContract;
use reporting::{AnalysisReport, ScannedFile};
use rules::DetectorRegistry;
use utils::collect_rust_files;

pub const ANALYZER_NAME: &str = "Sentinel Forge static analyzer MVP";
pub const DETECTOR_FOCUS: [&str; 6] = [
    "authorization",
    "storage",
    "validation",
    "arithmetic",
    "denial-of-service",
    "privilege-escalation",
];

pub fn banner() -> &'static str {
    ANALYZER_NAME
}

pub fn analyze_source(name: &str, source: &str) -> Result<AnalysisReport> {
    let registry = DetectorRegistry::with_builtins();
    analyze_sources(name, vec![(name.to_string(), source.to_string())], &registry)
}

pub fn analyze_path(path: impl AsRef<Path>) -> Result<AnalysisReport> {
    let path = path.as_ref();
    let files = collect_rust_files(path)?;
    let registry = DetectorRegistry::with_builtins();
    let mut sources = Vec::with_capacity(files.len());

    for file in files {
        let display = file.display().to_string();
        let source = fs::read_to_string(&file)
            .with_context(|| format!("failed to read source file `{display}`"))?;
        sources.push((display, source));
    }

    analyze_sources(&path.display().to_string(), sources, &registry)
}

fn analyze_sources(
    target: &str,
    sources: Vec<(String, String)>,
    registry: &DetectorRegistry,
) -> Result<AnalysisReport> {
    let mut scanned_files = Vec::with_capacity(sources.len());
    let mut findings = Vec::new();
    let mut total_functions = 0usize;

    for (file, source) in sources {
        let contract = parse_contract(&file, &source)
            .with_context(|| format!("failed to parse `{file}` as Rust source"))?;
        total_functions += contract.functions.len();
        let ir_contract = IRContract::from_contract(contract);
        let contract_findings = registry.analyze(&ir_contract);

        scanned_files.push(ScannedFile {
            file: ir_contract.file.clone(),
            functions: ir_contract.functions.len(),
            findings: contract_findings.len(),
        });
        findings.extend(contract_findings);
    }

    let mut report = AnalysisReport::new(target.to_string(), scanned_files, findings, total_functions);
    report.sort_findings();
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::{analyze_source, banner, DETECTOR_FOCUS};

    #[test]
    fn exposes_phase_three_detector_focus() {
        assert_eq!(banner(), "Sentinel Forge static analyzer MVP");
        assert!(DETECTOR_FOCUS.contains(&"authorization"));
        assert!(DETECTOR_FOCUS.contains(&"privilege-escalation"));
    }

    #[test]
    fn finds_missing_authorization_in_inline_source() {
        let report = analyze_source(
            "inline.rs",
            r#"
            pub fn transfer_admin(e: Env, new_admin: Address) {
                storage::set_admin(&e, &new_admin);
            }
            "#,
        )
        .expect("inline analysis should succeed");

        assert!(report.findings.iter().any(|finding| finding.id == "SF-001"));
    }
}
