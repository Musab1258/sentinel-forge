pub const DETECTOR_FOCUS: [&str; 4] = [
    "authorization",
    "storage",
    "arithmetic",
    "denial-of-service",
];

pub fn banner() -> &'static str {
    "Sentinel Forge static analyzer bootstrap"
}

#[cfg(test)]
mod tests {
    use super::{banner, DETECTOR_FOCUS};

    #[test]
    fn exposes_phase_one_detector_focus() {
        assert_eq!(banner(), "Sentinel Forge static analyzer bootstrap");
        assert!(DETECTOR_FOCUS.contains(&"authorization"));
    }
}

