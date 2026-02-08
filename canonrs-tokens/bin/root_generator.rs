use std::fs;
use std::path::Path;

pub fn generate(output_dir: &Path) {
    let css = r#"/* ======================================================================
   ROOT TOKENS — Final visual contract
   This is the ONLY layer allowed to style html/body
   ====================================================================== */

[data-theme] {
  --root-bg: var(--theme-surface-bg);
  --root-fg: var(--theme-surface-fg);
  --root-border: var(--theme-surface-border);
}
"#;

    fs::write(output_dir.join("root.css"), css).ok();
    println!("  ✓ root.css");
}
