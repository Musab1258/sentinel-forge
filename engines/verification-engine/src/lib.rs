pub fn capability() -> &'static str {
    "invariant-oriented formal verification"
}

#[cfg(test)]
mod tests {
    use super::capability;

    #[test]
    fn describes_the_engine_role() {
        assert!(capability().contains("verification"));
    }
}
