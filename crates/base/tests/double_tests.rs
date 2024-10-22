#[cfg(test)]
mod tests {
    use base::foundation_types::{Any, Double, Numeric};

    #[test]
    fn add_two_positive_doubles() {
        let a = Double::new(5.5);
        let b = Double::new(3.0);
        let result = a + b;
        assert_eq!(result.value, 8.5);
    }

    #[test]
    fn add_positive_and_negative_double() {
        let a = Double::new(5.5);
        let b = Double::new(-3.0);
        let result = a + b;
        assert_eq!(result.value, 2.5);
    }

    #[test]
    fn add_two_negative_doubles() {
        let a = Double::new(-5.5);
        let b = Double::new(-3.0);
        let result = a + b;
        assert_eq!(result.value, -8.5);
    }

    #[test]
    fn subtract_two_positive_doubles() {
        let a = Double::new(5.5);
        let b = Double::new(3.0);
        let result = a - b;
        assert_eq!(result.value, 2.5);
    }

    #[test]
    fn subtract_positive_and_negative_double() {
        let a = Double::new(5.5);
        let b = Double::new(-3.0);
        let result = a - b;
        assert_eq!(result.value, 8.5);
    }

    #[test]
    fn multiply_two_positive_doubles() {
        let a = Double::new(5.5);
        let b = Double::new(3.0);
        let result = a * b;
        assert_eq!(result.value, 16.5);
    }

    #[test]
    fn multiply_positive_and_negative_double() {
        let a = Double::new(5.5);
        let b = Double::new(-3.0);
        let result = a * b;
        assert_eq!(result.value, -16.5);
    }

    #[test]
    fn divide_two_positive_doubles() {
        let a = Double::new(5.0);
        let b = Double::new(2.0);
        let result = a / b;
        assert_eq!(result.value, 2.5);
    }

    #[test]
    fn divide_positive_and_negative_double() {
        let a = Double::new(5.0);
        let b = Double::new(-2.0);
        let result = a / b;
        assert_eq!(result.value, -2.5);
    }

    #[test]
    fn negate_positive_double() {
        let a = Double::new(5.5);
        let result = -a;
        assert_eq!(result.value, -5.5);
    }

    #[test]
    fn negate_negative_double() {
        let a = Double::new(-5.5);
        let result = -a;
        assert_eq!(result.value, 5.5);
    }

    #[test]
    fn double_is_equal() {
        let a = Double::new(5.5);
        let b = Double::new(5.50);
        assert!(a.is_equal(&b));
    }

    #[test]
    fn double_is_not_equal() {
        let a = Double::new(5.5);
        let b = Double::new(3.0);
        assert!(!a.is_equal(&b));
    }

    #[test]
    fn double_instance_of() {
        let a = Double::new(5.0);
        assert!(a.instance_of("Double"));
    }

    #[test]
    fn double_type_of() {
        let a = Double::new(5.0);
        assert_eq!(a.type_of(), "Double");
    }

    #[test]
    fn equal_operator_with_equal_doubles() {
        let a = Double::new(5.5);
        let b = Double::new(5.50);
        assert!(a == b);
    }

    #[test]
    fn equal_operator_with_unequal_doubles() {
        let a = Double::new(5.0);
        let b = Double::new(5.5);
        assert!(a != b);
    }

    #[test]
    fn floor_positive_double() {
        let a = Double::new(5.7);
        let result = a.floor();
        assert_eq!(result.value, 5);
    }

    #[test]
    fn floor_negative_double() {
        let a = Double::new(-5.7);
        let result = a.floor();
        assert_eq!(result.value, -6);
    }

    #[test]
    fn pow_positive_double() {
        let a = Double::new(2.0);
        let result = a.pow(&Double::new(3.0));
        assert_eq!(result.value, 8.0);
    }

    #[test]
    fn pow_negative_double() {
        let a = Double::new(-2.0);
        let result = a.pow(&Double::new(3.0));
        assert_eq!(result.value, -8.0);
    }

    #[test]
    fn pow_zero_double() {
        let a = Double::new(0.0);
        let result = a.pow(&Double::new(3.0));
        assert_eq!(result.value, 0.0);
    }

    #[test]
    fn pow_double_to_zero() {
        let a = Double::new(2.0);
        let result = a.pow(&Double::new(0.0));
        assert_eq!(result.value, 1.0);
    }

}