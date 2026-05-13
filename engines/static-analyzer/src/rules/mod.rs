use crate::detectors::{
    built_in_detectors, DenialOfServiceDetector, IntegerOverflowRiskDetector,
    MissingAuthorizationDetector, MissingValidationDetector, PrivilegeEscalationDetector,
    UnsafeStorageAccessDetector,
};
use crate::ir::IRContract;
use crate::reporting::{Finding, Severity};

pub trait Detector: Send + Sync {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn severity(&self) -> Severity;
    fn analyze(&self, contract: &IRContract) -> Vec<Finding>;
}

pub struct DetectorRegistry {
    detectors: Vec<Box<dyn Detector>>,
}

impl DetectorRegistry {
    pub fn new() -> Self {
        Self {
            detectors: Vec::new(),
        }
    }

    pub fn with_builtins() -> Self {
        let mut registry = Self::new();
        for detector in built_in_detectors() {
            registry.register(detector);
        }
        registry
    }

    pub fn register(&mut self, detector: Box<dyn Detector>) {
        self.detectors.push(detector);
    }

    pub fn analyze(&self, contract: &IRContract) -> Vec<Finding> {
        let mut findings = Vec::new();
        for detector in &self.detectors {
            findings.extend(detector.analyze(contract));
        }
        findings
    }

    pub fn detector_count(&self) -> usize {
        self.detectors.len()
    }
}

impl Default for DetectorRegistry {
    fn default() -> Self {
        Self::with_builtins()
    }
}

#[allow(dead_code)]
fn _keep_detector_types_referenced() {
    let _ = (
        MissingAuthorizationDetector,
        UnsafeStorageAccessDetector,
        MissingValidationDetector,
        IntegerOverflowRiskDetector,
        DenialOfServiceDetector,
        PrivilegeEscalationDetector,
    );
}
