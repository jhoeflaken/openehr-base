use crate::base::foundation_types::primitive_types::any::Any;
use crate::base::foundation_types::primitive_types::numeric::Numeric;
use crate::base::foundation_types::primitive_types::ordered::Ordered;
use crate::base::foundation_types::primitive_types::ordered_numeric::OrderedNumeric;

pub struct Real {
    pub value: f32,
}

impl Real {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

impl OrderedNumeric for Real {}

impl Numeric for Real {}

impl std::ops::Add for Real {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.value + other.value)
    }
}

impl std::ops::Sub for Real {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.value - other.value)
    }
}

impl std::ops::Mul for Real {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(self.value * other.value)
    }
}

impl std::ops::Div for Real {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self::new(self.value / other.value)
    }
}

impl std::ops::Neg for Real {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.value)
    }
}

impl Ordered for Real {}

impl PartialOrd for Real {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
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