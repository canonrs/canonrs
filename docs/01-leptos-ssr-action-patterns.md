# Leptos SSR + CanonRS — Padrões e Armadilhas

## Problemas resolvidos
Dialog não fechava após action async. Inputs não limpavam após submit. Dialog recriado após re-render quebrava o runtime.

---

## use_action_result — hook Tier S

O Leptos popula `action.value()` com resultado do SSR durante hydrate. `version()` já começa em `1` se o SSR executou a action. `pending()` é `false` no momento do dispatch — só vira `true` após o primeiro poll.

O hook correto usa **trigger explícito** para separar evento de estado:

```rust
pub fn use_action_result<T, E, F>(
    action: Action<(), Result<T, E>>,
    on_result: F,
) -> RwSignal<u32>
```

O `RwSignal<u32>` retornado deve ser incrementado no `on:click`. O Effect só processa quando `trigger > last_handled` E `version > last_ver` E `!pending`. `untrack` obrigatório para evitar loop reativo.

Uso na page:
```rust
let create_trigger = canonrs::use_action_result(create, move |result| { ... });

on:click=move |_| {
    create_trigger.update(|v| *v += 1);
    create.dispatch(());
}
```

---

## Dialog e ConfirmDialog — isolamento obrigatório

Dialog NÃO pode viver dentro de componente que chama `.update()` em signals. Quando `projects.update()` roda, o Leptos re-renderiza o componente — destruindo o runtime do dialog.

✅ Extrair para componente separado no nível do `Page`:
```rust
#[component]
fn DialogNewProject(create_trigger: RwSignal<u32>, create: Action<...>) -> impl IntoView { ... }

#[component]
pub fn ProjectsPage() -> impl IntoView {
    // signals aqui — não em ProjectsList
    view! {
        <DialogNewProject ... />
        <ProjectsList ... />
    }
}
```

---

## Input — uncontrolled é o modelo correto

CanonRS é DOM-driven. Input controlado via signal cria duas fontes de verdade e quebra o contrato.

✅ Input uncontrolled — ler do DOM no submit:
```rust
let doc = web_sys::window().unwrap().document().unwrap();
let n = doc.query_selector("[data-rs-input][name='name']").ok().flatten()
    .and_then(|el: web_sys::Element| el.dyn_into::<web_sys::HtmlInputElement>().ok())
    .map(|el: web_sys::HtmlInputElement| el.value()).unwrap_or_default();
```

✅ Reset via runtime — `close()` limpa inputs dentro do dialog automaticamente.

---

## Bootstrap Engine (CR-401)

Portal move durante hydrate, antes do loader JS existir. Solução: Stabilizer com 3 RAF frames garante DOM estável antes do init:

```javascript
stabilize() {
  return new Promise(resolve => {
    let frames = 0;
    const tick = () => { if (++frames > 3) { resolve(); return; } requestAnimationFrame(tick); };
    requestAnimationFrame(tick);
  });
}
```

---

## DOM References (CR-403)

Runtime nunca captura `Element` em closure — Leptos pode recriar o elemento. Sempre buscar pelo uid no momento da execução:
```rust
let Some(root_live) = doc.query_selector(&format!("[data-rs-dialog][data-rs-uid='{}']", uid)).ok().flatten()
else { return };
```

---

## O que NÃO funciona

- `version == 0` como guard SSR — SSR já incrementa version
- `spawn_local` dentro do Action — instável
- `stop_propagation` no botão submit — workaround, não resolve runtime
- `is_connected` guard em closures — quebra após re-render
- Input controlado via signal em arquitetura DOM-driven
