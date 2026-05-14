#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvariantSpec {
    pub id: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutationStrategy {
    pub name: String,
    pub preserves_structure: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuzzPlan {
    pub target: String,
    pub invariants: Vec<InvariantSpec>,
    pub strategies: Vec<MutationStrategy>,
}

impl FuzzPlan {
    pub fn summary(&self) -> String {
        format!(
            "{} invariants, {} mutation strategies",
            self.invariants.len(),
            self.strategies.len()
        )
    }
}

pub fn capability() -> &'static str {
    "property-based and state-mutation fuzzing"
}

pub fn starter_plan(target: &str) -> FuzzPlan {
    FuzzPlan {
        target: target.to_string(),
        invariants: vec![
            InvariantSpec {
                id: "authorization-guard".to_string(),
                description: "Privileged actions should remain authorization-gated.".to_string(),
            },
            InvariantSpec {
                id: "state-consistency".to_string(),
                description: "State transitions should preserve documented invariants.".to_string(),
            },
        ],
        strategies: vec![
            MutationStrategy {
                name: "boundary-values".to_string(),
                preserves_structure: true,
            },
            MutationStrategy {
                name: "stateful-call-sequences".to_string(),
                preserves_structure: false,
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::{capability, starter_plan};

    #[test]
    fn describes_the_engine_role() {
        assert!(capability().contains("fuzzing"));
    }

    #[test]
    fn builds_a_starter_plan() {
        let plan = starter_plan("examples/vulnerable-contracts");

        assert_eq!(plan.target, "examples/vulnerable-contracts");
        assert_eq!(plan.invariants.len(), 2);
        assert_eq!(plan.strategies.len(), 2);
        assert!(plan.summary().contains("mutation strategies"));
    }
}
