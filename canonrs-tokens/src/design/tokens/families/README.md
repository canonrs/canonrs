# Token Families

## Overview
Families são camadas semânticas que **compõem tokens do core** para criar vocabulários específicos de componentes UI.

## Princípios

### 1. Não-Absolutos
Families referenciam `core.rs` através de `var(--token-name)`:
```rust
FamilyToken::new("button-height", "var(--space-2xl)"),  // ✓ Correto
FamilyToken::new("button-height", "2.5rem"),            // ✗ Evitar
```

**Exceções permitidas:**
- Valores pixel-perfect: `"1px"` (bordas)
- Transparências: `"rgba(0, 0, 0, 0.5)"`
- Dimensões fixas específicas: `"32rem"` (modal-width)

### 2. Nomenclatura Hierárquica
```
{componente}-{propriedade}-{variante}
dialog-overlay-bg
button-ghost-bg-hover
field-error-border-color
```

### 3. Escopo Semântico
Cada family agrupa componentes relacionados:

| Family | Escopo | Componentes |
|--------|--------|-------------|
| **A** - Overlay | Camadas flutuantes | Dialog, Modal, ContextMenu, Popover |
| **B** - Selection | Escolhas e navegação | Tabs, Accordion, Command, Menu |
| **C** - Forms | Entrada de dados | Button, Input, Select, Checkbox, Switch |
| **D** - Navigation | Navegação estrutural | Breadcrumb, Pagination, NavigationMenu |
| **E** - Feedback | Notificações | Toast, Alert, Progress, Skeleton |
| **F** - Data | Visualização de dados | Table, Chart, Badge, Avatar |
| **G** - Composite | Composições complexas | Card, Calendar, Carousel |
| **H** - Layout | Estrutura espacial | Container, Grid, Separator, Spacer |
| **I** - Animation | Movimento e transições | Motion tokens, easing |
| **S** - State | Estados interativos | Hover, Focus, Disabled, Loading |
| **Z** - Layers | Elevação (z-index) | Stacking context |

## Estrutura de Arquivo
```rust
use crate::design::tokens::FamilyToken;

/// FAMILY C — Forms & Inputs
/// Components: Button, Field, Input, Select
/// Scope: Data entry, validation

pub const FAMILY_C_FORMS: &[FamilyToken] = &[
    // Button
    FamilyToken::new("button-height", "var(--space-2xl)"),
    FamilyToken::new("button-padding-x", "var(--space-md)"),
    FamilyToken::new("button-radius", "var(--radius-sm)"),
    
    // Field
    FamilyToken::new("field-height", "var(--space-2xl)"),
    FamilyToken::new("field-border", "1px"),  // Exceção: pixel-perfect
];
```

## Fluxo de Geração
```
core.rs + families/*.rs
         ↓
    cargo build
         ↓
.generated/core.css + .generated/family-*.css
         ↓
    canonrs.css
```

## Adicionando Nova Family

1. Criar `family_x_nome.rs`
2. Registrar em `mod.rs`:
```rust
pub mod family_x_nome;
pub use family_x_nome::FAMILY_X_NOME;
```
3. Gerar CSS: `cargo run --bin token-generator`

## Manutenção

- **Nunca** adicionar valores absolutos sem justificativa
- **Sempre** documentar exceções de valores diretos
- **Verificar** se token core já existe antes de criar novo
- **Agrupar** tokens relacionados com comentários
