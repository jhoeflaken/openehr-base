use super::{Any, Double};
use std::ops::{Add, Div, Mul, Neg, Sub};

/// This trait is the base trait for all numeric types in openEHR. It extends the `Any` trait and
/// adds numeric operations to it. See the
/// [openEHR specification](https://specifications.openehr.org/releases/BASE/Release-1.2.0/foundation_types.html#_numeric_class)
pub trait Numeric: Any + Add + Sub + Mul + Div + Neg + Sized {

    /// Returns the power of the number raised to the given exponent.
    fn pow(&self, exponent: &Double) -> Double;

}

