use crate::detectors::{base_finding, operation_details};
use crate::ir::{IRContract, IROperationKind};
use crate::reporting::{Confidence, Finding, Severity};
use crate::rules::Detector;

pub struct UnsafeStorageAccessDetector;

impl Detector for UnsafeStorageAccessDetector {
    fn id(&self) -> &'static str {
        "SF-002"
    }

    fn name(&self) -> &'static str {
        "unsafe-storage-access"
    }

    fn description(&self) -> &'static str {
        "Detects storage writes that appear to bypass validation or invariant checks."
    }

    fn severity(&self) -> Severity {
        Severity::High
    }

    fn analyze(&self, contract: &IRContract) -> Vec<Finding> {
        contract
            .functions
            .iter()
            .filter(|function| {
                function.is_public
                    && function.metadata.mutates_state
                    && !function.metadata.has_validation
                    && function.metadata.requires_auth
            })
            .map(|function| {
                let evidence = operation_details(function, IROperationKind::WriteStorage);
                base_finding(
                    contract,
                    function,
                    self.id(),
                    "Unsafe storage access pattern",
                    self.severity(),
                    Confidence::Medium,
                    self.name(),
                    format!(
                        "Function `{}` writes contract state without an observable validation guard.",
                        function.name
                    ),
                    "Add explicit ownership, bounds, or invariant checks before updating storage."
                        .to_string(),
                    evidence,
                )
            })
            .collect()
    }
}
