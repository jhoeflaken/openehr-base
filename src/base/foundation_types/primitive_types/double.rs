use crate::base::foundation_types::primitive_types::any::Any;
use crate::base::foundation_types::primitive_types::numeric::Numeric;
use crate::base::foundation_types::primitive_types::ordered::Ordered;
use crate::base::foundation_types::primitive_types::ordered_numeric::OrderedNumeric;

pub struct Double {
    value: f64,
}

impl Double {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl OrderedNumeric for Double {}

impl Numeric for Double {}

impl std::ops::Add for Double {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.value + other.value)
    }
}

impl std::ops::Sub for Double {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.value - other.value)
    }
}

impl std::ops::Mul for Double {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(self.value * other.value)
    }
}

impl std::ops::Div for Double {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self::new(self.value / other.value)
    }
}

impl std::ops::Neg for Double {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.value)
    }
}

impl Ordered for Double {}

impl PartialOrd for Double {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
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