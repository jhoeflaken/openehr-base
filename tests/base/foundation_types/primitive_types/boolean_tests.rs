#[cfg(test)]
mod boolean_tests {
    use openehr::base::foundation_types::primitive_types::boolean::Boolean;
    use openehr::base::foundation_types::primitive_types::any::Any;

    #[test]
    fn boolean_equality_with_same_value_returns_true() {
        let a = Boolean::new(true);
        let b = Boolean::new(true);
        assert!(a.is_equal(&b));
    }

    #[test]
    fn boolean_equality_with_different_values_returns_false() {
        let a = Boolean::new(true);
        let b = Boolean::new(false);
        assert!(!a.is_equal(&b));
    }

    #[test]
    fn boolean_equality_with_different_types_returns_false() {
        let a = Boolean::new(true);
        let b = "true";
        assert!(!a.is_equal(&b));
    }

    #[test]
    fn partial_eq_with_same_value_returns_true() {
        let a = Boolean::new(true);
        let b = Boolean::new(true);
        assert_eq!(a, b);
    }

    #[test]
    fn partial_eq_with_different_values_returns_false() {
        let a = Boolean::new(true);
        let b = Boolean::new(false);
        assert_ne!(a, b);
    }

    #[test]
    fn debug_format_matches_expected_output() {
        let boolean = Boolean::new(true);
        assert_eq!(format!("{:?}", boolean), "Boolean(true)");
    }
}

