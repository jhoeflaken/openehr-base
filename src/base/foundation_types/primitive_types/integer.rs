use crate::base::foundation_types::primitive_types::any::Any;

pub struct Integer {
    value: i32,
}

impl Integer {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}

impl Any for Integer {
    fn is_equal(&self, other: &dyn std::any::Any) -> bool {
        if let Some(other) = other.downcast_ref::<Integer>() {
            self.value == other.value
        } else {
            false
        }
    }
}

impl PartialEq for Integer {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Integer {}

impl std::fmt::Debug for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}