#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use base::foundation_types::{Any, Character};

    #[test]
    fn character_new_creates_instance() {
        let c = Character::new('a');
        assert_eq!(c.value, 'a');
    }

    #[test]
    fn character_is_equal_with_character() {
        let c1 = Character::new('a');
        let c2 = Character::new('a');
        assert!(c1.is_equal(&c2));
    }

    #[test]
    fn character_is_equal_with_char() {
        let c = Character::new('a');
        assert!(c == 'a');
    }

    #[test]
    fn character_partial_eq_with_character() {
        let c1 = Character::new('a');
        let c2 = Character::new('a');
        assert!(c1 == c2);
    }

    #[test]
    fn character_instance_of() {
        let c = Character::new('a');
        assert!(c.instance_of("Character"));
    }

    #[test]
    fn character_type_of() {
        let c = Character::new('a');
        assert_eq!(c.type_of(), "Character");
    }

    #[test]
    fn character_ord_comparison_less() {
        let c1 = Character::new('a');
        let c2 = Character::new('b');
        assert!(c1 < c2);
    }

    #[test]
    fn character_ord_comparison_greater() {
        let c1 = Character::new('b');
        let c2 = Character::new('a');
        assert!(c1 > c2);
    }

    #[test]
    fn character_partial_ord_comparison() {
        let c1 = Character::new('a');
        let c2 = Character::new('b');
        assert_eq!(c1.partial_cmp(&c2).unwrap(), Ordering::Less);
    }
}