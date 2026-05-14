#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathConstraint {
    pub expression: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolicPath {
    pub id: String,
    pub constraints: Vec<PathConstraint>,
    pub goal: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolicPlan {
    pub target: String,
    pub paths: Vec<SymbolicPath>,
}

impl SymbolicPlan {
    pub fn path_count(&self) -> usize {
        self.paths.len()
    }
}

pub fn capability() -> &'static str {
    "path exploration and constraint-driven execution analysis"
}

pub fn starter_plan(target: &str) -> SymbolicPlan {
    SymbolicPlan {
        target: target.to_string(),
        paths: vec![
            SymbolicPath {
                id: "auth-bypass".to_string(),
                constraints: vec![PathConstraint {
                    expression: "caller_is_not_admin && privileged_write_occurs".to_string(),
                }],
                goal: "Reach privileged state mutation without authorization.".to_string(),
            },
            SymbolicPath {
                id: "unchecked-arithmetic".to_string(),
                constraints: vec![PathConstraint {
                    expression: "amount + bonus exceeds expected bound".to_string(),
                }],
                goal: "Expose arithmetic paths that need checked math.".to_string(),
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::{capability, starter_plan};

    #[test]
    fn describes_the_engine_role() {
        assert!(capability().contains("constraint"));
    }

    #[test]
    fn builds_symbolic_paths() {
        let plan = starter_plan("examples/benchmark-contracts/nested_branching.rs");

        assert_eq!(plan.path_count(), 2);
        assert!(plan.paths[0].goal.contains("authorization"));
    }
}
