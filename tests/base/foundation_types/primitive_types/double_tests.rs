#[cfg(test)]
mod double_tests {
    use openehr::base::foundation_types::primitive_types::double::Double;
    use openehr::base::foundation_types::primitive_types::any::Any;

    #[test]
    fn equality_with_same_value_yields_true() {
        let double_a = Double::new(1.23);
        let double_b = Double::new(1.23);
        assert_eq!(double_a, double_b);
    }

    #[test]
    fn equality_with_different_values_yields_false() {
        let double_a = Double::new(1.23);
        let double_b = Double::new(4.56);
        assert_ne!(double_a, double_b);
    }

    #[test]
    fn is_equal_with_same_type_and_value_yields_true() {
        let double_a = Double::new(7.89);
        let double_b = Double::new(7.89);
        assert!(double_a.is_equal(&double_b as &dyn std::any::Any));
    }

    #[test]
    fn is_equal_with_same_type_and_different_value_yields_false() {
        let double_a = Double::new(10.11);
        let double_b = Double::new(12.13);
        assert!(!double_a.is_equal(&double_b as &dyn std::any::Any));
    }

    #[test]
    fn is_equal_with_different_type_yields_false() {
        let double = Double::new(14.15);
        let not_a_double = "not a double";
        assert!(!double.is_equal(&not_a_double as &dyn std::any::Any));
    }

    #[test]
    fn addition_of_two_doubles() {
        let double_a = Double::new(5.5);
        let double_b = Double::new(4.5);
        let result = double_a + double_b;
        assert_eq!(result.value, 10.0);
    }

    #[test]
    fn subtraction_of_two_doubles() {
        let double_a = Double::new(10.0);
        let double_b = Double::new(4.5);
        let result = double_a - double_b;
        assert_eq!(result.value, 5.5);
    }

    #[test]
    fn multiplication_of_two_doubles() {
        let double_a = Double::new(2.0);
        let double_b = Double::new(3.5);
        let result = double_a * double_b;
        assert_eq!(result.value, 7.0);
    }

    #[test]
    fn division_of_two_doubles() {
        let double_a = Double::new(10.0);
        let double_b = Double::new(2.0);
        let result = double_a / double_b;
        assert_eq!(result.value, 5.0);
    }

    #[test]
    fn negation_of_a_double() {
        let double = Double::new(5.0);
        let result = -double;
        assert_eq!(result.value, -5.0);
    }

    #[test]
    fn division_by_zero_yields_infinity() {
        let double_a = Double::new(1.0);
        let double_b = Double::new(0.0);
        let result = double_a / double_b;
        assert!(result.value.is_infinite());
    }

    #[test]
    fn partial_ordering_less_than() {
        let double_a = Double::new(1.1);
        let double_b = Double::new(2.2);
        assert!(double_a < double_b);
    }

    #[test]
    fn partial_ordering_greater_than() {
        let double_a = Double::new(3.3);
        let double_b = Double::new(2.2);
        assert!(double_a > double_b);
    }

    #[test]
    fn partial_ordering_less_than_or_equal_with_equal_values() {
        let double_a = Double::new(2.2);
        let double_b = Double::new(2.2);
        assert!(double_a <= double_b);
    }

    #[test]
    fn partial_ordering_less_than_or_equal_with_less_value() {
        let double_a = Double::new(1.1);
        let double_b = Double::new(2.2);
        assert!(double_a <= double_b);
    }

    #[test]
    fn partial_ordering_greater_than_or_equal_with_equal_values() {
        let double_a = Double::new(3.3);
        let double_b = Double::new(3.3);
        assert!(double_a >= double_b);
    }

    #[test]
    fn partial_ordering_greater_than_or_equal_with_greater_value() {
        let double_a = Double::new(4.4);
        let double_b = Double::new(3.3);
        assert!(double_a >= double_b);
    }

    #[test]
    fn debug_format_matches_expected_output() {
        let double = Double::new(-16.17);
        assert_eq!(format!("{:?}", double), "-16.17");
    }
}