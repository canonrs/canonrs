//! Accessibility helpers - ARIA attributes

pub fn aria_invalid(error: bool) -> Option<&'static str> {
    if error {
        Some("true")
    } else {
        None
    }
}

pub fn aria_disabled(disabled: bool) -> Option<&'static str> {
    if disabled {
        Some("true")
    } else {
        None
    }
}

pub fn aria_busy(loading: bool) -> Option<&'static str> {
    if loading {
        Some("true")
    } else {
        None
    }
}

pub fn aria_describedby(id: Option<&str>) -> Option<String> {
    id.map(|s| s.to_string())
}

pub fn role_alert() -> &'static str {
    "alert"
}
