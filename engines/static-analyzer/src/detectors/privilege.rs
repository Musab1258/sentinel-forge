use crate::detectors::{base_finding, operation_details};
use crate::ir::{IRContract, IROperationKind};
use crate::reporting::{Confidence, Finding, Severity};
use crate::rules::Detector;

pub struct PrivilegeEscalationDetector;

impl Detector for PrivilegeEscalationDetector {
    fn id(&self) -> &'static str {
        "SF-006"
    }

    fn name(&self) -> &'static str {
        "privilege-escalation"
    }

    fn description(&self) -> &'static str {
        "Detects public privilege or role transitions that do not enforce authorization."
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
                    && function.metadata.possible_privileged
                    && (function.metadata.mutates_state
                        || function
                            .operations
                            .iter()
                            .any(|operation| operation.kind == IROperationKind::PrivilegedAction))
                    && !function.metadata.requires_auth
            })
            .map(|function| {
                let mut evidence = operation_details(function, IROperationKind::PrivilegedAction);
                evidence.extend(operation_details(function, IROperationKind::WriteStorage));
                evidence.push("privileged transition lacks authorization gate".to_string());

                base_finding(
                    contract,
                    function,
                    self.id(),
                    "Privilege escalation risk",
                    self.severity(),
                    Confidence::High,
                    self.name(),
                    format!(
                        "Function `{}` appears to manage privileged state without verifying the caller.",
                        function.name
                    ),
                    "Require the current admin, owner, or authorized role to approve privilege changes before state updates."
                        .to_string(),
                    evidence,
                )
            })
            .collect()
    }
}
