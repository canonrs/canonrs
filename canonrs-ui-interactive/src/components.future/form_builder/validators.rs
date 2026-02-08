use super::types::{ValidationRule, ValidationError, FieldDef};
use regex::Regex;

/// Validate a single field against all its validation rules
pub fn validate_field(field: &FieldDef, value: &str) -> Vec<ValidationError> {
    let mut errors = vec![];

    for rule in &field.validations {
        if let Err(message) = apply_validation_rule(value, rule) {
            errors.push(ValidationError {
                field_id: field.id.clone(),
                message,
            });
        }
    }

    errors
}

/// Apply a single validation rule
fn apply_validation_rule(value: &str, rule: &ValidationRule) -> Result<(), String> {
    match rule {
        ValidationRule::Required => {
            if value.trim().is_empty() {
                Err("This field is required".to_string())
            } else {
                Ok(())
            }
        }

        ValidationRule::MinLength(min) => {
            if value.len() < *min {
                Err(format!("Must be at least {} characters", min))
            } else {
                Ok(())
            }
        }

        ValidationRule::MaxLength(max) => {
            if value.len() > *max {
                Err(format!("Must be at most {} characters", max))
            } else {
                Ok(())
            }
        }

        ValidationRule::Min(min_val) => {
            match value.parse::<f64>() {
                Ok(num) if num < *min_val => {
                    Err(format!("Must be at least {}", min_val))
                }
                Ok(_) => Ok(()),
                Err(_) => Err("Must be a valid number".to_string()),
            }
        }

        ValidationRule::Max(max_val) => {
            match value.parse::<f64>() {
                Ok(num) if num > *max_val => {
                    Err(format!("Must be at most {}", max_val))
                }
                Ok(_) => Ok(()),
                Err(_) => Err("Must be a valid number".to_string()),
            }
        }

        ValidationRule::Pattern(pattern) => {
            let re: regex::Regex = match Regex::new(pattern) {
                Ok(r) => r,
                Err(_) => return Err("Invalid regex pattern".to_string()),
            };

            if re.is_match(value) {
                Ok(())
            } else {
                Err("Invalid format".to_string())
            }
        }
        ValidationRule::Email => {
            let email_regex = Regex::new(
                r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"
            ).unwrap();

            if email_regex.is_match(value) {
                Ok(())
            } else {
                Err("Must be a valid email address".to_string())
            }
        }

        ValidationRule::Url => {
            let url_regex = Regex::new(
                r"^https?://[^\s/$.?#].[^\s]*$"
            ).unwrap();

            if url_regex.is_match(value) {
                Ok(())
            } else {
                Err("Must be a valid URL".to_string())
            }
        }

        ValidationRule::Custom(validator_fn) => {
            validator_fn(value)
        }
    }
}

/// Check if field dependency condition is met
pub fn check_dependency(
    dependency: &super::types::FieldDependency,
    form_values: &super::types::FormValues,
) -> bool {
    use super::types::DependencyCondition;

    let field_value = form_values.get(&dependency.field_id).map(|s| s.as_str()).unwrap_or("");

    match &dependency.condition {
        DependencyCondition::Equals(expected) => field_value == expected,
        DependencyCondition::NotEquals(expected) => field_value != expected,
        DependencyCondition::Contains(substring) => field_value.contains(substring),
        DependencyCondition::GreaterThan(threshold) => {
            field_value.parse::<f64>().map(|v| v > *threshold).unwrap_or(false)
        }
        DependencyCondition::LessThan(threshold) => {
            field_value.parse::<f64>().map(|v| v < *threshold).unwrap_or(false)
        }
        DependencyCondition::IsEmpty => field_value.is_empty(),
        DependencyCondition::IsNotEmpty => !field_value.is_empty(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_required_validation() {
        let rule = ValidationRule::Required;
        assert!(apply_validation_rule("", &rule).is_err());
        assert!(apply_validation_rule("  ", &rule).is_err());
        assert!(apply_validation_rule("value", &rule).is_ok());
    }

    #[test]
    fn test_min_length_validation() {
        let rule = ValidationRule::MinLength(5);
        assert!(apply_validation_rule("abc", &rule).is_err());
        assert!(apply_validation_rule("abcde", &rule).is_ok());
        assert!(apply_validation_rule("abcdef", &rule).is_ok());
    }

    #[test]
    fn test_email_validation() {
        let rule = ValidationRule::Email;
        assert!(apply_validation_rule("invalid", &rule).is_err());
        assert!(apply_validation_rule("test@", &rule).is_err());
        assert!(apply_validation_rule("test@example.com", &rule).is_ok());
        assert!(apply_validation_rule("user+tag@example.co.uk", &rule).is_ok());
    }

    #[test]
    fn test_url_validation() {
        let rule = ValidationRule::Url;
        assert!(apply_validation_rule("invalid", &rule).is_err());
        assert!(apply_validation_rule("http://", &rule).is_err());
        assert!(apply_validation_rule("http://example.com", &rule).is_ok());
        assert!(apply_validation_rule("https://example.com/path?query=1", &rule).is_ok());
    }

    #[test]
    fn test_number_range_validation() {
        let min_rule = ValidationRule::Min(10.0);
        let max_rule = ValidationRule::Max(100.0);

        assert!(apply_validation_rule("5", &min_rule).is_err());
        assert!(apply_validation_rule("10", &min_rule).is_ok());
        assert!(apply_validation_rule("50", &min_rule).is_ok());

        assert!(apply_validation_rule("50", &max_rule).is_ok());
        assert!(apply_validation_rule("100", &max_rule).is_ok());
        assert!(apply_validation_rule("101", &max_rule).is_err());
    }
}
