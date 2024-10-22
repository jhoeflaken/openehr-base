#[cfg(test)]
mod tests {
    use base::foundation_types::{Any, Double, Numeric, Real};

    #[test]
    fn add_two_positive_reals() {
        let a = Real::new(5.5);
        let b = Real::new(3.0);
        let result = a + b;
        assert_eq!(result.value, 8.5);
    }

    #[test]
    fn add_positive_and_negative_reals() {
        let a = Real::new(5.5);
        let b = Real::new(-3.0);
        let result = a + b;
        assert_eq!(result.value, 2.5);
    }

    #[test]
    fn add_two_negative_reals() {
        let a = Real::new(-5.5);
        let b = Real::new(-3.0);
        let result = a + b;
        assert_eq!(result.value, -8.5);
    }

    #[test]
    fn subtract_two_positive_reals() {
        let a = Real::new(5.5);
        let b = Real::new(3.0);
        let result = a - b;
        assert_eq!(result.value, 2.5);
    }

    #[test]
    fn subtract_positive_and_negative_real() {
        let a = Real::new(5.5);
        let b = Real::new(-3.0);
        let result = a - b;
        assert_eq!(result.value, 8.5);
    }

    #[test]
    fn multiply_two_positive_reals() {
        let a = Real::new(5.5);
        let b = Real::new(3.0);
        let result = a * b;
        assert_eq!(result.value, 16.5);
    }

    #[test]
    fn multiply_positive_and_negative_real() {
        let a = Real::new(5.5);
        let b = Real::new(-3.0);
        let result = a * b;
        assert_eq!(result.value, -16.5);
    }

    #[test]
    fn divide_two_positive_reals() {
        let a = Real::new(5.0);
        let b = Real::new(2.0);
        let result = a / b;
        assert_eq!(result.value, 2.5);
    }

    #[test]
    fn divide_positive_and_negative_real() {
        let a = Real::new(5.0);
        let b = Real::new(-2.0);
        let result = a / b;
        assert_eq!(result.value, -2.5);
    }

    #[test]
    fn negate_positive_real() {
        let a = Real::new(5.5);
        let result = -a;
        assert_eq!(result.value, -5.5);
    }

    #[test]
    fn negate_negative_real() {
        let a = Real::new(-5.5);
        let result = -a;
        assert_eq!(result.value, 5.5);
    }

    #[test]
    fn real_is_equal() {
        let a = Real::new(5.5);
        let b = Real::new(5.50);
        assert!(a.is_equal(&b));
    }

    #[test]
    fn real_is_not_equal() {
        let a = Real::new(5.5);
        let b = Real::new(3.0);
        assert!(!a.is_equal(&b));
    }

    #[test]
    fn real_instance_of() {
        let a = Real::new(5.0);
        assert!(a.instance_of("Real"));
    }

    #[test]
    fn real_type_of() {
        let a = Real::new(5.0);
        assert_eq!(a.type_of(), "Real");
    }

    #[test]
    fn equal_operator_with_equal_reals() {
        let a = Real::new(5.5);
        let b = Real::new(5.50);
        assert!(a == b);
    }

    #[test]
    fn equal_operator_with_unequal_reals() {
        let a = Real::new(5.0);
        let b = Real::new(5.5);
        assert!(a != b);
    }

    #[test]
    fn floor_positive_real() {
        let a = Real::new(5.7);
        let result = a.floor();
        assert_eq!(result.value, 5);
    }

    #[test]
    fn floor_negative_real() {
        let a = Real::new(-5.7);
        let result = a.floor();
        assert_eq!(result.value, -6);
    }

    #[test]
    fn pow_positive_real() {
        let a = Real::new(2.0);
        let result = a.pow(&Double::new(3.0));
        assert_eq!(result.value, 8.0);
    }

    #[test]
    fn pow_negative_real() {
        let a = Real::new(-2.0);
        let result = a.pow(&Double::new(3.0));
        assert_eq!(result.value, -8.0);
    }

    #[test]
    fn pow_zero_real() {
        let a = Real::new(0.0);
        let result = a.pow(&Double::new(3.0));
        assert_eq!(result.value, 0.0);
    }

    #[test]
    fn pow_real_to_zero() {
        let a = Real::new(2.0);
        let result = a.pow(&Double::new(0.0));
        assert_eq!(result.value, 1.0);
    }

}