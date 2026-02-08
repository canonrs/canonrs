pub mod core;
pub mod families;
pub mod primitives;
pub mod semantics;

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
