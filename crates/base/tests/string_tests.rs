#[cfg(test)]
mod tests {
    use base::foundation_types::{Any, String, StringExtension};

    #[test]
    fn string_is_integer_with_valid_integer() {
        let s = String::from("123");
        assert!(s.is_integer());
    }

    #[test]
    fn string_is_integer_with_invalid_integer() {
        let s = String::from("abc");
        assert!(!s.is_integer());
    }

    #[test]
    fn string_as_integer_with_valid_integer() {
        let s = String::from("123");
        assert_eq!(s.as_integer(), Some(123));
    }

    #[test]
    fn string_as_integer_with_invalid_integer() {
        let s = String::from("abc");
        assert_eq!(s.as_integer(), None);
    }

    #[test]
    fn string_is_equal_with_same_string() {
        let s1 = String::from("hello");
        let s2 = String::from("hello");
        assert!(s1.is_equal(&s2));
    }

    #[test]
    fn string_is_equal_with_different_string() {
        let s1 = String::from("hello");
        let s2 = String::from("world");
        assert!(!s1.is_equal(&s2));
    }

    #[test]
    fn string_equal_with_same_string() {
        let s1 = String::from("hello");
        let s2 = String::from("hello");
        assert_eq!(s1, s2);
    }

    #[test]
    fn string_equal_with_different_string() {
        let s1 = String::from("hello");
        let s2 = String::from("world");
        assert_ne!(s1, s2);
    }


    #[test]
    fn string_instance_of_string() {
        let s = String::from("hello");
        assert!(s.instance_of("String"));
    }

    #[test]
    fn string_type_of_string() {
        let s = String::from("hello");
        assert_eq!(s.type_of(), "String");
    }


}