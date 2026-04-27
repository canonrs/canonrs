# CR-405 â SSR Interaction Contract

## O Problema

Componentes interativos do CanonRS sao inicializados pelo runtime WASM uma unica vez. O runtime le o DOM no momento do init e assume que ele esta completo. Qualquer mutacao posterior nao faz parte do contrato.

Contrato obrigatorio:

> O DOM deve estar completo antes do runtime inicializar.

## Tipos de Componentes

**1. Snapshot Components**
Inicializados uma vez. Nao suportam re-render. Exigem dados presentes antes do init. Todo componente com data-rs-interaction e um Snapshot Component.

**2. Reactive Components**
Nao suportados pelo runtime atual. Requereriam runtime stateless e re-init apos mutacao.

## O Conflito com SPA Navigation

Em navegacao SPA, o componente monta no cliente sem passar pelo SSR. Dados externos chegam depois do mount â depois do runtime ja ter inicializado o componente com DOM vazio. O resultado e um Snapshot Component congelado em estado incompleto.

## Root Cause

SPA nav monta o AppShell no cliente. O runtime inicializa o Select com items=0. O fetch resolve depois. O DOM e atualizado, mas o runtime ja executou â o estado esta congelado. O problema nao e o mecanismo de deteccao. E o timing: o DOM deixou de ser deterministico no momento do init.

## Solucao

Hard navigation apos login. Forcar um request HTTP real garante que o SSR executa, o cookie esta disponivel, os dados chegam antes do render, e o DOM nasce completo.

```rust
#[cfg(feature = "hydrate")]
{
    let window = leptos::web_sys::window().unwrap();
    let _ = window.location().set_href("/dashboard");
}
```

## Regras

1. Dados criticos devem existir antes do init â via SSR ou prefetch antes do mount.

2. Fetch apos o mount e proibido para componentes interativos. SSR async, prefetch e client fetch antes da renderizacao sao validos.

3. Login, logout e troca de tenant usam hard nav â nunca navigate() do router.

4. SPA navigation so e segura quando nenhum novo componente interativo e montado.

5. data-rs-interaction implica lifecycle snapshot-only.

## Checklist

- O componente interativo depende de dados externos?
- Esses dados estao disponiveis antes do init?
- A navegacao que leva a este componente e hard nav ou SPA?
- Se SPA, nenhum novo componente interativo e montado?

Se qualquer resposta for negativa: usar hard nav ou resolver dados antes do mount.

## Padrao Correto

request HTTP -> SSR -> cookie -> fetch -> dados prontos
-> render HTML completo -> hydrate -> runtime inicializa -> DOM estavel
