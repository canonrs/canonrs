# 18 — Hydration, UID e Contrato de Primitives

## Causas dos Hydration Errors

### 1. Button dentro de Button
`ToolbarItem` era `<button>` contendo `<ButtonPrimitive>` (`<button>`). HTML inválido — o browser separa os elementos, gerando estruturas diferentes entre SSR e WASM.

**Fix:** `ToolbarItem` mudado para `<div>`.

**Regra:** `<button>` como container de elementos interativos é proibido.

### 2. UID não determinístico
`AtomicU64` global não reseta entre requests SSR. No WASM recomeça do zero. A ordem de renderização pode diferir, gerando UIDs diferentes para os mesmos elementos.

**Fix:** `UidContext` via `provide_context` do Leptos — um contador por árvore, provido pelo `CanonRSRoot`.

**Regra:** UID nunca pode ser gerado por contador global. Usar `UidContext`.

### 3. UID em layouts passivos
Stack, Flex, Grid, Container e server layouts tinham `data-rs-uid` desnecessário. Layouts sem interaction não precisam de UID — só incrementavam o contador.

**Fix:** Removido `generate()` e `data-rs-uid` de todos os layouts passivos.

**Regra:** `data-rs-uid` apenas em primitives com `data-rs-interaction`.

---

## UiStateAttrs — Value Object

Lógica de projeção de estado não pertence ao primitive (viola contrato) nem ao UI (verboso). Pertence ao `state_engine` como value object:

```rust
// state_engine.rs
pub struct UiStateAttrs {
    pub data_rs_state:    Option<&'static str>,
    pub data_rs_disabled: Option<&'static str>,
    pub disabled:         bool,
    pub aria_disabled:    Option<&'static str>,
    pub aria_busy:        Option<&'static str>,
    pub aria_pressed:     Option<&'static str>,
}
```

O primitive recebe o value object pronto — zero lógica:

```rust
let attrs = UiStateAttrs::from_button(disabled, loading, pressed);
view! { <button data-rs-state=attrs.data_rs_state disabled=attrs.disabled .../> }
```

| Abordagem | Nível |
|---|---|
| Strings avulsas via props | Junior |
| Lógica no primitive | Médio |
| Value Object UiStateAttrs | Tier S |

---

## Contrato de Primitives — Novas Regras

### ✔ PERMITIDO
- `UiStateAttrs` como prop única
- `data-rs-uid` fixo via prop quando caller referencia externamente

### ❌ PROIBIDO
- `<button>` como container de interativos
- Lógica condicional inline no `view!`
- `data-rs-uid` em layouts passivos
- Branching que produz elementos HTML diferentes
- `AtomicU32/U64` próprio — usar `generate()`
- Concatenação manual de state tokens

---

## Responsabilidade por Camada

| Camada | Responsabilidade |
|---|---|
| Primitive | Projeta `UiStateAttrs` — zero lógica |
| UI | Proxy puro |
| Boundary | Normaliza props (`bool → State`) |
| state_engine | Calcula `UiStateAttrs` |
| Runtime | Gerencia tokens em `data-rs-state` no cliente |

---

## Debug

```javascript
// comparar UIDs SSR vs WASM
[...document.querySelectorAll('[data-rs-uid]')].map(el => el.getAttribute('data-rs-uid'))
```

```bash
curl -s http://localhost:3000/page | grep -o 'data-rs-uid="[^"]*"'
```
