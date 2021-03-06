use serde_json::{Value};

use super::super::errors;
use super::super::scope;

#[allow(missing_copy_implementations)]
pub struct MaxItems {
    pub fragment: Vec<String>,
    pub length: u64
}

impl super::Validator for MaxItems {
    fn validate(&self, val: &Value, path: &str, _scope: &scope::Scope) -> super::ValidationState {
        let array = nonstrict_process!(val.as_array(), path);

        if (array.len() as u64) <= self.length {
            super::ValidationState::new()
        } else {
            val_error!(
                errors::MaxItems {
                    fragment: self.fragment.clone(),
                    path: path.to_string()
                }
            )
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct MinItems {
    pub fragment: Vec<String>,
    pub length: u64
}

impl super::Validator for MinItems {
    fn validate(&self, val: &Value, path: &str, _scope: &scope::Scope) -> super::ValidationState {
        let array = nonstrict_process!(val.as_array(), path);

        if (array.len() as u64) >= self.length {
            super::ValidationState::new()
        } else {
            val_error!(
                errors::MinItems {
                    fragment: self.fragment.clone(),
                    path: path.to_string()
                }
            )
        }
    }
}
