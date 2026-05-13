use crate::detectors::{base_finding, operation_details};
use crate::ir::{IRContract, IROperationKind};
use crate::reporting::{Confidence, Finding, Severity};
use crate::rules::Detector;

pub struct MissingAuthorizationDetector;

impl Detector for MissingAuthorizationDetector {
    fn id(&self) -> &'static str {
        "SF-001"
    }

    fn name(&self) -> &'static str {
        "missing-authorization"
    }

    fn description(&self) -> &'static str {
        "Detects public state-changing functions that do not call require_auth()."
    }

    fn severity(&self) -> Severity {
        Severity::Critical
    }

    fn analyze(&self, contract: &IRContract) -> Vec<Finding> {
        contract
            .functions
            .iter()
            .filter(|function| {
                function.is_public
                    && function.metadata.mutates_state
                    && !function.metadata.requires_auth
            })
            .map(|function| {
                let mut evidence = operation_details(function, IROperationKind::WriteStorage);
                evidence.push("missing require_auth() call".to_string());

                base_finding(
                    contract,
                    function,
                    self.id(),
                    "Missing authorization check",
                    self.severity(),
                    Confidence::High,
                    self.name(),
                    format!(
                        "Public function `{}` mutates storage without an authorization gate.",
                        function.name
                    ),
                    "Require authorization before any privileged or state-changing operation."
                        .to_string(),
                    evidence,
                )
            })
            .collect()
    }
}
