#[cfg(test)]
mod tests {
    use base::foundation_types::{Any, Boolean};

    #[test]
    fn boolean_new_creates_instance() {
        let b = Boolean::new(true);
        assert_eq!(b.value, true);
    }

    #[test]
    fn boolean_is_equal_with_boolean() {
        let b1 = Boolean::new(true);
        let b2 = Boolean::new(true);
        assert!(b1.is_equal(&b2));
    }

    #[test]
    fn boolean_is_equal_with_bool() {
        let b = Boolean::new(false);
        assert!(b == false);
    }

    #[test]
    fn boolean_not_operator() {
        let b = Boolean::new(true);
        let not_b = !&b;
        assert!(not_b == Boolean::new(false));
    }

    #[test]
    fn boolean_partial_eq_with_boolean() {
        let b1 = Boolean::new(true);
        let b2 = Boolean::new(true);
        assert!(b1 == b2);
    }

    #[test]
    fn boolean_partial_eq_with_bool() {
        let b = Boolean::new(true);
        assert!(b == true);
    }

    #[test]
    fn boolean_instance_of() {
        let b = Boolean::new(true);
        assert!(b.instance_of("Boolean"));
    }

    #[test]
    fn boolean_type_of() {
        let b = Boolean::new(true);
        assert_eq!(b.type_of(), "Boolean");
    }

}