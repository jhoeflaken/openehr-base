use crate::base::foundation_types::primitive_types::any::Any;

pub struct Integer64 {
    value: i64,
}

impl Integer64 {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}

impl Any for Integer64 {
    fn is_equal(&self, other: &dyn std::any::Any) -> bool {
        if let Some(other) = other.downcast_ref::<Integer64>() {
            self.value == other.value
        } else {
            false
        }
    }
}

impl PartialEq for Integer64 {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Integer64 {}

impl std::fmt::Debug for Integer64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}