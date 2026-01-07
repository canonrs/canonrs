use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Field type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FieldType {
    Text,
    Email,
    Password,
    Number,
    Tel,
    Url,
    TextArea,
    Select,
    MultiSelect,
    Checkbox,
    Radio,
    Date,
    DateTime,
    Time,
    File,
    Color,
    Range,
}

/// Validation rule
#[derive(Clone, Debug)]
pub enum ValidationRule {
    Required,
    MinLength(usize),
    MaxLength(usize),
    Min(f64),
    Max(f64),
    Pattern(String),
    Email,
    Url,
    Custom(fn(&str) -> Result<(), String>),
}

/// Field definition
#[derive(Clone)]
pub struct FieldDef {
    pub id: String,
    pub label: String,
    pub field_type: FieldType,
    pub placeholder: Option<String>,
    pub default_value: Option<String>,
    pub required: bool,
    pub validations: Vec<ValidationRule>,
    pub options: Vec<SelectOption>,
    pub help_text: Option<String>,
    pub depends_on: Option<FieldDependency>,
}

impl FieldDef {
    pub fn new(id: impl Into<String>, label: impl Into<String>, field_type: FieldType) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            field_type,
            placeholder: None,
            default_value: None,
            required: false,
            validations: vec![],
            options: vec![],
            help_text: None,
            depends_on: None,
        }
    }
    
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
        self
    }
    
    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.default_value = Some(value.into());
        self
    }
    
    pub fn required(mut self) -> Self {
        self.required = true;
        self.validations.push(ValidationRule::Required);
        self
    }
    
    pub fn min_length(mut self, len: usize) -> Self {
        self.validations.push(ValidationRule::MinLength(len));
        self
    }
    
    pub fn max_length(mut self, len: usize) -> Self {
        self.validations.push(ValidationRule::MaxLength(len));
        self
    }
    
    pub fn pattern(mut self, regex: impl Into<String>) -> Self {
        self.validations.push(ValidationRule::Pattern(regex.into()));
        self
    }
    
    pub fn email(mut self) -> Self {
        self.validations.push(ValidationRule::Email);
        self
    }
    
    pub fn min(mut self, value: f64) -> Self {
        self.validations.push(ValidationRule::Min(value));
        self
    }
    
    pub fn max(mut self, value: f64) -> Self {
        self.validations.push(ValidationRule::Max(value));
        self
    }
    
    pub fn options(mut self, opts: Vec<SelectOption>) -> Self {
        self.options = opts;
        self
    }
    
    pub fn help_text(mut self, text: impl Into<String>) -> Self {
        self.help_text = Some(text.into());
        self
    }
    
    pub fn depends_on(mut self, dependency: FieldDependency) -> Self {
        self.depends_on = Some(dependency);
        self
    }
}

/// Select option
#[derive(Clone, Debug)]
pub struct SelectOption {
    pub label: String,
    pub value: String,
}

impl SelectOption {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
        }
    }
}

/// Field dependency (conditional rendering)
#[derive(Clone, Debug)]
pub struct FieldDependency {
    pub field_id: String,
    pub condition: DependencyCondition,
}

#[derive(Clone, Debug)]
pub enum DependencyCondition {
    Equals(String),
    NotEquals(String),
    Contains(String),
    GreaterThan(f64),
    LessThan(f64),
    IsEmpty,
    IsNotEmpty,
}

/// Form schema
#[derive(Clone)]
pub struct FormSchema {
    pub id: String,
    pub title: String,
    pub fields: Vec<FieldDef>,
    pub submit_label: String,
    pub cancel_label: Option<String>,
}

impl FormSchema {
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            fields: vec![],
            submit_label: "Submit".to_string(),
            cancel_label: None,
        }
    }
    
    pub fn field(mut self, field: FieldDef) -> Self {
        self.fields.push(field);
        self
    }
    
    pub fn submit_label(mut self, label: impl Into<String>) -> Self {
        self.submit_label = label.into();
        self
    }
    
    pub fn cancel_label(mut self, label: impl Into<String>) -> Self {
        self.cancel_label = Some(label.into());
        self
    }
}

/// Form values
pub type FormValues = HashMap<String, String>;

/// Validation error
#[derive(Clone, Debug)]
pub struct ValidationError {
    pub field_id: String,
    pub message: String,
}

/// Form state
#[derive(Clone, Debug)]
pub struct FormState {
    pub values: FormValues,
    pub errors: Vec<ValidationError>,
    pub touched: HashMap<String, bool>,
    pub is_submitting: bool,
    pub is_valid: bool,
}

impl Default for FormState {
    fn default() -> Self {
        Self {
            values: HashMap::new(),
            errors: vec![],
            touched: HashMap::new(),
            is_submitting: false,
            is_valid: false,
        }
    }
}
