# Template Examples - canonrs-ui-interactive

## REGRA DE OURO
Examples devem demonstrar INTERAÇÃO real (callbacks + estado mutável + resultado visível).

---

## ESTRUTURA MÍNIMA
```rust
use leptos::prelude::*;
use super::ExampleInteractive;

#[component]
pub fn BasicExample() -> impl IntoView {
    let count = RwSignal::new(0);
    
    view! {
        <ExampleInteractive
            on_action=Callback::new(move |_| count.update(|n| *n += 1))
        />
        <p>"Count: " {move || count.get()}</p>
    }
}
```

---

## LEPTOS 0.8 - REGRAS CRÍTICAS

### ✅ CORRETO
```rust
// Signal
let value = RwSignal::new(String::new());

// Children
{children.map(|c| c())}

// Option props
prop=value.unwrap_or_default()

// Conditional rendering
{move || {
    if condition {
        Either::Left(view! { <div>A</div> })
    } else {
        Either::Right(view! { <div>B</div> })
    }
}}

// State em closures (evita FnOnce)
let stored = StoredValue::new(value);
on_click=Callback::new(move |_| stored.get_value())
```

### ❌ ERRADO
```rust
// ❌ signal() não existe
let (val, set) = signal(0);

// ❌ Children direto
{children}

// ❌ Option direto
prop=optional_value

// ❌ .then() em closures
{move || condition.then(|| view! {...})}

// ❌ Move que consome
let id = node.id;
move || handler.run(id) // FnOnce error
```

---

## CHECKLIST

- [ ] Usa `RwSignal::new()` não `signal()`
- [ ] Demonstra callback funcionando
- [ ] Mostra resultado da interação
- [ ] `Children` usa `.map(|c| c())`
- [ ] Props `Option` tem `unwrap_or_default()`
- [ ] Closures que capturam usam `StoredValue` ou clone
- [ ] Condicionais usam `Either::Left/Right`

---

## ERROS COMUNS

**E0282** (type annotations) → Use `Either::Left/Right` em branches  
**E0525** (FnOnce) → Use `StoredValue` ou clone antes do closure  
**E0308** (Option) → Use `.unwrap_or_default()` ou `.map()`  
**IntoRender** → `Children` precisa `.map(|c| c())`

---

**Status**: Template v2.0 com Leptos 0.8  
**Data**: 2026-02-05
