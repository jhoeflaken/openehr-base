use crate::base::foundation_types::primitive_types::any::Any;

pub struct Double {
    value: f64,
}

impl Double {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Any for Double {
    fn is_equal(&self, other: &dyn std::any::Any) -> bool {
        if let Some(other) = other.downcast_ref::<Double>() {
            self.value == other.value
        } else {
            false
        }
    }
}

impl PartialEq for Double {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Double {}

impl std::fmt::Debug for Double {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}