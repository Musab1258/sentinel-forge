mod arithmetic;
mod authorization;
mod dos;
mod privilege;
mod storage;
mod validation;

use crate::ir::{IRContract, IRFunction, IROperationKind};
use crate::reporting::{Confidence, Finding, Severity};
use crate::tracing::build_trace;

pub use arithmetic::IntegerOverflowRiskDetector;
pub use authorization::MissingAuthorizationDetector;
pub use dos::DenialOfServiceDetector;
pub use privilege::PrivilegeEscalationDetector;
pub use storage::UnsafeStorageAccessDetector;
pub use validation::MissingValidationDetector;

pub fn built_in_detectors() -> Vec<Box<dyn crate::rules::Detector>> {
    vec![
        Box::new(MissingAuthorizationDetector),
        Box::new(UnsafeStorageAccessDetector),
        Box::new(MissingValidationDetector),
        Box::new(IntegerOverflowRiskDetector),
        Box::new(DenialOfServiceDetector),
        Box::new(PrivilegeEscalationDetector),
    ]
}

fn base_finding(
    contract: &IRContract,
    function: &IRFunction,
    id: &str,
    title: &str,
    severity: Severity,
    confidence: Confidence,
    detector: &str,
    description: String,
    remediation: String,
    evidence: Vec<String>,
) -> Finding {
    let line = function
        .operations
        .first()
        .map(|operation| operation.line)
        .unwrap_or(function.line);

    Finding::new(
        id,
        title,
        severity,
        confidence,
        description,
        remediation,
        contract.file.clone(),
        function.name.clone(),
        line,
        detector,
        evidence,
        build_trace(function),
    )
}

fn operation_details(function: &IRFunction, kind: IROperationKind) -> Vec<String> {
    function
        .operations
        .iter()
        .filter(|operation| operation.kind == kind)
        .map(|operation| format!("line {}: {}", operation.line, operation.detail))
        .collect()
}
