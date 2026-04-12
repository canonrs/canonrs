# CanonRS Architecture — Primitive to Layout

## O Fluxo

    Primitive → UI → Boundary → Block → Layout → Page

Cada camada tem uma responsabilidade única. Nenhuma camada conhece as camadas acima dela.

---

## Camadas

### Primitive
HTML puro + ARIA + data-rs-* attributes. Zero lógica. Zero estilo.
Define o contrato semântico do componente — uid SSR, interaction group, state.

    <button
        data-rs-button=""
        data-rs-uid=uid
        data-rs-interaction="init"
        data-rs-state=state
        aria-disabled=disabled
    />

### UI
Mapping 1:1 sobre o primitive. Recebe props tipadas, passa para o primitive.
Sem cálculo. Sem decisão. Sem estado. Estado e defaults nascem fora do UI.

    pub fn ButtonUi(variant: ButtonVariant, size: ButtonSize) -> impl IntoView {
        view! { <ButtonPrimitive variant=variant.as_str() size=size.as_str() /> }
    }

### Boundary
API pública do componente. Pode aplicar defaults tipados e normalizar props.
Sem lógica de negócio. Sem branching. Sem parsing. Delega para o UI.

    pub fn Button(variant: ButtonVariant, size: ButtonSize) -> impl IntoView {
        view! { <ButtonUi variant=variant size=size /> }
    }

### Block
Composição de boundaries em uma unidade semântica reutilizável.
Slots sao sempre Option<ChildrenFn> — funcoes lazy, nunca valores diretos.

    pub fn Hero(
        header: Option<ChildrenFn>,
        content: ChildrenFn,
        actions: Option<ChildrenFn>,
    ) -> impl IntoView {
        view! {
            <div data-rs-block="">
                {header.map(|h| view! { <div data-rs-region="header">{h()}</div> })}
                <div data-rs-region="content">{content()}</div>
                {actions.map(|a| view! { <div data-rs-region="actions">{a()}</div> })}
            </div>
        }
    }

### Layout
Estrutura de pagina. Slots sao Option<ChildrenFn> — define regioes nomeadas.

    pub fn MarketingLayout(
        header: Option<ChildrenFn>,
        hero: Option<ChildrenFn>,
        content: Option<ChildrenFn>,
        footer: Option<ChildrenFn>,
    ) -> impl IntoView { ... }

### Page (App)
Composicao final. Slots passados como closures — sem Arc, sem overhead.

    view! {
        <MarketingLayout
            hero=|| view! {
                <Hero
                    content=|| view! {
                        <HeroTitle>"Hello"</HeroTitle>
                    }.into_any()
                    actions=|| view! {
                        <Button variant=ButtonVariant::Primary>"Start"</Button>
                    }.into_any()
                />
            }.into_any()
        />
    }

---

## Regras

    Camada    | Tipo de slot        | Responsabilidade
    ----------|---------------------|------------------
    Primitive | —                   | contrato HTML + ARIA
    UI        | Children            | mapping puro 1:1
    Boundary  | Children            | API publica + defaults
    Block     | Option<ChildrenFn>  | composicao semantica
    Layout    | Option<ChildrenFn>  | estrutura de pagina
    Page      | ChildrenFn          | composicao final

Slot nao e valor. Slot e funcao.
