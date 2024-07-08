use std::fmt::Debug;
use crate::base::foundation_types::primitive_types::any::Any;

/// This struct represents the OpenEHR Boolean type as described in the specification.
/// It is a wrapper around a Rust boolean value.
/// See https://specifications.openehr.org/releases/BASE/latest/foundation_types.html#_boolean_class
/// for more information.
pub struct Boolean {
    value: bool,
}

/// Implementation of the OpenEHR Boolean type.
impl Boolean {
    /// Creates a new Boolean instance with the given value.
    pub fn new(value: bool) -> Boolean {
        Boolean { value }
    }
}

/// Implementation of the [Any] trait for the Boolean type.
impl Any for Boolean {
    /// Returns true if the given object is a Boolean instance and its value is equal to the
    /// value of this instance. Otherwise, it returns false.
    fn is_equal(&self, other: &dyn std::any::Any) -> bool {
        if let Some(that) = other.downcast_ref::<Boolean>() {
            self.value == that.value
        } else {
            false
        }
    }
}

/// Implementation of the [PartialEq] trait for the Boolean type to implement the "==" and " !="
/// operators for the Boolean type.
impl PartialEq for Boolean {
    /// Returns true if the value of this instance is equal to the value of the other instance.
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

/// Implementation of the [Eq] trait for the Boolean type to implement the "==" and " !="
impl Eq for Boolean {}

impl Debug for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}


