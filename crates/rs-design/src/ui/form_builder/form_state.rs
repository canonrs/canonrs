use leptos::prelude::*;
use std::collections::HashMap;
use super::types::{FormSchema, FormValues, ValidationError, FormState, FieldDef};
use super::validators::{validate_field, check_dependency};

/// Form state manager - handles values, validation, touched fields
#[derive(Clone)]
pub struct FormStateManager {
    pub values: RwSignal<FormValues>,
    pub errors: RwSignal<Vec<ValidationError>>,
    pub touched: RwSignal<HashMap<String, bool>>,
    pub is_submitting: RwSignal<bool>,
    schema: FormSchema,
}

impl FormStateManager {
    /// Create new form state manager
    pub fn new(schema: FormSchema) -> Self {
        let mut initial_values = HashMap::new();
        for field in &schema.fields {
            if let Some(default) = &field.default_value {
                initial_values.insert(field.id.clone(), default.clone());
            } else {
                initial_values.insert(field.id.clone(), String::new());
            }
        }
        
        Self {
            values: RwSignal::new(initial_values),
            errors: RwSignal::new(vec![]),
            touched: RwSignal::new(HashMap::new()),
            is_submitting: RwSignal::new(false),
            schema,
        }
    }
    
    /// Get value for a field
    pub fn get_value(&self, field_id: &str) -> Signal<String> {
        let field_id = field_id.to_string();
        let values = self.values;
        Signal::derive(move || {
            values.get()
                .get(&field_id)
                .cloned()
                .unwrap_or_default()
        })
    }
    
    /// Set value for a field
    pub fn set_value(&self, field_id: String, value: String) {
        self.values.update(|values| {
            values.insert(field_id.clone(), value);
        });
        self.validate_field(&field_id);
    }
    
    /// Mark field as touched
    pub fn set_touched(&self, field_id: String) {
        self.touched.update(|touched| {
            touched.insert(field_id.clone(), true);
        });
        self.validate_field(&field_id);
    }
    
    /// Check if field is touched
    pub fn is_touched(&self, field_id: &str) -> Signal<bool> {
        let field_id = field_id.to_string();
        let touched = self.touched;
        Signal::derive(move || {
            touched.get()
                .get(&field_id)
                .copied()
                .unwrap_or(false)
        })
    }
    
    /// Get errors for a field
    pub fn get_errors(&self) -> Signal<Vec<ValidationError>> {
        let errors = self.errors;
        Signal::derive(move || errors.get())
    }
    
    /// Validate a single field
    fn validate_field(&self, field_id: &str) {
        if let Some(field) = self.schema.fields.iter().find(|f| f.id == field_id) {
            let value = self.values.get().get(field_id).cloned().unwrap_or_default();
            let field_errors = validate_field(field_id, &value, &field.validations);
            
            self.errors.update(|errors| {
                errors.retain(|e| e.field_id != field_id);
                errors.extend(field_errors);
            });
        }
    }
    
    /// Validate all fields
    pub fn validate_all(&self) -> bool {
        let mut all_errors = vec![];
        
        for field in &self.schema.fields {
            if let Some(dep) = &field.depends_on {
                if !check_dependency(dep, &self.values.get()) {
                    continue;
                }
            }
            
            let value = self.values.get().get(&field.id).cloned().unwrap_or_default();
            let field_errors = validate_field(&field.id, &value, &field.validations);
            all_errors.extend(field_errors);
        }
        
        let is_valid = all_errors.is_empty();
        self.errors.set(all_errors);
        
        self.touched.update(|touched| {
            for field in &self.schema.fields {
                touched.insert(field.id.clone(), true);
            }
        });
        
        is_valid
    }
    
    /// Check if field should be visible based on dependencies
    pub fn is_field_visible(&self, field: &FieldDef) -> Signal<bool> {
        let dependency = field.depends_on.clone();
        let values = self.values;
        Signal::derive(move || {
            if let Some(dep) = &dependency {
                check_dependency(dep, &values.get())
            } else {
                true
            }
        })
    }
    
    /// Get current form state snapshot
    pub fn get_state(&self) -> FormState {
        FormState {
            values: self.values.get(),
            errors: self.errors.get(),
            touched: self.touched.get(),
            is_submitting: self.is_submitting.get(),
            is_valid: self.errors.get().is_empty(),
        }
    }
    
    /// Reset form to initial values
    pub fn reset(&self) {
        let mut initial_values = HashMap::new();
        for field in &self.schema.fields {
            if let Some(default) = &field.default_value {
                initial_values.insert(field.id.clone(), default.clone());
            } else {
                initial_values.insert(field.id.clone(), String::new());
            }
        }
        
        self.values.set(initial_values);
        self.errors.set(vec![]);
        self.touched.set(HashMap::new());
        self.is_submitting.set(false);
    }
    
    /// Set form as submitting
    pub fn set_submitting(&self, submitting: bool) {
        self.is_submitting.set(submitting);
    }
    
    /// Get all form values
    pub fn get_all_values(&self) -> FormValues {
        self.values.get()
    }
    
    /// Set multiple values at once
    pub fn set_values(&self, values: FormValues) {
        self.values.set(values);
        for field_id in self.schema.fields.iter().map(|f| &f.id) {
            self.validate_field(field_id);
        }
    }
    
    /// Check if form is valid
    pub fn is_valid(&self) -> Signal<bool> {
        let errors = self.errors;
        Signal::derive(move || {
            errors.get().is_empty()
        })
    }
    
    /// Get form schema
    pub fn schema(&self) -> &FormSchema {
        &self.schema
    }
}

/// Hook to create form state manager
pub fn use_form_state(schema: FormSchema) -> FormStateManager {
    FormStateManager::new(schema)
}
