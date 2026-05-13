use crate::detectors::{base_finding, operation_details};
use crate::ir::{IRContract, IROperationKind};
use crate::reporting::{Confidence, Finding, Severity};
use crate::rules::Detector;

pub struct IntegerOverflowRiskDetector;

impl Detector for IntegerOverflowRiskDetector {
    fn id(&self) -> &'static str {
        "SF-004"
    }

    fn name(&self) -> &'static str {
        "integer-overflow-risk"
    }

    fn description(&self) -> &'static str {
        "Detects raw arithmetic in public functions when no checked arithmetic operation is visible."
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
                    && function.metadata.raw_arithmetic_count > 0
                    && function.metadata.checked_arithmetic_count == 0
            })
            .map(|function| {
                let evidence = operation_details(function, IROperationKind::Arithmetic);
                base_finding(
                    contract,
                    function,
                    self.id(),
                    "Unchecked arithmetic path",
                    self.severity(),
                    Confidence::Medium,
                    self.name(),
                    format!(
                        "Function `{}` uses raw arithmetic without an observed checked_* or saturating_* operation.",
                        function.name
                    ),
                    "Prefer checked arithmetic or explicit bounds checks around externally influenced values."
                        .to_string(),
                    evidence,
                )
            })
            .collect()
    }
}
