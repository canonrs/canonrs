---
component: CopyButton
layer: UI
status: Stable
since: v0.8
last_review: 2026-01-15
ownership: canonrs
keywords:
  - design system
  - leptos
  - ssr
  - clipboard
  - copy
  - data attributes
  - event delegation
path_primitive: packages-rust/rs-design/src/primitives/copy_button.rs
path_ui: packages-rust/rs-design/src/ui/copy_button/copy_button.rs
---

# CopyButton

## 1. Introdução Conceitual

O **CopyButton** é um componente UI que declara a intenção de copiar texto para o clipboard do usuário. Ele não executa a cópia diretamente — em vez disso, renderiza atributos `data-*` que são interpretados pelo runtime JavaScript do shell.

Este componente resolve o problema de **separação de responsabilidades** em ambientes SSR:
- **Rust/Leptos:** declara o que deve acontecer
- **Shell Runtime:** executa o efeito no browser

O CopyButton existe na camada **UI** porque fornece uma interface ergonômica sobre o `CopyButtonPrimitive`, adicionando texto padrão ("Copy") e semântica de alto nível.

**O que ele NÃO faz:**
- Não chama `navigator.clipboard` ou `document.execCommand`
- Não manipula DOM diretamente
- Não gerencia estado de "copiado" (isso é responsabilidade do runtime)
- Não lida com permissões de clipboard

---

## 2. Responsabilidade Arquitetural (Contrato)

### Responsabilidade

- Renderizar um `<button>` com atributos `data-copy-button` e `data-copy-text`
- Aceitar `text: String` (conteúdo a ser copiado)
- Aceitar `disabled: bool` (estado do botão)
- Fornecer children padrão ("Copy")
- Delegar para `CopyButtonPrimitive`

### Não Responsabilidade

- **NÃO** executa clipboard API
- **NÃO** gerencia feedback visual além do que o runtime fornece
- **NÃO** valida conteúdo do texto
- **NÃO** lida com fallbacks para browsers antigos
- **NÃO** persiste estado entre renderizações

---

## 3. Posição no Ecossistema CanonRS
```text
User Click
 ↓
CopyButton (UI Layer)
 ↓
CopyButtonPrimitive (Primitive Layer)
 ↓
HTML: <button data-copy-button data-copy-text="...">
 ↓
Shell Runtime JS (inline no <head>)
 ↓
Event Delegation (document.addEventListener)
 ↓
navigator.clipboard.writeText() ou execCommand('copy')
 ↓
Browser Clipboard API
```

**Contexto SSR:**
- SSR renderiza HTML estático com `data-*` attributes
- Hydration NÃO adiciona event listeners (eles já existem no runtime)
- Runtime JS executa ANTES do hydration (via inline script)

---

## 4. Tokens Aplicados

### Nenhum token visual aplicado diretamente

O `CopyButton` **não aplica tokens** porque:
1. Ele delega para `CopyButtonPrimitive` que renderiza HTML puro
2. Estilos são aplicados via CSS externo ou composição com outros componentes
3. O foco é no **contrato de dados**, não na apresentação

**Se precisar estilizar:**
- Compor com `Button` do rs-design
- Aplicar classes Tailwind via prop `class`
- Usar variant system do design system

---

## 5. Estrutura Técnica (Como Funciona)

### Código do Componente UI
```rust
use leptos::prelude::*;
use crate::primitives::copy_button::CopyButtonPrimitive;

#[component]
pub fn CopyButton(
    text: String,
    #[prop(optional)] disabled: bool,
) -> impl IntoView {
    view! {
        <CopyButtonPrimitive copy_text=text>
            "Copy"
        </CopyButtonPrimitive>
    }
}
```

### Código do Primitive
```rust
#[component]
pub fn CopyButtonPrimitive(
    children: Children,
    copy_text: String,
) -> impl IntoView {
    view! {
        <button
            data-copy-button=""
            data-copy-text=copy_text
        >
            {children()}
        </button>
    }
}
```

