/// Combobox option data structure
#[derive(Clone, Debug, PartialEq)]
pub struct ComboboxOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl ComboboxOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }
    
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Selection mode (Família B)
#[derive(Clone, Copy, Default)]
pub enum ComboboxSelectionMode {
    #[default]
    Single,
    Multi,
}
