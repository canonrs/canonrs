pub mod primitives;
pub mod foundation;
pub mod semantics;
pub mod components;
pub mod state;
pub mod layers;

// Legacy — mantido para compatibilidade com tokens-engine durante migração
pub mod families;

/// Estrutura canônica para tokens de família
#[derive(Debug, Clone)]
pub struct FamilyToken {
    pub name: &'static str,
    pub value: &'static str,
}

impl FamilyToken {
    pub const fn new(name: &'static str, value: &'static str) -> Self {
        Self { name, value }
    }
}
