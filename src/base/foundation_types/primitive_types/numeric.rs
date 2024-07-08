use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::base::foundation_types::primitive_types::any::Any;

pub trait Numeric: Any + Add + Sub + Mul + Div + Neg + Sized {
}