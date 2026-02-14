use std::fs;
use std::path::Path;

pub fn generate(output_dir: &Path) {
    let css = r#"/* ======================================================================
   ROOT TOKENS — Final visual contract
   Consumes semantic layer, never theme directly
   ====================================================================== */

[data-theme] {
  --root-bg: var(--color-background);
  --root-fg: var(--color-foreground);
  --root-border: var(--color-border);
}
"#;

    fs::write(output_dir.join("root.css"), css).ok();
    println!("  ✓ root.css (via semantic)");
}
