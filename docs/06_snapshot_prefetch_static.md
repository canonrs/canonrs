# CR-408 — Interaction Modes: Snapshot, Prefetch, Static

## O Principio

Nao importa quando o dado chega. Importa quando o componente nasce.

Componentes com data-rs-interaction sao inicializados pelo runtime uma unica vez.
O DOM deve estar completo no momento do init — sem excecoes.

---

## Comparacao

| Modo | Quando dados existem | DOM no init | Fonte dos dados |
|---|---|---|---|
| Snapshot | Antes do SSR render | Completo | SSR handler / cache sincrono |
| Prefetch | Antes do mount (via Suspense) | Completo | LocalResource + Suspense |
| Static | Compile-time | Completo | Hardcoded |

---

## Modo Snapshot

Dados existem antes do SSR render. DOM nasce completo no HTML inicial.
Runtime init encontra tudo pronto — sem MutationObserver, sem Suspense.

```
SSR handler (cookie disponivel)
  -> fetch dados
  -> render HTML completo com Select + items
  -> hydrate
  -> runtime init_all
  -> Select funcionando
```

Fontes validas:
- SSR handler com cookie no contexto do request
- Cache sincrono (sessionStorage lido antes do render)

Restricoes:
- Cache sincrono so funciona no cliente — SSR renderiza vazio e causa hydration mismatch
- Resource sem cookie disponivel retorna vazio — nao e Snapshot real

---

## Modo Prefetch

Dados chegam antes do mount do componente interativo — Suspense bloqueia o mount ate o fetch resolver.
O Select nunca nasce vazio. Runtime init via MutationObserver apos o Suspense resolver.

```
SSR -> HTML sem Select (LocalResource nao roda no SSR)
  -> hydrate
  -> LocalResource dispara fetch
  -> Suspense bloqueia mount do Select
  -> fetch resolve
  -> Select monta com DOM completo
  -> MutationObserver -> runtime init_all
  -> Select funcionando
```

Requisitos:
- Login via hard nav (set_href) — garante SSR fresh com cookie disponivel para o fetch
- Suspense envolve apenas o componente interativo — nunca o layout
- MutationObserver ativo no canonrs.bundle.js para capturar mounts pos-hydrate

Variante Modo 4 (cache):
- Login retorna projetos no payload
- Cliente grava no sessionStorage antes do hard nav
- LocalResource le cache — Suspense resolve sem fetch adicional

---

## Modo Static

Items fixos em tempo de compilacao. Sem fetch, sem async, sem Resource.
SSR renderiza o Select completo. Runtime init_all encontra DOM pronto diretamente.

```rust
<Select>
    <SelectTrigger>
        <SelectValue placeholder="Escolha...">{""}</SelectValue>
    </SelectTrigger>
    <SelectContent>
        <SelectItem value="a" selected=SelectionState::Unselected>"Opcao A"</SelectItem>
        <SelectItem value="b" selected=SelectionState::Unselected>"Opcao B"</SelectItem>
    </SelectContent>
</Select>
```

---

## O que nunca fazer

- Montar o componente interativo antes dos dados chegarem (sem Suspense)
- Usar Resource SSR sem cookie disponivel no contexto — retorna vazio, DOM incompleto
- Cache sincrono no SSR — diverge do cliente, causa hydration mismatch
- navigate() do router apos login — SPA nav nao garante SSR fresh
- Mutar o DOM do Select apos o init do runtime — comportamento indefinido

---

## Regra Final

```
Snapshot  -> dado antes do SSR
Prefetch  -> dado antes do mount
Static    -> dado antes de tudo

Em todos os casos: componente nasce com DOM completo.
```
