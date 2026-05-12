pub fn capability() -> &'static str {
    "property-based and state-mutation fuzzing"
}

#[cfg(test)]
mod tests {
    use super::capability;

    #[test]
    fn describes_the_engine_role() {
        assert!(capability().contains("fuzzing"));
    }
}