### HTML Gerado (SSR)
```html
<button data-copy-button="" data-copy-text="Hello World">
  Copy
</button>
```

### Runtime JS (Shell)
```javascript
document.addEventListener("click", (e) => {
  const btn = e.target.closest("[data-copy-button]");
  if (!btn) return;
  
  const text = btn.getAttribute("data-copy-text");
  if (!text) return;
  
  // Fallback: execCommand para compatibilidade
  const textarea = document.createElement("textarea");
  textarea.value = text;
  textarea.style.position = "fixed";
  textarea.style.opacity = "0";
  document.body.appendChild(textarea);
  textarea.select();
  document.execCommand("copy");
  document.body.removeChild(textarea);
  
  // Feedback visual
  const prev = btn.textContent;
  btn.textContent = "Copied!";
  setTimeout(() => (btn.textContent = prev), 1500);
}, true);
```

---

## 6. Fluxo de Execução
```text
1. SSR Render (Rust/Leptos)
   └→ Gera HTML: <button data-copy-button data-copy-text="...">

2. Shell Inline Runtime (Antes de Hydration)
   └→ Registra event listener em document
   └→ Usa event delegation para capturar clicks

3. User Click
   └→ Browser dispara evento click
   └→ Runtime detecta [data-copy-button]
   └→ Lê data-copy-text
   └→ Executa clipboard API
   └→ Atualiza textContent para "Copied!"

4. Hydration (Após Runtime)
   └→ Leptos hidrata componentes
   └→ NÃO sobrescreve listeners (runtime usa capture phase)
```

**Ordem crítica:**
- Runtime inline **antes** de `<HydrationScripts/>`
- Event listener registrado **antes** de qualquer hydration
- Uso de `capture: true` garante precedência sobre Leptos

---

## 7. Casos de Uso Canônicos

### Uso Básico
```rust
use rs_design::ui::CopyButton;

view! {
    <CopyButton text="npm install canonrs".to_string()/>
}
```

### Com Conteúdo Customizado
```rust
view! {
    <CopyButtonPrimitive copy_text="const x = 42;".to_string()>
        <Icon name="clipboard"/>
        "Copy Code"
    </CopyButtonPrimitive>
}
```

### Em Code Block
```rust
view! {
    <div class="relative">
        <pre><code>{code_content}</code></pre>
        <CopyButton 
            text=code_content.clone()
            class="absolute top-2 right-2"
        />
    </div>
}
```

### Com Disabled State
```rust
let (loading, set_loading) = create_signal(false);

view! {
    <CopyButton 
        text="data".to_string()
        disabled=loading.get()
    />
}
```

---

## 8. Anti-Patterns (PROIBIDOS)

### ❌ Chamar Clipboard API no Componente
```rust
// NUNCA FAÇA ISSO
#[component]
pub fn BadCopyButton(text: String) -> impl IntoView {
    let on_click = move |_| {
        // ❌ VIOLA Canon Rule #102
        window().navigator().clipboard().write_text(&text);
    };
    
    view! {
        <button on:click=on_click>"Copy"</button>
    }
}
```

**Por que é proibido:**
- Componentes Rust não devem chamar browser APIs
- Runtime JS é infra do shell, não lógica de componente
- Quebra SSR (window/navigator não existem no servidor)

---

### ❌ Gerenciar Estado de Feedback no Componente
```rust
// EVITE ISSO
#[component]
pub fn BadCopyButton(text: String) -> impl IntoView {
    let (copied, set_copied) = create_signal(false);
    
    view! {
        <button data-copy-button data-copy-text=text>
            {move || if copied.get() { "Copied!" } else { "Copy" }}
        </button>
    }
}
```

**Por que evitar:**
- Runtime JS já gerencia feedback (`btn.textContent = "Copied!"`)
- Duplicar estado causa dessincronização
- Aumenta complexidade sem benefício

