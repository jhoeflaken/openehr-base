use crate::foundation_types::{Any, Ordered};

pub type String = std::string::String;

pub trait StringExtension {
    fn is_integer(&self) -> bool;
    fn as_integer(&self) -> Option<i64>;
}

impl StringExtension for String {

    fn is_integer(&self) -> bool {
        self.parse::<i64>().is_ok()
    }

    fn as_integer(&self) -> Option<i64> {
        self.parse::<i64>().ok()
    }

}

impl Any for String {
    fn is_equal(&self, other: &Self) -> bool {
        self == other
    }

    fn instance_of(&self, type_name: &str) -> bool {
        "String" == type_name
    }

    fn type_of(&self) -> std::string::String {
        "String".to_string()
    }

}

impl Ordered for String {}