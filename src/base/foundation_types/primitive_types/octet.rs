use std::fmt::Debug;

use crate::base::foundation_types::primitive_types::any::Any;

pub struct Octet {
    value: u8,
}

impl Octet {
    pub fn new(value: u8) -> Octet {
        Octet { value }
    }
}

impl Any for Octet {
    fn is_equal(&self, other: &dyn std::any::Any) -> bool {
        if let Some(that) = other.downcast_ref::<Octet>() {
            self.value == that.value
        } else {
            false
        }
    }
}

impl PartialEq for Octet {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Octet {}

impl Debug for Octet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}