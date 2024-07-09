#[cfg(test)]
mod tests {
    use openehr::base::foundation_types::primitive_types::string::String;

    #[test]
    fn string_new_creates_string_with_given_value() {
        let value = "Hello".to_string();
        let custom_string = String::new(value.clone());
        assert_eq!(custom_string.value, value);
    }

    #[test]
    fn add_combines_two_strings() {
        let string1 = String::new("Hello".to_string());
        let string2 = String::new(", world!".to_string());
        let result = string1 + &string2;
        assert_eq!(result.value, "Hello, world!");
    }

    #[test]
    fn is_integer_returns_true_for_integer_string() {
        let integer_string = String::new("123".to_string());
        assert!(integer_string.is_integer());
    }

    #[test]
    fn is_integer_returns_false_for_non_integer_string() {
        let non_integer_string = String::new("abc".to_string());
        assert!(!non_integer_string.is_integer());
    }

    #[test]
    fn as_integer_parses_integer_string_correctly() {
        let integer_string = String::new("123".to_string());
        assert_eq!(integer_string.as_integer(), Ok(123));
    }

    #[test]
    fn as_integer_panics_on_non_integer_string() {
        let non_integer_string = String::new("abc".to_string());
        assert!(non_integer_string.as_integer().is_err());
    }

    #[test]
    fn equal_strings_are_equal() {
        let string1 = String::new("test".to_string());
        let string2 = String::new("test".to_string());
        assert_eq!(string1, string2);
    }

    #[test]
    fn different_strings_are_not_equal() {
        let string1 = String::new("test1".to_string());
        let string2 = String::new("test2".to_string());
        assert_ne!(string1, string2);
    }

    #[test]
    fn strings_order_correctly_less_than() {
        let string1 = String::new("a".to_string());
        let string2 = String::new("b".to_string());
        assert!(string1 < string2);
    }

    #[test]
    fn strings_order_correctly_bigger_than() {
        let string1 = String::new("b".to_string());
        let string2 = String::new("a".to_string());
        assert!(string1 > string2);
    }


    #[test]
    fn debug_format_matches_expected_for_string() {
        let custom_string = String::new("Hello, world!".to_string());
        assert_eq!(format!("{:?}", custom_string), "Hello, world!");
    }
}