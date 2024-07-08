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
    fn addition_of_two_reals() {
        let real_a = Real::new(5.5);
        let real_b = Real::new(4.5);
        let result = real_a + real_b;
        assert_eq!(result.value, 10.0);
    }

    #[test]
    fn subtraction_of_two_reals() {
        let real_a = Real::new(10.0);
        let real_b = Real::new(4.5);
        let result = real_a - real_b;
        assert_eq!(result.value, 5.5);
    }

    #[test]
    fn multiplication_of_two_reals() {
        let real_a = Real::new(2.0);
        let real_b = Real::new(3.5);
        let result = real_a * real_b;
        assert_eq!(result.value, 7.0);
    }

    #[test]
    fn division_of_two_reals() {
        let real_a = Real::new(10.0);
        let real_b = Real::new(2.0);
        let result = real_a / real_b;
        assert_eq!(result.value, 5.0);
    }

    #[test]
    fn negation_of_a_real() {
        let real = Real::new(5.0);
        let result = -real;
        assert_eq!(result.value, -5.0);
    }

    #[test]
    fn division_by_zero_yields_infinity() {
        let real_a = Real::new(1.0);
        let real_b = Real::new(0.0);
        let result = real_a / real_b;
        assert!(result.value.is_infinite());
    }

    #[test]
    fn partial_ordering_less_than() {
        let real_a = Real::new(1.1);
        let real_b = Real::new(2.2);
        assert!(real_a < real_b);
    }

    #[test]
    fn partial_ordering_greater_than() {
        let real_a = Real::new(3.3);
        let real_b = Real::new(2.2);
        assert!(real_a > real_b);
    }

    #[test]
    fn partial_ordering_less_than_or_equal_with_equal_values() {
        let real_a = Real::new(2.2);
        let real_b = Real::new(2.2);
        assert!(real_a <= real_b);
    }

    #[test]
    fn partial_ordering_less_than_or_equal_with_less_value() {
        let real_a = Real::new(1.1);
        let real_b = Real::new(2.2);
        assert!(real_a <= real_b);
    }

    #[test]
    fn partial_ordering_greater_than_or_equal_with_equal_values() {
        let real_a = Real::new(3.3);
        let real_b = Real::new(3.3);
        assert!(real_a >= real_b);
    }

    #[test]
    fn partial_ordering_greater_than_or_equal_with_greater_value() {
        let real_a = Real::new(4.4);
        let real_b = Real::new(3.3);
        assert!(real_a >= real_b);
    }

    #[test]
    fn debug_format_matches_expected_output() {
        let real = Real::new(123.456);
        assert_eq!(format!("{:?}", real), "123.456");
    }
}
