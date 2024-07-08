use crate::base::foundation_types::primitive_types::any::Any;
use crate::base::foundation_types::primitive_types::numeric::Numeric;
use crate::base::foundation_types::primitive_types::ordered::Ordered;
use crate::base::foundation_types::primitive_types::ordered_numeric::OrderedNumeric;

pub struct Integer {
    pub value: i32,
}

impl Integer {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}

impl OrderedNumeric for Integer {}

impl Numeric for Integer {}

impl std::ops::Add for Integer {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.value + other.value)
    }
}

impl std::ops::Sub for Integer {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.value - other.value)
    }
}

impl std::ops::Mul for Integer {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(self.value * other.value)
    }
}

impl std::ops::Div for Integer {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self::new(self.value / other.value)
    }
}

impl std::ops::Neg for Integer {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.value)
    }
}

impl Ordered for Integer {}

impl PartialOrd for Integer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
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