use crate::detectors::{base_finding, operation_details};
use crate::ir::{IRContract, IROperationKind};
use crate::reporting::{Confidence, Finding, Severity};
use crate::rules::Detector;

pub struct MissingValidationDetector;

impl Detector for MissingValidationDetector {
    fn id(&self) -> &'static str {
        "SF-003"
    }

    fn name(&self) -> &'static str {
        "missing-validation"
    }

    fn description(&self) -> &'static str {
        "Detects public entrypoints that accept external inputs and perform risky operations without checks."
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
                    && function.metadata.has_non_env_params
                    && !function.metadata.has_validation
                    && (function.metadata.mutates_state
                        || function.metadata.raw_arithmetic_count > 0
                        || function.metadata.external_call_count > 0)
            })
            .map(|function| {
                let mut evidence = operation_details(function, IROperationKind::WriteStorage);
                evidence.extend(operation_details(function, IROperationKind::Arithmetic));
                evidence.push("no validation guard detected".to_string());

                base_finding(
                    contract,
                    function,
                    self.id(),
                    "Missing input validation",
                    self.severity(),
                    Confidence::Medium,
                    self.name(),
                    format!(
                        "Function `{}` accepts external parameters but performs risky work without a clear validation step.",
                        function.name
                    ),
                    "Validate externally supplied values before arithmetic, storage writes, or external calls."
                        .to_string(),
                    evidence,
                )
            })
            .collect()
    }
}
