use crate::detect::Mode;

pub fn generate_profiles_toml(_mode: Mode) -> String {
    format!(r#"
[profile.canonrs-ssr]
inherits = "release"
lto = false           # REQUIRED: Children + SSR + LTO = linker crash
strip = false         # REQUIRED: trait objects need symbols
codegen-units = 16    # REQUIRED: LLVM needs space for anon closures
opt-level = 2

[profile.canonrs-csr]
inherits = "release"
lto = "thin"
strip = "symbols"
codegen-units = 1
opt-level = "z"

[profile.canonrs-hybrid]
inherits = "release"
lto = false
strip = false
codegen-units = 16
opt-level = 2
"#)
}
