---
component: Tabs
layer: UI
status: Stable
since: v0.1
last_review: 2026-01-15
ownership: canonrs
keywords:
  - design system
  - leptos
  - ssr
  - tabs
  - navigation
path primitive: /opt/docker/monorepo/packages-rust/rs-design/src/primitives/tabs.rs
path UI: /opt/docker/monorepo/packages-rust/rs-design/src/ui/tabs/tabs.rs
---

# Tabs UI Component

## 1. Introdução Conceitual

O componente **Tabs** organiza conteúdo relacionado em painéis navegáveis através de gatilhos (triggers). É uma camada de ergonomia sobre os primitives de tabs, fornecendo uma API simples e semântica para construir interfaces tabuladas acessíveis.

**Por que existe no CanonRS?**  
Tabs são um padrão fundamental de navegação/organização de conteúdo. O UI component reduz boilerplate, encapsula composição correta dos primitives e garante consistência semântica.

**Camada**: UI (compõe primitives, expõe API amigável)

**O que NÃO faz:**
- Não gerencia estado global de tabs
- Não aplica animações (CSS cuida disso)
- Não acessa browser APIs
- Não executa efeitos colaterais

---

## 2. Responsabilidade Arquitetural (Contrato)

### Responsabilidade

- Compor `TabsPrimitive`, `TabsListPrimitive`, `TabsTriggerPrimitive`, `TabsContentPrimitive`
- Aceitar `RwSignal<String>` como estado reativo da tab ativa
- Fornecer prop `class` opcional para customização no `TabsList`
- Passar props essenciais (`value`, `children`) aos primitives
- Renderizar HTML semântico via primitives

### Não Responsabilidade

- **NÃO** gerencia ciclo de vida de estado (estado vem do pai)
- **NÃO** aplica estilos inline ou classes ad-hoc
- **NÃO** usa `window()`, `document()`, `web_sys`
- **NÃO** registra event listeners globais
- **NÃO** executa lógica de negócio

---

## 3. Posição no Ecossistema CanonRS
```text
User Code (página/block)
  ↓
Tabs UI (ergonomia, composição)
  ↓
TabsPrimitive, TabsListPrimitive, TabsTriggerPrimitive, TabsContentPrimitive
  ↓
HTML com data-attributes (data-tabs, data-tabs-trigger, data-state)
  ↓
CSS tokens (style/ui/tabs.css)
  ↓
Browser (rendering + interatividade via Leptos reactivity)
```

**SSR**: Tabs renderiza HTML completo com estado inicial (`data-state="active"/"inactive"`)  
**Hydration**: Leptos hidrata event handlers (`on:click`) e reatividade (`active_value.get()`)  
**Runtime**: Não há runtime JS adicional — interatividade é nativa do Leptos

---

## 4. Tokens Aplicados (OBRIGATÓRIO)

### Layout
- `--space-md` (gap entre tabs container e content)
- `--space-xs` (gap entre triggers)
- `--space-sm`, `--space-md` (padding interno dos triggers)

### Tipografia
- `--font-family-sans`
- `--font-size-sm`
- `--font-weight-medium`

### Cor
- `--color-fg-muted` (trigger inativo)
- `--color-fg-default` (trigger hover)
- `--color-primary-fg` (trigger ativo - texto)
- `--color-primary-border` (trigger ativo - borda inferior)
- `--color-bg-muted` (trigger hover - background)
- `--color-border-default` (borda inferior da TabsList)

### Border
- `--border-width-hairline`
- `--nav-indicator-thickness` (borda ativa do trigger)

### State
- `--state-focus-ring`

### Motion
- `--motion-duration-fast`
- `--motion-ease-standard`

---

## 5. Estrutura Técnica (Como Funciona)

**SSR Render:**
```html
<div data-tabs="">
  <div data-tabs-list="" role="tablist">
    <button data-tabs-trigger="" data-state="active" role="tab" aria-selected="true">Tab 1</button>
    <button data-tabs-trigger="" data-state="inactive" role="tab" aria-selected="false">Tab 2</button>
  </div>
  <div data-tabs-content="" data-state="active" role="tabpanel">Content 1</div>
  <div data-tabs-content="" data-state="inactive" role="tabpanel">Content 2</div>
</div>
```

**Hydration:**  
Leptos hidrata:
- `on:click` nos triggers (atualiza `RwSignal`)
- Reatividade nos `data-state` (muda via `active_value.get()`)
- `aria-selected` dinâmico

**Contrato esperado:**  
CSS aplica `display: none` em `[data-tabs-content][data-state="inactive"]`

---

