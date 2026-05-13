use std::path::PathBuf;

use static_analyzer::{analyze_path, analyze_source};

fn fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join(path)
}

#[test]
fn scans_single_contract_fixture() {
    let report = analyze_path(fixture("contracts/missing_authorization.rs"))
        .expect("fixture scan should succeed");

    assert!(report.findings.iter().any(|finding| finding.id == "SF-001"));
    assert!(report.findings.iter().any(|finding| finding.id == "SF-006"));
}

#[test]
fn scans_directory_of_contracts() {
    let report = analyze_path(fixture("contracts")).expect("directory scan should succeed");

    assert!(report.summary.files >= 4);
    assert!(report.findings.iter().any(|finding| finding.id == "SF-004"));
    assert!(report.findings.iter().any(|finding| finding.id == "SF-005"));
}

#[test]
fn guarded_contract_stays_clean() {
    let source = std::fs::read_to_string(fixture("regressions/guarded_admin.rs"))
        .expect("guarded fixture should exist");
    let report = analyze_source("guarded_admin.rs", &source).expect("analysis should succeed");

    assert!(report.findings.is_empty());
}
