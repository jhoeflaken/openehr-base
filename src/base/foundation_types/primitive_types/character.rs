use crate::base::foundation_types::primitive_types::any::Any;
use crate::base::foundation_types::primitive_types::ordered::Ordered;

pub struct Character {
    pub value: char,
}

impl Character {
    pub fn new(value: char) -> Self {
        Self { value }
    }
}

impl Ordered for Character {}

impl PartialOrd for Character {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Any for Character {
    fn is_equal(&self, other: &dyn std::any::Any) -> bool {
        if let Some(other) = other.downcast_ref::<Character>() {
            self.value == other.value
        } else {
            false
        }
    }
}

impl PartialEq for Character {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Character {}

impl std::fmt::Debug for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}