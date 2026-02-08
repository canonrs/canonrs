use leptos::prelude::*;
use std::collections::HashMap;
use super::types::{FormSchema, FormValues, ValidationError, FieldDef};
use super::validators::{validate_field, check_dependency};

#[derive(Clone)]
pub struct FormStateManager {
    pub values: RwSignal<FormValues>,
    pub errors: RwSignal<Vec<ValidationError>>,
    pub touched: RwSignal<HashMap<String, bool>>,
    pub is_submitting: RwSignal<bool>,
    schema: FormSchema,
}

impl FormStateManager {
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

    pub fn get_value(&self, field_id: &str) -> RwSignal<String> {
        let field_id = field_id.to_string();
        let values = self.values;

        RwSignal::new(
            values.get()
                .get(&field_id)
                .cloned()
                .unwrap_or_default()
        )
    }

    pub fn set_value(&self, field_id: String, value: String) {
        self.values.update(|vals| {
            vals.insert(field_id.clone(), value);
        });
        self.validate_field(&field_id);
    }

    pub fn get_all_values(&self) -> HashMap<String, String> {
        self.values.get()
    }

    pub fn get_errors(&self) -> RwSignal<Vec<ValidationError>> {
        self.errors
    }

    pub fn is_touched(&self, field_id: &str) -> RwSignal<bool> {
        let field_id = field_id.to_string();
        let touched = self.touched;

        RwSignal::new(
            touched.get()
                .get(&field_id)
                .copied()
                .unwrap_or(false)
        )
    }

    pub fn set_touched(&self, field_id: String) {
        self.touched.update(|t| {
            t.insert(field_id, true);
        });
    }

    pub fn validate_field(&self, field_id: &str) {
        let field = self.schema.fields.iter()
            .find(|f| f.id == field_id);

        if let Some(field) = field {
            let value = self.values.get()
                .get(field_id)
                .cloned()
                .unwrap_or_default();

            let field_errors = validate_field(field, &value);

            self.errors.update(|errors| {
                errors.retain(|e| e.field_id != field_id);
                errors.extend(field_errors);
            });
        }
    }

    pub fn validate_all(&self) -> bool {
        let mut all_errors = Vec::new();

        for field in &self.schema.fields {
            if self.is_field_visible(field) {
                let value = self.values.get()
                    .get(&field.id)
                    .cloned()
                    .unwrap_or_default();

                let field_errors = validate_field(field, &value);
                all_errors.extend(field_errors);
            }
        }

        let is_valid = all_errors.is_empty();
        self.errors.set(all_errors);
        is_valid
    }

    pub fn is_field_visible(&self, field: &FieldDef) -> bool {
        if let Some(dep) = &field.depends_on {
            let values = self.values.get();
            check_dependency(dep, &values)
        } else {
            true
        }
    }

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

    pub fn set_submitting(&self, submitting: bool) {
        self.is_submitting.set(submitting);
    }
}
impl FormStateManager { pub fn fields(&self) -> &[FieldDef] { &self.schema.fields } }
