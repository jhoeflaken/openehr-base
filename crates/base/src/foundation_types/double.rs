use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::foundation_types::{Any, Integer, Numeric, OrderNumeric, Ordered};

pub struct Double {
    pub value: f64,
}

impl Double {

    pub fn new(value: f64) -> Self {
        Double {
            value
        }
    }

    pub fn floor(&self) -> Integer {
        Integer::new(self.value.floor() as i32)
    }

}

impl Any for Double {

    fn is_equal(&self, other: &Self) -> bool {
        self.value == other.value
    }

    fn instance_of(&self, type_name: &str) -> bool {
        type_name == "Double"
    }

    fn type_of(&self) -> String {
        "Double".to_string()
    }

}

impl Numeric for Double {

    fn pow(&self, exponent: &Double) -> Double {
        Double::new((self.value).powf(exponent.value))
    }

}

impl Ordered for Double {
}

impl PartialOrd for Double {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.value.total_cmp(&other.value))
    }

}

impl Ord for Double {

    fn cmp(&self, other: &Self) -> Ordering {
        self.value.total_cmp(&other.value)
    }

}

impl PartialEq for Double {

    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }

}

impl Eq for Double {
}

impl Add for Double {
    type Output = Double;

    fn add(self, other: Self) -> Double {
        Double::new(self.value + other.value)
    }

}

impl Sub for Double {
    type Output = Double;

    fn sub(self, other: Self) -> Double {
        Double::new(self.value - other.value)
    }

}

impl Div for Double {
    type Output = Double;

    fn div(self, other: Self) -> Double {
        Double::new((self.value) / (other.value))
    }

}

impl Mul for Double {
    type Output = Double;

    fn mul(self, other: Self) -> Double {
        Double::new(self.value * other.value)
    }

}

impl Neg for Double {
    type Output = Double;

    fn neg(self) -> Double {
        Double::new(-self.value)
    }

}

impl OrderNumeric for Double {
}
