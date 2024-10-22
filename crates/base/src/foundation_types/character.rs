use std::cmp::Ordering;
use crate::foundation_types::{Any, Ordered};

pub struct Character {
    pub value: char,
}

impl Character {

    pub fn new(value: char) -> Character {
        Character { value }
    }

}

impl Any for Character {

    fn is_equal(&self, other: &Character) -> bool {
        self.value == other.value
    }

    fn instance_of(&self, class_name: &str) -> bool {
        class_name == "Character"
    }

    fn type_of(&self) -> String {
        "Character".to_string()
    }

}

impl PartialEq<Character> for Character {

    fn eq(&self, other: &Character) -> bool {
        self.value == other.value
    }

}

impl Eq for Character {}

impl PartialEq<char> for Character {

    fn eq(&self, other: &char) -> bool {
        self.value == *other
    }

}

impl Ordered for Character {}

impl Ord for Character {

    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }

}

impl PartialOrd for Character {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.value.cmp(&other.value))
    }

}

