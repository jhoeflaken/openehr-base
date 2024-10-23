use std::cmp::PartialEq;

/// This is the base trait for all types in openEHR. It contains the bare minimum of operations
/// that all types must implement. See the
/// [openEHR specification](https://specifications.openehr.org/releases/BASE/Release-1.2.0/foundation_types.html#_the_any_class)
pub trait Any: PartialEq {

    /// Returns true if the two objects are equal, false otherwise.
    fn is_equal(&self, other: &Self) -> bool;

    /// Returns true if the object is an instance of the given type, false otherwise.
    fn instance_of(&self, type_name: &str) -> bool;

    /// Returns the type of the object as a string.
    fn type_of(&self) -> String;

}