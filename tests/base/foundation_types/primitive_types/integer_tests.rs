#[cfg(test)]
mod integer_tests {
    use openehr::base::foundation_types::primitive_types::any::Any;
    use openehr::base::foundation_types::primitive_types::integer::Integer;


    #[test]
    fn equality_with_same_value_yields_true() {
        let int_a = Integer::new(10);
        let int_b = Integer::new(10);
        assert_eq!(int_a, int_b);
    }

    #[test]
    fn equality_with_different_values_yields_false() {
        let int_a = Integer::new(10);
        let int_b = Integer::new(20);
        assert_ne!(int_a, int_b);
    }

    #[test]
    fn is_equal_with_same_type_and_value_yields_true() {
        let int_a = Integer::new(30);
        let int_b = Integer::new(30);
        assert!(int_a.is_equal(&int_b as &dyn std::any::Any));
    }

    #[test]
    fn is_equal_with_same_type_and_different_value_yields_false() {
        let int_a = Integer::new(40);
        let int_b = Integer::new(50);
        assert!(!int_a.is_equal(&int_b as &dyn std::any::Any));
    }

    #[test]
    fn is_equal_with_different_type_yields_false() {
        let int = Integer::new(60);
        let not_an_int = "not an integer";
        assert!(!int.is_equal(&not_an_int as &dyn std::any::Any));
    }

    #[test]
    fn addition_of_two_integers() {
        let integer_a = Integer::new(34567890);
        let integer_b = Integer::new(10);
        let result = integer_a + integer_b;
        assert_eq!(result.value, 34567900);
    }

    #[test]
    fn subtraction_of_two_integers() {
        let integer_a = Integer::new(34567890);
        let integer_b = Integer::new(10);
        let result = integer_a - integer_b;
        assert_eq!(result.value, 34567880);
    }

    #[test]
    fn multiplication_of_two_integers() {
        let integer_a = Integer::new(34567890);
        let integer_b = Integer::new(10);
        let result = integer_a * integer_b;
        assert_eq!(result.value, 345678900);
    }

    #[test]
    fn division_of_two_integers() {
        let integer_a = Integer::new(34567890);
        let integer_b = Integer::new(10);
        let result = integer_a / integer_b;
        assert_eq!(result.value, 3456789);
    }

    #[test]
    fn negation_of_a_integer() {
        let integer = Integer::new(34567890);
        let result = -integer;
        assert_eq!(result.value, -34567890);
    }

    #[test]
    fn partial_ordering_less_than() {
        let integer_a = Integer::new(10);
        let integer_b = Integer::new(20);
        assert!(integer_a < integer_b);
    }

    #[test]
    fn partial_ordering_greater_than() {
        let integer_a = Integer::new(30);
        let integer_b = Integer::new(20);
        assert!(integer_a > integer_b);
    }

    #[test]
    fn partial_ordering_less_than_or_equal_with_equal_values() {
        let integer_a = Integer::new(20);
        let integer_b = Integer::new(20);
        assert!(integer_a <= integer_b);
    }

    #[test]
    fn partial_ordering_less_than_or_equal_with_less_value() {
        let integer_a = Integer::new(11);
        let integer_b = Integer::new(22);
        assert!(integer_a <= integer_b);
    }

    #[test]
    fn partial_ordering_greater_than_or_equal_with_equal_values() {
        let integer_a = Integer::new(33);
        let integer_b = Integer::new(33);
        assert!(integer_a >= integer_b);
    }

    #[test]
    fn debug_format_matches_expected_output() {
        let int = Integer::new(-70);
        assert_eq!(format!("{:?}", int), "-70");
    }
}