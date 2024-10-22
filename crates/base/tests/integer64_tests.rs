#[cfg(test)]
mod tests {
    use base::foundation_types::{Any, Double, Integer64, Numeric};

    #[test]
    fn add_two_positive_integers() {
        let a = Integer64::new(5);
        let b = Integer64::new(3);
        let result = a + b;
        assert_eq!(result.value, 8);
    }

    #[test]
    fn add_positive_and_negative_integer() {
        let a = Integer64::new(5);
        let b = Integer64::new(-3);
        let result = a + b;
        assert_eq!(result.value, 2);
    }

    #[test]
    fn add_two_negative_integers() {
        let a = Integer64::new(-5);
        let b = Integer64::new(-3);
        let result = a + b;
        assert_eq!(result.value, -8);
    }

    #[test]
    fn subtract_two_positive_integers() {
        let a = Integer64::new(5);
        let b = Integer64::new(3);
        let result = a - b;
        assert_eq!(result.value, 2);
    }

    #[test]
    fn subtract_positive_and_negative_integer() {
        let a = Integer64::new(5);
        let b = Integer64::new(-3);
        let result = a - b;
        assert_eq!(result.value, 8);
    }

    #[test]
    fn multiply_two_positive_integers() {
        let a = Integer64::new(5);
        let b = Integer64::new(3);
        let result = a * b;
        assert_eq!(result.value, 15);
    }

    #[test]
    fn multiply_positive_and_negative_integer() {
        let a = Integer64::new(5);
        let b = Integer64::new(-3);
        let result = a * b;
        assert_eq!(result.value, -15);
    }

    #[test]
    fn divide_two_positive_integers() {
        let a = Integer64::new(6);
        let b = Integer64::new(3);
        let result = a / b;
        assert_eq!(result.value, 2.0);
    }

    #[test]
    fn divide_positive_and_negative_integer() {
        let a = Integer64::new(6);
        let b = Integer64::new(-3);
        let result = a / b;
        assert_eq!(result.value, -2.0);
    }

    #[test]
    fn modulo_two_positive_integers() {
        let a = Integer64::new(5);
        let b = Integer64::new(3);
        let result = a % b;
        assert_eq!(result.value, 2);
    }

    #[test]
    fn modulo_positive_and_negative_integer() {
        let a = Integer64::new(5);
        let b = Integer64::new(-3);
        let result = a % b;
        assert_eq!(result.value, 2);
    }

    #[test]
    fn negate_positive_integer() {
        let a = Integer64::new(5);
        let result = -a;
        assert_eq!(result.value, -5);
    }

    #[test]
    fn negate_negative_integer() {
        let a = Integer64::new(-5);
        let result = -a;
        assert_eq!(result.value, 5);
    }

    #[test]
    fn integer_is_equal() {
        let a = Integer64::new(5);
        let b = Integer64::new(5);
        assert!(a.is_equal(&b));
    }

    #[test]
    fn integer_is_not_equal() {
        let a = Integer64::new(5);
        let b = Integer64::new(3);
        assert!(!a.is_equal(&b));
    }

    #[test]
    fn integer_instance_of() {
        let a = Integer64::new(5);
        assert!(a.instance_of("Integer64"));
    }

    #[test]
    fn integer_type_of() {
        let a = Integer64::new(5);
        assert_eq!(a.type_of(), "Integer64");
    }

    #[test]
    fn equal_operator_with_equal_integers() {
        let a = Integer64::new(5);
        let b = Integer64::new(5);
        assert!(a == b);
    }

    #[test]
    fn equal_operator_with_unequal_integers() {
        let a = Integer64::new(5);
        let b = Integer64::new(3);
        assert!(a != b);
    }

    #[test]
    fn pow_positive_integer() {
        let a = Integer64::new(2);
        let result = a.pow(&Double::new(3.0));
        assert_eq!(result.value, 8.0);
    }

    #[test]
    fn pow_negative_integer() {
        let a = Integer64::new(-2);
        let result = a.pow(&Double::new(3.0));
        assert_eq!(result.value, -8.0);
    }

    #[test]
    fn pow_zero_integer() {
        let a = Integer64::new(0);
        let result = a.pow(&Double::new(3.0));
        assert_eq!(result.value, 0.0);
    }

    #[test]
    fn pow_integer_to_zero() {
        let a = Integer64::new(2);
        let result = a.pow(&Double::new(0.0));
        assert_eq!(result.value, 1.0);
    }

}