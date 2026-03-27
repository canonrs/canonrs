//! Pure utility functions for build modules

pub fn pascal_to_kebab(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 { result.push('-'); }
        result.push(c.to_lowercase().next().unwrap());
    }
    result
}

pub fn to_const_name(id: &str) -> String {
    id.replace('-', "_").to_uppercase()
}

pub fn to_title_case(s: &str) -> String {
    s.split('-')
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                None    => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn to_family(family: &str) -> &'static str {
    match family {
        "overlay"      => "ComponentFamily::Overlay",
        "input"        => "ComponentFamily::Input",
        "feedback"     => "ComponentFamily::Feedback",
        "navigation"   => "ComponentFamily::Navigation",
        "layout"       => "ComponentFamily::Layout",
        "data_display" => "ComponentFamily::DataDisplay",
        "typography"   => "ComponentFamily::Typography",
        "interactive"  => "ComponentFamily::Interactive",
        _              => "ComponentFamily::Utility",
    }
}

pub fn to_catalog_category(cat: &str) -> &'static str {
    match cat {
        "Action"     => "CatalogCategory::Action",
        "Display"    => "CatalogCategory::Display",
        "Feedback"   => "CatalogCategory::Feedback",
        "Form"       => "CatalogCategory::Form",
        "Navigation" => "CatalogCategory::Navigation",
        "Overlay"    => "CatalogCategory::Overlay",
        "Data"       => "CatalogCategory::Data",
        "Layout"     => "CatalogCategory::Layout",
        _            => "CatalogCategory::Display",
    }
}

pub fn prop_field_to_rust(field: &str) -> String {
    if field.starts_with("Select(") {
        let inner = field.trim_start_matches("Select(").trim_end_matches(')');
        let opts: Vec<String> = split_outside_parens(inner).into_iter()
            .map(|o: &str| {
                let kv: Vec<&str> = o.splitn(2, ':').collect();
                if kv.len() == 2 { format!("(\"{}\", \"{}\")", kv[0].trim(), kv[1].trim()) }
                else { format!("(\"{}\", \"{}\")", o.trim(), o.trim()) }
            })
            .collect();
        format!("PropFieldType::Select(&[{}])", opts.join(", "))
    } else {
        match field {
            "Text"   => "PropFieldType::Text".to_string(),
            "Number" => "PropFieldType::Number".to_string(),
            "Toggle" => "PropFieldType::Toggle".to_string(),
            "Color"  => "PropFieldType::Color".to_string(),
            _        => "PropFieldType::Text".to_string(),
        }
    }
}

/// Divide string por vírgulas ignorando vírgulas dentro de parênteses
fn split_outside_parens(s: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut depth  = 0usize;
    let mut start  = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => { if depth > 0 { depth -= 1; } }
            ',' if depth == 0 => {
                result.push(s[start..i].trim());
                start = i + 1;
            }
            _ => {}
        }
    }
    if start < s.len() { result.push(s[start..].trim()); }
    result
}