## 6. Fluxo de Execução
```text
SSR Render (servidor)
 → HTML estático com data-state inicial
 → Leptos hydration (cliente)
 → Event handlers ativos (on:click)
 → RwSignal.set() atualiza estado
 → Reatividade atualiza data-state
 → CSS mostra/oculta content
```

**Sem runtime JS adicional** — interatividade é Leptos puro.

---

## 7. Casos de Uso Canônicos

### Documentação de API
```rust
<Tabs value=active_tab>
  <TabsList>
    <TabsTrigger value="overview">"Overview"</TabsTrigger>
    <TabsTrigger value="examples">"Examples"</TabsTrigger>
  </TabsList>
  <TabsContent value="overview">{/* docs */}</TabsContent>
  <TabsContent value="examples">{/* code */}</TabsContent>
</Tabs>
```

### Configurações/Settings
```rust
<Tabs value=section>
  <TabsList class="settings-tabs">
    <TabsTrigger value="account">"Account"</TabsTrigger>
    <TabsTrigger value="privacy">"Privacy"</TabsTrigger>
  </TabsList>
  <TabsContent value="account"><AccountForm /></TabsContent>
  <TabsContent value="privacy"><PrivacySettings /></TabsContent>
</Tabs>
```

### Code Examples (Primitive vs UI)
```rust
<Tabs value=code_type>
  <TabsList>
    <TabsTrigger value="primitive">"Primitive"</TabsTrigger>
    <TabsTrigger value="ui">"UI Layer"</TabsTrigger>
  </TabsList>
  <TabsContent value="primitive"><CodeBlock>{primitive_code}</CodeBlock></TabsContent>
  <TabsContent value="ui"><CodeBlock>{ui_code}</CodeBlock></TabsContent>
</Tabs>
```

---

## 8. Anti-Patterns (PROIBIDOS)

### ❌ Gerenciar estado interno
```rust
// ERRADO - UI não deve criar estado próprio
#[component]
pub fn Tabs(children: Children) -> impl IntoView {
    let value = RwSignal::new("default".to_string()); // ❌
    // ...
}
```
**Por quê?** Estado deve vir do pai para permitir controle externo.

### ❌ Aplicar classes inline nos primitives
```rust
// ERRADO
<TabsTriggerPrimitive class="custom-class"> // ❌
```
**Por quê?** Primitives usam data-attributes, não classes. Wrapping com `<div class=...>` é correto.

### ❌ Lógica de negócio no UI
```rust
// ERRADO
on:click=move |_| {
    // validação, API call, etc ❌
    active_value.set(value.clone());
}
```
**Por quê?** UI apenas compõe. Lógica vai no bloco/página.

### ❌ Usar browser APIs
```rust
// ERRADO
use web_sys::window; // ❌
```
**Por quê?** Quebra SSR. Browser APIs vão no shell runtime.

---

## 9. SSR, Hydration e Runtime

### SSR
- Renderiza HTML completo com `data-state` inicial
- `aria-selected` e `role` corretos desde o servidor
- Conteúdo inativo renderizado (mas oculto via CSS)

### Hydration
- Leptos hidrata event handlers (`on:click`)
- Reatividade (`active_value.get()`) funciona imediatamente
- Sem flash de conteúdo incorreto (estado SSR = estado inicial)

### Runtime
- **Sem runtime JS adicional**
- Interatividade via Leptos reactivity nativo
- CSS puro para transições/animações

### AutoReload / Hot-reload
- Funciona normalmente (sem runtime global para quebrar)
- Estado reativo sobrevive ao hot-reload

---

## 10. Checklist de Conformidade

- [x] SSR-safe (renderiza HTML completo)
- [x] Sem JS imperativo (usa Leptos reactivity)
- [x] Usa tokens canônicos
- [x] Tokens listados na seção 4
- [x] Anti-patterns documentados
- [x] Canon Rules citadas

---

## 11. Canon Rules Aplicadas

### Canon Rules Applied

- **Canon Rule #102** — Runtime JS Is Shell Infrastructure  
  *Tabs não inclui runtime JS — interatividade é Leptos puro*

- **Canon Rule #103** — Critical Runtime JS Must Be Inline in SSR  
  *Não há runtime crítico (sem scripts)*

- **Canon Rule #104** — AutoReload Breaks Script Order Guarantees  
  *Sem scripts globais para quebrar*

- **Canon Rule #105** — Primitives Render Data-Attributes, Not Classes  
  *Tabs UI compõe primitives corretamente, sem forçar classes neles*

- **Canon Rule #106** — UI Components Compose Primitives, Never Raw HTML  
  *Tabs UI usa apenas TabsPrimitive, TabsListPrimitive, etc*

- **Canon Rule #107** — State Management Lives Outside UI Components  
  *`value: RwSignal<String>` vem do pai, UI não cria estado próprio*
