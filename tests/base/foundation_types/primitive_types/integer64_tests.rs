#[cfg(test)]
mod integer64_tests {
    use openehr::base::foundation_types::primitive_types::any::Any;
    use openehr::base::foundation_types::primitive_types::integer64::Integer64;

    #[test]
    fn equality_with_same_value_returns_true() {
        let int64_a = Integer64::new(100);
        let int64_b = Integer64::new(100);
        assert_eq!(int64_a, int64_b);
    }

    #[test]
    fn equality_with_different_values_returns_false() {
        let int64_a = Integer64::new(-100);
        let int64_b = Integer64::new(100);
        assert_ne!(int64_a, int64_b);
    }

    #[test]
    fn is_equal_with_same_value_returns_true() {
        let int64_a = Integer64::new(1234567890);
        let int64_b = Integer64::new(1234567890);
        assert!(int64_a.is_equal(&int64_b as &dyn std::any::Any));
    }

    #[test]
    fn is_equal_with_different_values_returns_false() {
        let int64_a = Integer64::new(1234567890);
        let int64_b = Integer64::new(987654321);
        assert!(!int64_a.is_equal(&int64_b as &dyn std::any::Any));
    }

    #[test]
    fn is_equal_with_different_type_returns_false() {
        let int64 = Integer64::new(0);
        let not_an_integer64 = "not an integer64";
        assert!(!int64.is_equal(&not_an_integer64 as &dyn std::any::Any));
    }

    #[test]
    fn debug_format_matches_expected_output() {
        let int64 = Integer64::new(-1234567890);
        assert_eq!(format!("{:?}", int64), "-1234567890");
    }
}