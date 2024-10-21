use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use crate::foundation_types::{Any, Numeric, Ordered, OrderNumeric, Double};

pub struct Integer {
    pub value: i32,
}

impl Integer {

    pub fn new(value: i32) -> Self {
        Integer {
            value
        }
    }

}

impl Any for Integer {

    fn is_equal(&self, other: &Self) -> bool {
        self.value == other.value
    }

    fn instance_of(&self, type_name: &str) -> bool {
        type_name == "Integer"
    }

    fn type_of(&self) -> String {
        "Integer".to_string()
    }

}

impl Numeric for Integer {

    fn pow(&self, exponent: &Double) -> Double {
        Double::new((self.value as f64).powf(exponent.value))
    }

}

impl Ordered for Integer {
}

impl PartialOrd for Integer {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.value.cmp(&other.value))
    }

}

impl Ord for Integer {

    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }

}

impl PartialEq for Integer {

    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }

}

impl Eq for Integer {
}

impl Add for Integer {
    type Output = Integer;

    fn add(self, other: Self) -> Integer {
        Integer::new(self.value + other.value)
    }

}

impl Sub for Integer {
    type Output = Integer;

    fn sub(self, other: Self) -> Integer {
        Integer::new(self.value - other.value)
    }

}

impl Div for Integer {
    type Output = Double;

    fn div(self, other: Self) -> Double {
        Double::new((self.value as f64) / (other.value as f64))
    }

}

impl Mul for Integer {
    type Output = Integer;

    fn mul(self, other: Self) -> Integer {
        Integer::new(self.value * other.value)
    }

}

impl Neg for Integer {
    type Output = Integer;

    fn neg(self) -> Integer {
        Integer::new(-self.value)
    }

}

impl OrderNumeric for Integer {
}

impl Rem for Integer {
    type Output = Integer;

    fn rem(self, other: Self) -> Self {
        Integer::new(self.value % other.value)
    }

}