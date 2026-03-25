//! @canon-level: strict
//! @canon-owner: primitives-team
//! BehaviorEngine — define behaviors formais por categoria
//! É o contrato entre o Rust primitive e o JS behavior layer

/// Behaviors suportados pelo sistema CanonRS
#[derive(Debug, Clone, PartialEq)]
pub enum Behavior {
    /// Open/close com focus trap e keyboard ESC
    Overlay,
    /// Seleção simples ou múltipla com teclado
    Selection,
    /// Navegação entre items com arrow keys
    Navigation,
    /// Drag e resize de painéis
    Resize,
    /// Toggle on/off
    Toggle,
    /// Expand/collapse com animação
    Disclosure,
}

impl Behavior {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Overlay    => "overlay",
            Self::Selection  => "selection",
            Self::Navigation => "navigation",
            Self::Resize     => "resize",
            Self::Toggle     => "toggle",
            Self::Disclosure => "disclosure",
        }
    }

    /// Retorna o atributo data-rs-behavior para o DOM
    pub fn data_attr(&self) -> (&'static str, &'static str) {
        ("data-rs-behavior", self.as_str())
    }
}

/// Mapa de componente → behaviors esperados
pub fn component_behaviors(name: &str) -> &'static [Behavior] {
    match name {
        "Dialog" | "Drawer" | "Sheet" | "Modal" | "AlertDialog" =>
            &[Behavior::Overlay],
        "Select" | "Combobox" | "RadioGroup" | "ToggleGroup" =>
            &[Behavior::Selection],
        "Tabs" | "NavigationMenu" | "Menubar" =>
            &[Behavior::Navigation],
        "Accordion" =>
            &[Behavior::Navigation, Behavior::Disclosure],
        "Resizable" =>
            &[Behavior::Resize],
        "Toggle" | "Switch" | "Checkbox" =>
            &[Behavior::Toggle],
        "Collapsible" =>
            &[Behavior::Disclosure],
        _ => &[],
    }
}
