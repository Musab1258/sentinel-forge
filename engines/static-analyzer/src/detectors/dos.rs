use crate::detectors::{base_finding, operation_details};
use crate::ir::{IRContract, IROperationKind};
use crate::reporting::{Confidence, Finding, Severity};
use crate::rules::Detector;

pub struct DenialOfServiceDetector;

impl Detector for DenialOfServiceDetector {
    fn id(&self) -> &'static str {
        "SF-005"
    }

    fn name(&self) -> &'static str {
        "denial-of-service-pattern"
    }

    fn description(&self) -> &'static str {
        "Detects unbounded loops or iteration in public functions that may amplify execution cost."
    }

    fn severity(&self) -> Severity {
        Severity::Medium
    }

    fn analyze(&self, contract: &IRContract) -> Vec<Finding> {
        contract
            .functions
            .iter()
            .filter(|function| {
                function.is_public
                    && function.metadata.loop_count > 0
                    && (function.metadata.mutates_state
                        || function.metadata.external_call_count > 0
                        || function.metadata.has_non_env_params)
            })
            .map(|function| {
                let mut evidence = operation_details(function, IROperationKind::Loop);
                evidence.extend(operation_details(function, IROperationKind::WriteStorage));
                evidence.extend(operation_details(function, IROperationKind::ExternalCall));

                base_finding(
                    contract,
                    function,
                    self.id(),
                    "Potential denial-of-service pattern",
                    self.severity(),
                    Confidence::Medium,
                    self.name(),
                    format!(
                        "Function `{}` contains looping behavior that may scale with user-controlled input or expensive state work.",
                        function.name
                    ),
                    "Bound iteration, chunk large workloads, or move repeated work behind safer batching controls."
                        .to_string(),
                    evidence,
                )
            })
            .collect()
    }
}
