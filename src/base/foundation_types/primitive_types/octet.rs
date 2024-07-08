use std::fmt::Debug;

use crate::base::foundation_types::primitive_types::any::Any;
use crate::base::foundation_types::primitive_types::ordered::Ordered;

pub struct Octet {
    value: u8,
}

impl Octet {
    pub fn new(value: u8) -> Octet {
        Octet { value }
    }
}

impl Ordered for Octet {}

impl PartialOrd for Octet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
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