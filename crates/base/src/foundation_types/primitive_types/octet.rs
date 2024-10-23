use std::cmp::Ordering;
use crate::foundation_types::Any;

pub struct Octet {
    pub value: u8
}

impl Octet {

    pub fn new(value: u8) -> Octet {
        Octet { value }
    }

}

impl Any for Octet {

    fn is_equal(&self, other: &Octet) -> bool {
        self.value == other.value
    }

    fn instance_of(&self, class_name: &str) -> bool {
        class_name == "Octet"
    }

    fn type_of(&self) -> String {
        "Octet".to_string()
    }

}

impl PartialEq<u8> for Octet {

    fn eq(&self, other: &u8) -> bool {
        self.value == *other
    }

}

impl PartialEq<Octet> for Octet {

    fn eq(&self, other: &Octet) -> bool {
        self.value == other.value
    }
}

impl Eq for Octet {}

impl PartialOrd for Octet {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }

}

impl Ord for Octet {

    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }

}