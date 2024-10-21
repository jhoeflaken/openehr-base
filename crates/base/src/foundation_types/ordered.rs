use crate::foundation_types::Any;

pub trait Ordered: Any + PartialOrd + Ord {
}
