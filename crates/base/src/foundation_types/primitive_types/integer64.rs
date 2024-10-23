use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use crate::foundation_types::{Any, Numeric, Ordered, OrderNumeric, Double};

pub struct Integer64 {
    pub value: i64,
}

impl Integer64 {

    pub fn new(value: i64) -> Self {
        Integer64 {
            value
        }
    }

}

impl Any for Integer64 {

    fn is_equal(&self, other: &Self) -> bool {
        self.value == other.value
    }

    fn instance_of(&self, type_name: &str) -> bool {
        type_name == "Integer64"
    }

    fn type_of(&self) -> String {
        "Integer64".to_string()
    }

}

impl Numeric for Integer64 {

    fn pow(&self, exponent: &Double) -> Double {
        Double::new((self.value as f64).powf(exponent.value))
    }

}

impl Ordered for Integer64 {
}

impl PartialOrd for Integer64 {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.value.cmp(&other.value))
    }

}

impl Ord for Integer64 {

    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }

}

impl PartialEq for Integer64 {

    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }

}

impl Eq for Integer64 {
}

impl Add for Integer64 {
    type Output = Integer64;

    fn add(self, other: Self) -> Integer64 {
        Integer64::new(self.value + other.value)
    }

}

impl Sub for Integer64 {
    type Output = Integer64;

    fn sub(self, other: Self) -> Integer64 {
        Integer64::new(self.value - other.value)
    }

}

impl Div for Integer64 {
    type Output = Double;

    fn div(self, other: Self) -> Double {
        Double::new((self.value as f64) / (other.value as f64))
    }

}

impl Mul for Integer64 {
    type Output = Integer64;

    fn mul(self, other: Self) -> Integer64 {
        Integer64::new(self.value * other.value)
    }

}

impl Neg for Integer64 {
    type Output = Integer64;

    fn neg(self) -> Integer64 {
        Integer64::new(-self.value)
    }

}

impl OrderNumeric for Integer64 {
}

impl Rem for Integer64 {
    type Output = Integer64;

    fn rem(self, other: Self) -> Self {
        Integer64::new(self.value % other.value)
    }

}