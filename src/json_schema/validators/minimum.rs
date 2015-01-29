use serialize::json;

use super::super::errors;
use super::super::scope;

#[allow(missing_copy_implementations)]
pub struct Minimum {
    pub number: f64,
    pub exclusive: bool
}

impl super::Validator for Minimum {
    fn validate(&self, val: &json::Json, path: &str, strict: bool, _scope: &scope::Scope) -> super::ValidatorResult {
        let number = strict_process!(val.as_f64(), path, strict, "The value must be a number");

        let valid = if self.exclusive {
            number > self.number
        } else {
            number >= self.number
        };

        if valid {
            Ok(())
        } else {
            Err(val_error!(
                errors::Minimum {
                    path: path.to_string()
                }
            ))
        }
    }
}