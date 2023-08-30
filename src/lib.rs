mod utils;

use std::str::FromStr;
use wasm_bindgen::prelude::*;
use cedar_policy::{Schema, PolicySet, Validator, ValidationMode};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct ValidationResult {
    pub valid: bool,
    error: Option<String>,
}

#[wasm_bindgen]
impl ValidationResult {
    #[wasm_bindgen(getter)]
    pub fn error(&self) -> Option<String> {
        self.error.clone()
    }
}

#[wasm_bindgen(js_name=validateSyntax)]
pub fn validate_syntax(policy: &str) -> ValidationResult {
    return match PolicySet::from_str(policy) {
        Ok(_) => ValidationResult { 
            valid: true,
            error: None
        },
        Err(errors) => {
            let errors = errors.errors_as_strings();
            return ValidationResult { 
                valid: false,
                error: Some(errors.join("\n"))
            }
        }
    };
}

#[wasm_bindgen(js_name=validatePolicy)]
pub fn validate_policy(schema: &str, policy: &str) -> ValidationResult {
    println!("Validating policy");
    let schema = match Schema::from_str(schema) {
        Ok(schema) => schema,
        Err(..) => {
            return ValidationResult { 
                valid: false,
                error: Some("Invalid schema".to_string())
            }
        }
    };

    let validator = Validator::new(schema);
    let policy_set = match PolicySet::from_str(policy) {
        Ok(policy_set) => policy_set,
        Err(..) => {
            return ValidationResult { 
                valid: false,
                error: Some("Failed to create policy set".to_string())
            }
        }
    };

    let result = validator.validate(&policy_set, ValidationMode::Strict);
    if result.validation_passed() {
        return ValidationResult { 
            valid: true,
            error: None
        }
    } else {
        let mut error = String::new();

        for err in result.validation_errors() {
            error.push_str(&format!("{}\n", err));
        }

        return ValidationResult { 
            valid: false,
            error: Some(error)
        }
    }
}
