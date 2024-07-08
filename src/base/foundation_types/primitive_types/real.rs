use crate::base::foundation_types::primitive_types::any::Any;

pub struct Real {
    value: f32,
}

impl Real {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

impl Any for Real {
    fn is_equal(&self, other: &dyn std::any::Any) -> bool {
        if let Some(other) = other.downcast_ref::<Real>() {
            self.value == other.value
        } else {
            false
        }
    }
}

impl PartialEq for Real {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Real {}

impl std::fmt::Debug for Real {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}