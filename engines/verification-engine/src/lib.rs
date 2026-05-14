#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertySpec {
    pub id: String,
    pub statement: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationPlan {
    pub target: String,
    pub properties: Vec<PropertySpec>,
}

impl VerificationPlan {
    pub fn property_count(&self) -> usize {
        self.properties.len()
    }
}

pub fn capability() -> &'static str {
    "invariant-oriented formal verification"
}

pub fn starter_plan(target: &str) -> VerificationPlan {
    VerificationPlan {
        target: target.to_string(),
        properties: vec![
            PropertySpec {
                id: "authorized-admin-rotation".to_string(),
                statement: "Only the current admin can rotate privileged ownership.".to_string(),
            },
            PropertySpec {
                id: "bounded-distribution".to_string(),
                statement: "Distribution loops remain bounded by an explicit limit.".to_string(),
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::{capability, starter_plan};

    #[test]
    fn describes_the_engine_role() {
        assert!(capability().contains("verification"));
    }

    #[test]
    fn builds_a_verification_plan() {
        let plan = starter_plan("examples/secure-contracts");

        assert_eq!(plan.property_count(), 2);
        assert!(plan.properties[0].statement.contains("admin"));
    }
}
