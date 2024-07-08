/// Any is the root of the openEHR type hierarchy. It defines the is_equal method that is used to
/// compare two objects of any type and support for the PartialEq and Eq traits implementing the
/// equality operators == and !=.
pub trait Any: std::any::Any + PartialEq + Eq {
    fn is_equal(&self, other: &dyn std::any::Any) -> bool;
}