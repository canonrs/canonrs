use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct HSLColor {
    pub h: f32,
    pub s: f32,
    pub l: f32,
}

pub struct ThemeColors {
    pub light: HashMap<String, HSLColor>,
    pub dark: HashMap<String, HSLColor>,
}

fn normalize_theme_key(css_key: &str) -> String {
    let mappings: HashMap<&str, &str> = [
        // Surfaces
        ("background", "surface-bg"),
        ("foreground", "surface-fg"),
        ("card", "surface-elevated"),
        ("card-foreground", "surface-elevated-fg"),
        ("muted", "surface-muted"),
        ("muted-foreground", "surface-fg-muted"),
        ("border", "surface-border"),
        ("input", "surface-border"),

        // Actions
        ("primary", "action-primary-bg"),
        ("primary-foreground", "action-primary-fg"),
        ("secondary", "action-secondary-bg"),
        ("secondary-foreground", "action-secondary-fg"),
        ("accent", "action-accent-bg"),
        ("accent-foreground", "action-accent-fg"),
        ("ring", "action-focus-ring"),

        // States
        ("destructive", "state-error-bg"),
        ("destructive-foreground", "state-error-fg"),

        // Success
        ("success", "state-success-bg"),
        ("success-foreground", "state-success-fg"),
        ("success-border", "state-success-border"),

        // Warning
        ("warning", "state-warning-bg"),
        ("warning-foreground", "state-warning-fg"),
        ("warning-border", "state-warning-border"),

        // Info
        ("info", "state-info-bg"),
        ("info-foreground", "state-info-fg"),

        // Overlays
        ("popover", "overlay-bg"),
        ("popover-foreground", "overlay-fg"),

        // Charts
        ("chart-1", "chart-1"),
        ("chart-2", "chart-2"),
        ("chart-3", "chart-3"),
        ("chart-4", "chart-4"),
        ("chart-5", "chart-5"),

        // Sidebar
        ("sidebar", "sidebar-bg"),
        ("sidebar-foreground", "sidebar-fg"),
        ("sidebar-border", "sidebar-border"),
        ("sidebar-accent", "sidebar-accent-bg"),
        ("sidebar-accent-foreground", "sidebar-accent-fg"),
        ("sidebar-primary", "sidebar-primary-bg"),
        ("sidebar-primary-foreground", "sidebar-primary-fg"),
        ("sidebar-ring", "sidebar-ring"),

        ("shadow-color", "shadow-color"),
    ].iter().copied().collect();

    mappings.get(css_key)
        .map(|s| s.to_string())
        .unwrap_or_else(|| css_key.to_string())
}

pub fn parse_hsl(value: &str) -> Option<HSLColor> {
    let value = value.trim().trim_start_matches("hsl(").trim_end_matches(')');
    let parts: Vec<&str> = value.split_whitespace().collect();
    if parts.len() != 3 { return None; }

    let h = parts[0].parse::<f32>().ok()?;
    let s = parts[1].trim_end_matches('%').parse::<f32>().ok()?;
    let l = parts[2].trim_end_matches('%').parse::<f32>().ok()?;

    Some(HSLColor { h, s, l })
}

pub fn parse_css_theme(content: &str) -> ThemeColors {
    let mut light = HashMap::new();
    let mut dark = HashMap::new();
    let mut in_root = false;
    let mut in_dark = false;

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with(":root") { in_root = true; in_dark = false; }
        else if line.starts_with(".dark") { in_dark = true; in_root = false; }
        else if line == "}" { in_root = false; in_dark = false; }
        else if (in_root || in_dark) && line.starts_with("--") {
            if let Some((key, value)) = line.split_once(':') {
                let key = key.trim().trim_start_matches("--").to_string();
                let value = value.trim().trim_end_matches(';');
                if let Some(color) = parse_hsl(value) {
                    if in_dark { dark.insert(key, color); }
                    else { light.insert(key, color); }
                }
            }
        }
    }

    ThemeColors { light, dark }
}

// ============================================================
// CANON CONTRACT: Ensure required states always exist
// States are part of CanonRS contract, not optional preset
// Success/Warning/Info use enterprise-standard colors
// ============================================================
fn ensure_required_states(colors: &mut HashMap<String, HSLColor>) {
    // Success: CanonRS standard green (enterprise contract)
    if !colors.contains_key("success") {
        colors.insert("success".to_string(), HSLColor { h: 142.0, s: 76.0, l: 36.0 });
    }
    if !colors.contains_key("success-foreground") {
        colors.insert("success-foreground".to_string(), HSLColor { h: 0.0, s: 0.0, l: 100.0 });
    }

    // Warning: CanonRS standard orange (enterprise contract)
    if !colors.contains_key("warning") {
        colors.insert("warning".to_string(), HSLColor { h: 38.0, s: 92.0, l: 50.0 });
    }
    if !colors.contains_key("warning-foreground") {
        colors.insert("warning-foreground".to_string(), HSLColor { h: 0.0, s: 0.0, l: 0.0 });
    }

    // Info: CanonRS standard blue (enterprise contract)
    if !colors.contains_key("info") {
        colors.insert("info".to_string(), HSLColor { h: 221.0, s: 83.0, l: 53.0 });
    }
    if !colors.contains_key("info-foreground") {
        colors.insert("info-foreground".to_string(), HSLColor { h: 0.0, s: 0.0, l: 100.0 });
    }
}

pub fn generate_css_theme(theme_name: &str, colors: &ThemeColors) -> String {
    let mut css = format!("/* {} - Normalized vocabulary */\n[data-theme=\"{}\"] {{\n", theme_name, theme_name);

    let mut light_colors = colors.light.clone();
    ensure_required_states(&mut light_colors);

    for (key, color) in &light_colors {
        let normalized = normalize_theme_key(key);
        css.push_str(&format!("  --theme-{}: hsl({} {}% {}%);\n", normalized, color.h, color.s, color.l));
    }

    css.push_str("}\n\n");

    if !colors.dark.is_empty() {
        css.push_str(&format!("[data-theme=\"{}\"].dark {{\n", theme_name));
        
        let mut dark_colors = colors.dark.clone();
        ensure_required_states(&mut dark_colors);

        for (key, color) in &dark_colors {
            let normalized = normalize_theme_key(key);
            css.push_str(&format!("  --theme-{}: hsl({} {}% {}%);\n", normalized, color.h, color.s, color.l));
        }
        css.push_str("}\n\n");
    }

    css
}

pub fn generate_themes(output_dir: &Path) {
    let themes_dir = Path::new("../canonrs-ui/themes-engine/ingest/css");
    if !themes_dir.exists() {
        println!("  ⚠ No themes directory found, skipping");
        return;
    }

    let mut themes_css = String::from("/* AUTO-GENERATED - THEME LAYER (normalized vocabulary) */\n\n");

    for entry in fs::read_dir(themes_dir).expect("Failed to read themes") {
        let path = entry.expect("Entry error").path();
        if path.extension().and_then(|s| s.to_str()) != Some("css") { continue; }

        let theme_name = path.file_stem().and_then(|s| s.to_str()).expect("Invalid name");
        let content = fs::read_to_string(&path).expect("Read failed");
        let colors = parse_css_theme(&content);

        themes_css.push_str(&generate_css_theme(theme_name, &colors));
    }

    fs::write(output_dir.join("themes.css"), themes_css).expect("Write failed");
    println!("  ✓ themes.css (normalized)");
}
