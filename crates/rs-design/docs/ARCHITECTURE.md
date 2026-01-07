# Arquitetura do Design System - 6 Camadas

## Filosofia

**tokens → primitives → ui → blocks → layouts → pages**

Cada camada depende apenas das anteriores. Separação rigorosa: Design ≠ Lógica ≠ Negócio.

## Estrutura
```
packages-rust/rs-design/
├── tokens/      → CSS variables (source of truth)
├── primitives/  → HTML semântico + acessibilidade
├── ui/          → Componentes estilizados genéricos
├── blocks/      → Composições reutilizáveis
└── layouts/     → Shells de página (Header, Sidebar)

apps/*/src/pages/ → Páginas com lógica de negócio
```

## 1. Tokens (Base Absoluta)

CSS variables como single source of truth:
```css
/* style/tokens.css */
@theme inline {
  --color-primary: 38 92% 50%;      /* HSL puro */
  --color-background: 0 0% 100%;
  --space-md: 1rem;
  --radius-md: 0.375rem;
  --size-control-md: 2.5rem;
}

:root {
  --color-background: 0 0% 100%;
}

.dark {
  --color-background: 0 0% 9%;
}

[data-density="compact"] {
  --size-control-md: 2rem;
}
```

**Bindings Rust**: Existem apenas para tipagem, nunca como source of truth.

**Regra**: Tokens vivem em CSS. Rust consome, não define.

## 2. Primitives (HTML Semântico)

HTML semântico + acessibilidade + data-attributes. Sem tokens, sem Tailwind, sem classes visuais:
```rust
// primitives/button.rs
#[component]
pub fn ButtonPrimitive(
    #[prop(optional)] variant: Option<ButtonVariant>,
    #[prop(optional)] class: String,
    children: Children,
) -> impl IntoView {
    view! { 
        <button 
            data-variant=variant.unwrap_or(ButtonVariant::Solid)
            data-size="md"
            class=class
        >
            {children()}
        </button> 
    }
}
```

**Importante**: `data-variant`, `data-size` são contrato estrutural, não estilo visual.

**Regra**: Zero lógica de negócio, zero tokens CSS inline.

## 3. UI Components (Estilizados)

Combinam Primitives + Tokens (via Tailwind):
```rust
// ui/button.rs
use crate::primitives::ButtonPrimitive;

#[component]
pub fn Button(
    #[prop(default = ButtonVariant::Solid)] variant: ButtonVariant,
    children: Children,
) -> impl IntoView {
    view! {
        <ButtonPrimitive 
            variant=variant
            class="h-[var(--size-control-md)] px-[var(--space-control-x)] bg-gradient-to-r from-[hsl(var(--color-primary))] to-[hsl(var(--color-accent-foreground))]"
        >
            {children()}
        </ButtonPrimitive>
    }
}
```

**Componentes**: Button, Input, Card, Label, Checkbox, Switch, Dropdown, Sidebar, Charts, Animate.

**Regra**: Stateless, sem lógica de negócio.

## 4. Blocks (Composições)

Combinam múltiplos UI components:
```rust
// blocks/forms/auth/login_form_04.rs
use crate::ui::{Card, Button, Input, Label};

#[component]
pub fn LoginForm04(
    on_submit: Callback<(String, String)>,
    is_loading: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <Card>
            <form on:submit=handle_submit>
                <Input placeholder="Email"/>
                <Input type="password" placeholder="Password"/>
                <Button disabled=is_loading.get()>"Login"</Button>
            </form>
        </Card>
    }
}
```

**Organização**: `blocks/navigation/`, `blocks/user/`, `blocks/charts/`, `blocks/forms/auth/`.

**Regra**: Sem rotas, sem auth logic. Apenas composição visual + validação.

## 5. Layouts (Shells de Página)

Estruturas que organizam Blocks:
```rust
// layouts/login_layout.rs
#[component]
pub fn LoginLayout(children: Children) -> impl IntoView {
    view! {
        <div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-[hsl(var(--color-background))] to-[hsl(var(--color-muted))]">
            <div class="absolute top-4 right-4">
                <LanguageToggle/>
            </div>
            {children()}
        </div>
    }
}
```

**Layouts disponíveis**: `LoginLayout`, `DashboardLayout`.

**Regra**: Apenas estrutura visual. NÃO contém providers globais (vão no App).

## 6. Providers (Contexto Global)

State management global:
```rust
// layouts/theme_provider.rs
#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    view! {
        <div data-name="ThemeProvider">
            {children()}
        </div>
    }
}
```

**Providers disponíveis**: `ThemeProvider`, `LanguageProvider`.

**Regra**: Providos APENAS no `App()`, nunca em layouts.

## 7. Pages (Lógica de Negócio)

Páginas específicas que combinam tudo:
```rust
// apps/core-auth/src/pages/auth/login.rs
use rs_design::layouts::LoginLayout;
use rs_design::blocks::forms::auth::LoginForm04;

#[component]
pub fn LoginPage() -> impl IntoView {
    let auth = use_auth();
    let is_loading = RwSignal::new(false);
    
    let on_submit = Callback::new(move |(email, password)| {
        is_loading.set(true);
        auth.login(email, password).await;
        navigate("/dashboard");
    });

    view! {
        <LoginLayout>
            <LoginForm04 on_submit=on_submit is_loading=is_loading/>
        </LoginLayout>
    }
}
```

**Localização**: `apps/*/src/pages/` (fora do design system).

**Regra**: Única camada com lógica de negócio, auth, navegação, API calls.

## App Setup (Providers Globais)
```rust
// app.rs
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_auth();

    view! {
        <Router>
            <LanguageProvider initial_language="pt">
                <ThemeProvider>
                    <main>
                        <Routes>
                            <Route path="/login" view=LoginPage/>
                        </Routes>
                    </main>
                </ThemeProvider>
            </LanguageProvider>
        </Router>
    }
}
```

**Regra**: Providers APENAS no `App()`, nunca em layouts ou blocks.

## Regras de Ouro

❌ **NUNCA**: Tokens em Rust como source of truth  
❌ **NUNCA**: Primitives com classes visuais  
❌ **NUNCA**: UI components com lógica de negócio  
❌ **NUNCA**: Blocks com rotas ou auth  
❌ **NUNCA**: Layouts com providers globais  
❌ **NUNCA**: Providers em camadas inferiores  
✅ **SEMPRE**: Tokens em CSS variables  
✅ **SEMPRE**: Pages com lógica completa  
✅ **SEMPRE**: Providers no App()

## Quando Criar

**Token**: Nova cor/spacing/valor reutilizável (CSS variable)  
**Primitive**: Novo elemento HTML base (raro)  
**UI Component**: Dropdown, Modal, Toast (genéricos)  
**Block**: ProfileCard, CommentBox (composições)  
**Layout**: BlogLayout, CheckoutLayout (shells)  
**Provider**: AuthProvider, CartProvider (state global)  
**Page**: Sempre (aplicação real)

## Resultado

✅ Componentes 100% reutilizáveis  
✅ Mudanças centralizadas em CSS  
✅ Separação de responsabilidades  
✅ Manutenção e testes isolados  
✅ Escalabilidade enterprise
