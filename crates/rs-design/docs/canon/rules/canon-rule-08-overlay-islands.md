# Canon Rule #8: Overlay Islands (Client-Only Architecture)

**Status:** Normative  
**Applies to:** DropdownMenu, Popover, ContextMenu, CommandPalette, Dialog (quando dinâmicos)

---

## The Problem

**Overlays dinâmicos** (com listas reativas) + **SSR** = **Hydration Hell**

### Why?

1. SSR gera HTML estático
2. Client tenta hidratar
3. Lista dinâmica altera DOM
4. Leptos detecta mismatch
5. **Panic**

### Attempted Solutions (All Failed)

| Approach | Why It Failed |
|----------|--------------|
| `cfg!(feature = "ssr")` | Compile-time, não runtime |
| `#[cfg(feature = "hydrate")]` | Código não existe no SSR, quebra composição |
| `is_server()` | Edge case: pode retornar `true` no client em alguns builds |
| `.map().collect_view()` | `Vec<View>` não é `Sync` |
| `StoredValue<Vec<View>>` | Mesmo problema, não compila |
| `<For>` dentro de overlay | Hydration mismatch direto |

---

## The Correct Solution: Browser Detection

### Pattern
```rust
use leptos::prelude::*;

#[component]
pub fn DynamicOverlay(items: Vec<String>) -> impl IntoView {
    let items = StoredValue::new(items);
    let is_browser = RwSignal::new(false);
    
    // Runtime browser detection
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        if web_sys::window().is_some() {
            is_browser.set(true);
        }
    });

    view! {
        <Show
            when=move || is_browser.get()
            fallback=|| view! {
                // SSR placeholder (same size as real component)
                <div class="w-20 h-8"></div>
            }
        >
            <DropdownMenu>
                <DropdownMenuContent>
                    {move || {
                        items.get_value()
                            .iter()
                            .map(|item| view! { <MenuItem>{item}</MenuItem> })
                            .collect_view()
                    }}
                </DropdownMenuContent>
            </DropdownMenu>
        </Show>
    }
}
```

### Why This Works

1. **SSR:** `window` não existe → `is_browser = false` → renderiza placeholder
2. **Client:** `window` existe → Effect roda → `is_browser = true` → renderiza overlay
3. **Zero hydration mismatch:** Placeholder no SSR, overlay no client
4. **Runtime puro:** Não depende de feature flags
5. **Robust:** Funciona em todos os modos de build (dev/watch/release/SSR)

---

## Affected Components

### Must Use Island Pattern

- ✅ `DropdownMenu` (com lista dinâmica)
- ✅ `Popover` (com conteúdo dinâmico)
- ✅ `ContextMenu` (com itens dinâmicos)
- ✅ `CommandPalette` (sempre dinâmico)
- ✅ `Select` (com opções dinâmicas)
- ✅ `DataTableViewOptions` (column toggle)

### Can Use SSR Normally

- ✅ `Table` (dados estáticos)
- ✅ `Card` (layout fixo)
- ✅ `Button` (sem overlay)
- ✅ `Input` (formulário)
- ✅ `Checkbox` (controle simples)

---

## Implementation Checklist

- [ ] Component usa overlay (Dropdown/Popover/Dialog)?
- [ ] Overlay contém lista dinâmica (`<For>` ou `.map()`)?
- [ ] Se SIM aos dois: aplicar Island Pattern
- [ ] Placeholder SSR tem **mesmo tamanho** do componente real
- [ ] Testado em SSR + hydration + client

---

## Edge Cases

### "is_server() retorna true no client"

**Sintoma:** `is_server()` continua `true` mesmo após hydration

**Causa:** Bug conhecido do Leptos em alguns setups de build

**Solução:** Usar `web_sys::window()` ao invés de `is_server()`

### "Flash of placeholder"

**Sintoma:** Usuário vê placeholder antes do overlay aparecer

**Causa:** Normal - Effect precisa rodar primeiro

**Solução:** 
- Placeholder deve ter tamanho/estilo similar
- Adicionar `opacity-0` no placeholder se necessário
- Ou aceitar como trade-off de SSR correto

---

## Performance Implications

### SSR

- ✅ Payload menor (sem overlay no HTML)
- ✅ First Paint mais rápido
- ❌ Componente não visível em SSR

### Client

- ✅ Zero hydration errors
- ✅ Overlay funcional 100%
- ⚠️ Leve delay até Effect rodar (imperceptível)

---

## Comparison: CanonRS vs shadcn

| Aspect | shadcn (React) | CanonRS (Leptos) |
|--------|----------------|------------------|
| **Estratégia** | Client-only implícito | Island explícito |
| **SSR** | Não real (Next.js client components) | SSR verdadeiro |
| **Governança** | Não documentada | Regra canônica |
| **Robustez** | "Just works" | Engenharia consciente |

**Veredito:** CanonRS é **mais rigoroso**, não inferior.

---

## Examples

### ✅ Correto: DataTableViewOptions
```rust
// SSR: placeholder
// Client: dropdown com checkboxes dinâmicos
<Show when=move || is_browser.get() ...>
    <DropdownMenu>
        {move || columns.iter().map(...).collect_view()}
    </DropdownMenu>
</Show>
```

### ❌ Errado: Lista dinâmica sem Island
```rust
// QUEBRA hydration
<DropdownMenu>
    <For each=|| items .../>
</DropdownMenu>
```

### ❌ Errado: cfg!() para runtime
```rust
// NÃO funciona
<Show when=|| !cfg!(feature = "ssr") ...>
```

---

## Normative Status

- Violations **MUST** block PRs
- Exceptions require explicit approval
- All overlay components **MUST** document if they need Islands
- Design system **MUST** provide Island utilities

---

**Author:** Canon Working Group  
**Date:** 2025-12-28  
**Version:** 1.0  
**Replaces:** None (primeira definição formal)