**Exceção:** Se precisar de animações complexas ou acessibilidade avançada.

---

### ❌ Usar Script Externo em Vez de Inline
```rust
// NÃO FAÇA
view! {
    <head>
        <script src="/copy-runtime.js"></script>
        <HydrationScripts/>
    </head>
}
```

**Por que é proibido:**
- AutoReload pode reordenar scripts (Canon Rule #104)
- Runtime pode carregar DEPOIS do hydration
- Race conditions silenciosas

**Correto:** Runtime inline no shell (Canon Rule #103)

---

## 9. SSR, Hydration e Runtime

### Impacto do SSR

- **HTML é gerado no servidor** com `data-*` attributes
- **Nenhum JavaScript é necessário para SSR** — o HTML é válido e acessível
- **Sem hydration mismatch** porque não há estado visual gerenciado

### Ordem de Execução
```text
Server:
  1. Leptos renderiza CopyButton
  2. Gera <button data-copy-button ...>

Browser (Primeira Carga):
  1. HTML chega
  2. Inline runtime script executa (antes de hydration)
  3. Event listener registrado
  4. HydrationScripts carrega
  5. Leptos hidrata componentes
  6. Componente hidratado, listener já ativo

Browser (Hot Reload):
  1. AutoReload injeta scripts dinamicamente
  2. Runtime inline AINDA executa primeiro (imune a reordenação)
  3. Listeners permanecem ativos
```

### AutoReload / Hot-Reload

**Problema histórico (resolvido):**
- Antes: `<script src="/runtime.js">` era reordenado por AutoReload
- Scripts externos carregavam DEPOIS do hydration
- Listeners registravam tarde demais

**Solução atual:**
- Runtime inline no `<head>` do shell
- Executa antes de qualquer dinâmica
- Imune a AutoReload (Canon Rule #104)

### Restrições de Runtime Global

- Runtime JS **deve** usar event delegation
- Runtime JS **não pode** depender de ordem de scripts externos
- Runtime JS **deve** estar inline no shell
- Runtime JS **não pode** ser carregado via import dinâmico

---

## 10. Checklist de Conformidade
```md
- [x] SSR-safe (não usa window/navigator)
- [x] Sem JS imperativo (apenas data-* attributes)
- [ ] Usa tokens (N/A — componente não estilizado)
- [x] Tokens listados (N/A)
- [x] Anti-patterns documentados
- [x] Rules citadas
- [x] Fluxo de execução explicado
- [x] Casos de uso canônicos
- [x] Posição no ecossistema
```

---

## 11. Canon Rules Applied

### Canon Rules Applied

- **Canon Rule #102** — Runtime JS Is Shell Infrastructure  
  O CopyButton declara intenção via `data-*`. O shell executa o efeito.

- **Canon Rule #103** — Critical Runtime JS Must Be Inline in SSR  
  O runtime de clipboard está inline no `<head>` do shell, não em arquivo externo.

- **Canon Rule #104** — AutoReload Breaks Script Order Guarantees  
  Runtime inline garante execução antes de hydration, imune a reordenação do AutoReload.

---

## Referências Técnicas

- Leptos SSR: https://book.leptos.dev/ssr/
- Clipboard API: https://developer.mozilla.org/en-US/docs/Web/API/Clipboard_API
- Event Delegation: https://javascript.info/event-delegation
- Canon Rule #102: `/opt/docker/monorepo/libs/canonRS-rules/canon-rule-102-runtime-js-is-shell-infrastructure.md`
- Canon Rule #103: `/opt/docker/monorepo/libs/canonRS-rules/canon-rule-103-critical-runtime-js-must-be-inline-in-ssr.md`

---

## Changelog

### v0.8 (2026-01-15)
- Initial stable release
- Runtime inline implementado
- Documentação completa conforme padrão v1.0

---
