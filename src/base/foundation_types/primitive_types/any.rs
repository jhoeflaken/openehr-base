

pub trait Any: std::any::Any + PartialEq + Eq {

    fn is_equal(&self, other: &dyn std::any::Any) -> bool;

}