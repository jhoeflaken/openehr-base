use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::foundation_types::{Any, Double, Integer, Numeric, OrderNumeric, Ordered};

pub struct Real {
    pub value: f32,
}

impl Real {

    pub fn new(value: f32) -> Self {
        Real {
            value
        }
    }

    pub fn floor(&self) -> Integer {
        Integer::new(self.value.floor() as i32)
    }

}

impl Any for Real {

    fn is_equal(&self, other: &Self) -> bool {
        self.value == other.value
    }

    fn instance_of(&self, type_name: &str) -> bool {
        type_name == "Real"
    }

    fn type_of(&self) -> String {
        "Real".to_string()
    }

}

impl Numeric for Real {

    fn pow(&self, exponent: &Double) -> Double {
        Double::new((self.value as f64).powf(exponent.value))
    }

}

impl Ordered for Real {
}

impl PartialOrd for Real {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.value.total_cmp(&other.value))
    }

}

impl Ord for Real {

    fn cmp(&self, other: &Self) -> Ordering {
        self.value.total_cmp(&other.value)
    }

}

impl PartialEq for Real {

    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }

}

impl Eq for Real {
}

impl Add for Real {
    type Output = Real;

    fn add(self, other: Self) -> Real {
        Real::new(self.value + other.value)
    }

}

impl Sub for Real {
    type Output = Real;

    fn sub(self, other: Self) -> Real {
        Real::new(self.value - other.value)
    }

}

impl Div for Real {
    type Output = Real;

    fn div(self, other: Self) -> Real {
        Real::new((self.value) / (other.value))
    }

}

impl Mul for Real {
    type Output = Real;

    fn mul(self, other: Self) -> Real {
        Real::new(self.value * other.value)
    }

}

impl Neg for Real {
    type Output = Real;

    fn neg(self) -> Real {
        Real::new(-self.value)
    }

}

impl OrderNumeric for Real {
}
