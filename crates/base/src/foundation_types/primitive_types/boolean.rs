use std::ops::Not;
use crate::foundation_types::Any;


pub struct Boolean {
    pub value: bool,
}

impl Boolean {

    pub fn new(value: bool) -> Boolean {
        Boolean { value }
    }

}

impl Any for Boolean {

    fn is_equal(&self, other: &Boolean) -> bool {
        self.value == other.value
    }

    fn instance_of(&self, class_name: &str) -> bool {
        class_name == "Boolean"
    }

    fn type_of(&self) -> String {
        "Boolean".to_string()
    }

}

impl PartialEq<bool> for Boolean {

    fn eq(&self, other: &bool) -> bool {
        self.value == *other
    }

}

impl PartialEq<Boolean> for Boolean {

    fn eq(&self, other: &Boolean) -> bool {
        self.value == other.value
    }
}

impl Eq for Boolean {}

impl Not for &Boolean {
    type Output = Boolean;

    fn not(self) -> Boolean {
        Boolean::new(!self.value)
    }

}