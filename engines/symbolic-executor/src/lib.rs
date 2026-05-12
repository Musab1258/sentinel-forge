pub fn capability() -> &'static str {
    "path exploration and constraint-driven execution analysis"
}

#[cfg(test)]
mod tests {
    use super::capability;

    #[test]
    fn describes_the_engine_role() {
        assert!(capability().contains("constraint"));
    }
}

