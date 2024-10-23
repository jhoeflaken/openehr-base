
use crate::foundation_types::{Any, String, Uri};

pub struct TerminologyCode {
    pub terminology_id: String,
    pub terminology_version: String,
    pub code_string: String,
    pub uri: Uri
}

impl PartialEq for TerminologyCode {
    fn eq(&self, other: &Self) -> bool {
        self.terminology_id == other.terminology_id &&
        self.terminology_version == other.terminology_version &&
        self.code_string == other.code_string &&
        self.uri == other.uri
    }
}

impl Any for TerminologyCode {

    fn is_equal(&self, other: &Self) -> bool {
        self.terminology_id == other.terminology_id &&
        self.terminology_version == other.terminology_version &&
        self.code_string == other.code_string &&
        self.uri == other.uri
    }

    fn instance_of(&self, type_name: &str) -> bool {
        "TerminologyCode" == type_name
    }

    fn type_of(&self) -> std::string::String {
        "TerminologyCode".to_string()
    }

}