#[cfg(test)]
mod octet_tests {
    use openehr::base::foundation_types::primitive_types::octet::Octet;
    use openehr::base::foundation_types::primitive_types::any::Any;

    #[test]
    fn equality_with_same_value_returns_true() {
        let octet_a = Octet::new(255);
        let octet_b = Octet::new(255);
        assert_eq!(octet_a, octet_b);
    }

    #[test]
    fn equality_with_different_values_returns_false() {
        let octet_a = Octet::new(100);
        let octet_b = Octet::new(101);
        assert_ne!(octet_a, octet_b);
    }

    #[test]
    fn is_equal_with_same_value_returns_true() {
        let octet_a = Octet::new(50);
        let octet_b = Octet::new(50);
        assert!(octet_a.is_equal(&octet_b as &dyn std::any::Any));
    }

    #[test]
    fn is_equal_with_different_values_returns_false() {
        let octet_a = Octet::new(200);
        let octet_b = Octet::new(201);
        assert!(!octet_a.is_equal(&octet_b as &dyn std::any::Any));
    }

    #[test]
    fn is_equal_with_different_type_returns_false() {
        let octet = Octet::new(255);
        let not_an_octet = "not an octet";
        assert!(!octet.is_equal(&not_an_octet as &dyn std::any::Any));
    }

    #[test]
    fn debug_format_matches_expected_output() {
        let octet = Octet::new(123);
        assert_eq!(format!("{:?}", octet), "Octet(123)");
    }
}