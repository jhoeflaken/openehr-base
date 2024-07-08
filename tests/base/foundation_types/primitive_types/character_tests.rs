#[cfg(test)]
mod character_tests {
    use openehr::base::foundation_types::primitive_types::any::Any;
    use openehr::base::foundation_types::primitive_types::character::Character;

    #[test]
    fn equality_with_same_character_returns_true() {
        let char_a = Character::new('a');
        let char_b = Character::new('a');
        assert_eq!(char_a, char_b);
    }

    #[test]
    fn equality_with_different_characters_returns_false() {
        let char_a = Character::new('a');
        let char_b = Character::new('b');
        assert_ne!(char_a, char_b);
    }

    #[test]
    fn is_equal_with_same_character_returns_true() {
        let char_a = Character::new('x');
        let char_b = Character::new('x');
        assert!(char_a.is_equal(&char_b as &dyn std::any::Any));
    }

    #[test]
    fn is_equal_with_different_characters_returns_false() {
        let char_a = Character::new('y');
        let char_b = Character::new('z');
        assert!(!char_a.is_equal(&char_b as &dyn std::any::Any));
    }

    #[test]
    fn is_equal_with_different_type_returns_false() {
        let character = Character::new('!');
        let not_a_character = 42; // Not a character
        assert!(!character.is_equal(&not_a_character as &dyn std::any::Any));
    }

    #[test]
    fn character_partial_ordering_less_than() {
        let char_a = Character::new('a');
        let char_b = Character::new('b');
        assert!(char_a < char_b);
    }

    #[test]
    fn character_partial_ordering_less_than_or_equal_with_equal_values() {
        let char_a = Character::new('c');
        let char_b = Character::new('c');
        assert!(char_a <= char_b);
    }

    #[test]
    fn character_partial_ordering_less_than_or_equal_with_less_value() {
        let char_a = Character::new('d');
        let char_b = Character::new('e');
        assert!(char_a <= char_b);
    }

    #[test]
    fn character_partial_ordering_greater_than() {
        let char_a = Character::new('y');
        let char_b = Character::new('x');
        assert!(char_a > char_b);
    }

    #[test]
    fn character_partial_ordering_greater_than_or_equal_with_equal_values() {
        let char_a = Character::new('z');
        let char_b = Character::new('z');
        assert!(char_a >= char_b);
    }

    #[test]
    fn character_partial_ordering_greater_than_or_equal_with_greater_value() {
        let char_a = Character::new('r');
        let char_b = Character::new('q');
        assert!(char_a >= char_b);
    }

    #[test]
    fn debug_format_matches_expected_output() {
        let character = Character::new('c');
        assert_eq!(format!("{:?}", character), "c");
    }
}