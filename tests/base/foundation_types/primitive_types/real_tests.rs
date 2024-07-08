#[cfg(test)]
mod real_tests {
    use openehr::base::foundation_types::primitive_types::any::Any;
    use openehr::base::foundation_types::primitive_types::real::Real;


    #[test]
    fn equality_with_same_value_returns_true() {
        let real_a = Real::new(10.5);
        let real_b = Real::new(10.5);
        assert_eq!(real_a, real_b);
    }

    #[test]
    fn equality_with_different_values_returns_false() {
        let real_a = Real::new(10.5);
        let real_b = Real::new(10.6);
        assert_ne!(real_a, real_b);
    }

    #[test]
    fn is_equal_with_same_value_returns_true() {
        let real_a = Real::new(20.0);
        let real_b = Real::new(20.0);
        assert!(real_a.is_equal(&real_b as &dyn std::any::Any));
    }

    #[test]
    fn is_equal_with_different_values_returns_false() {
        let real_a = Real::new(30.1);
        let real_b = Real::new(30.2);
        assert!(!real_a.is_equal(&real_b as &dyn std::any::Any));
    }

    #[test]
    fn is_equal_with_different_type_returns_false() {
        let real = Real::new(40.0);
        let not_a_real = "not a real";
        assert!(!real.is_equal(&not_a_real as &dyn std::any::Any));
    }

    #[test]
    fn debug_format_matches_expected_output() {
        let real = Real::new(123.456);
        assert_eq!(format!("{:?}", real), "123.456");
    }
}
