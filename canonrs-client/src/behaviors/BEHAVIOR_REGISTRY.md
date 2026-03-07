# Behavior Registry — Como Funciona

## Como Instanciar em um novo APP?

// src/lib.rs (ou main.rs se SSR)

#[cfg(feature = "hydrate")] #[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
console_error_panic_hook::set_once();
leptos::mount::hydrate_body(App);
canonrs::behaviors::init_canonrs_behaviors(); // ← ÚNICO IMPORT
}
Pronto. Não precisa mais nada.

✅ Auto-descobre todos behaviors via data-behavior-\*
✅ MutationObserver reentra automaticamente
✅ Zero configuração por componente

## Como deve ser construido um behavior?

#[cfg(feature = "hydrate")]
use super::\*; #[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;

#[cfg(feature = "hydrate")]
pub fn register() {
register_behavior("data-behavior-my-thing", Box::new(|element_id, state| {
let document = window().unwrap().document().unwrap();
let element = document.get_element_by_id(element_id)?;

        // Seu código aqui

        Ok(())
    }));

}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
Adicionar em behaviors/mod.rs:
rustpub mod my_behavior;
// Em register_all_behaviors():
my_behavior::register();

UI usa: <div id="x" data-behavior-my-thing />
