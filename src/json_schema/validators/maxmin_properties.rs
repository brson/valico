use serde_json::{Value};

use super::super::errors;
use super::super::scope;

#[allow(missing_copy_implementations)]
pub struct MaxProperties {
    pub fragment: Vec<String>,
    pub length: u64
}

impl super::Validator for MaxProperties {
    fn validate(&self, val: &Value, path: &str, _scope: &scope::Scope) -> super::ValidationState {
        let object = nonstrict_process!(val.as_object(), path);

        if (object.len() as u64) <= self.length {
            super::ValidationState::new()
        } else {
            val_error!(
                errors::MaxProperties {
                    fragment: self.fragment.clone(),
                    path: path.to_string()
                }
            )
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct MinProperties {
    pub fragment: Vec<String>,
    pub length: u64
}

impl super::Validator for MinProperties {
    fn validate(&self, val: &Value, path: &str, _scope: &scope::Scope) -> super::ValidationState {
        let object = nonstrict_process!(val.as_object(), path);

        if (object.len() as u64) >= self.length {
            super::ValidationState::new()
        } else {
            val_error!(
                errors::MinProperties {
                    fragment: self.fragment.clone(),
                    path: path.to_string()
                }
            )
        }
    }
}
