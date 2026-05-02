# CanonRS Prelude — Instructions

## Como importar

Todo arquivo que usa componentes CanonRS deve ter exatamente estes três imports:

```rust
use leptos::prelude::*;
use canonrs::prelude::*;
use canonrs::slot;
```

O `canonrs::slot` é necessário explicitamente por conflito com o atributo `#[slot]` do Leptos.

---

## O que está disponível via `canonrs::prelude::*`

### UI Components
Todos os componentes de `src/ui/` via seus `_boundary.rs`:
`Button`, `Input`, `Badge`, `Banner`, `Toast`, `Spinner`, `EmptyState`, `Toolbar`,
`Tabs`, `Select`, `Dialog`, `Tooltip`, `NavItem`, `NavGroup` e todos os demais.

### Blocks
Componentes compostos de `src/blocks/`:
`PageHeader`, `CardBlock`, `Hero`, `SidebarLayout`, `SectionBlock`,
`FormFieldBlock`, `StatGroupBlock`, `DataTableBlock`.

### Layouts
`DashboardLayout`, `ThreePaneLayout`, `MarketingLayout`, `SplitViewLayout`,
`FullscreenLayout`, `WizardLayout`, `PageLayout`.

### Primitivas
Todos os tipos de `canonrs_core::primitives::*`:
`ButtonVariant`, `ButtonSize`, `StackGap`, `GridCols`, `FlexJustify` e demais enums.

---

## Como usar slots

Slots são closures que retornam `AnyView`. Sempre terminam com `.into_any()`:

```rust
<PageHeader
    title=slot!(|| view! { "Titulo" }.into_any())
    actions=slot!(|| view! {
        <Button>"Acao"</Button>
    }.into_any())
/>
```

Para slots com captura de escopo use `move`:

```rust
slot!(move || view! { <Button>{label.get()}</Button> }.into_any())
```

---

## Como o prelude é gerado

O `ui::prelude` é gerado automaticamente pelo `build.rs` a partir dos `*_boundary.rs`
de cada componente em `src/ui/`. Nao edite manualmente.

O `blocks::prelude` é mantido manualmente em `src/blocks/prelude.rs`.

Ao adicionar um novo componente em `src/ui/`, basta criar o `*_boundary.rs`
e o `build.rs` o inclui automaticamente no proximo build.

---

## Regra de ouro

Nunca importe de `canonrs_server::ui::` diretamente nos apps.
Sempre use `canonrs::prelude::*` — o facade é o unico ponto de entrada.
