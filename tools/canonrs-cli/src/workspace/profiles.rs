use crate::detect::Mode;

pub fn generate_profiles_toml(mode: Mode) -> String {
    match mode {
        Mode::Ssr => SSR_PROFILE.to_string(),
        Mode::Csr => CSR_PROFILE.to_string(),
        Mode::Hybrid => HYBRID_PROFILE.to_string(),
    }
}

const SSR_PROFILE: &str = r#"
[profile.canonrs-ssr]
inherits = "release"
lto = false
strip = false
codegen-units = 16
opt-level = 2
"#;

const CSR_PROFILE: &str = r#"
[profile.canonrs-csr]
inherits = "release"
lto = "thin"
strip = "symbols"
codegen-units = 1
opt-level = "z"
"#;

const HYBRID_PROFILE: &str = r#"
[profile.canonrs-hybrid]
inherits = "release"
lto = false
strip = false
codegen-units = 16
opt-level = 2
"#;
