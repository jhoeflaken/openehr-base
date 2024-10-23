use crate::foundation_types::{Any, String};
use super::terminology_code::TerminologyCode;

pub struct TerminologyTerm {
    pub concept: TerminologyCode,
    pub text: String,
}

impl PartialEq for TerminologyTerm {

    fn eq(&self, other: &Self) -> bool {
        self.concept == other.concept &&
        self.text == other.text
    }

}

impl Any for TerminologyTerm {

    fn is_equal(&self, other: &Self) -> bool {
        self.concept == other.concept &&
        self.text == other.text
    }

    fn instance_of(&self, type_name: &str) -> bool {
        "TerminologyTerm" == type_name
    }

    fn type_of(&self) -> std::string::String {
        "TerminologyTerm".to_string()
    }

}