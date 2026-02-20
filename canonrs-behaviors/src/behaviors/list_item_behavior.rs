#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;

/// list_item_behavior delega ao list_behavior.
/// Itens individuais são controlados pelo root [data-list].
/// Este behavior existe para compatibilidade com o registry.
#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-list-item", Box::new(|_id: &str, _state: &ComponentState| -> BehaviorResult<()> {
        // Controlado pelo list_behavior via root [data-list]
        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
