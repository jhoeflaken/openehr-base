use std::fmt::Debug;
use std::ops::Add;
use crate::base::foundation_types::primitive_types::any::Any;
use crate::base::foundation_types::primitive_types::ordered::Ordered;

pub struct String {
    pub value: std::string::String
}

impl String {
    pub fn new(value: std::string::String) -> Self {
        Self { value }
    }
    pub fn is_integer(&self) -> bool {
        self.value.parse::<i32>().is_ok()
    }
    pub fn as_integer(&self) -> Result<i32, std::num::ParseIntError> {
        self.value.parse::<i32>()
    }

    // TODO Expose methods from std::string::String
}

impl Add<&String> for String {
    type Output = Self;

    fn add(self, other: &Self) -> Self::Output {
        String::new(format!("{}{}", self.value, other.value))
    }
}

impl Any for String {
    fn is_equal(&self, other: &dyn std::any::Any) -> bool {
        if let Some(other) = other.downcast_ref::<String>() {
            self == other
        } else {
            false
        }
    }
}

impl PartialEq for String {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for String {}

impl Ordered for String {}

impl PartialOrd for String {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Debug for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}



