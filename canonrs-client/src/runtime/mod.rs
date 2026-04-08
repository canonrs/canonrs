//! CanonRS Runtime — detector e router de grupos de interação.

pub mod detect;
pub mod init;

/// Ponto de entrada único. Chamado após hydrate_islands().
#[cfg(target_arch = "wasm32")]
pub fn init() {
    let groups = detect::detect_groups();
    init::init_all(groups);
}
