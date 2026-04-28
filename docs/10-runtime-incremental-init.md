# CR-412 — Runtime Incremental Init Engine

## Contexto

O runtime CanonRS era snapshot-based — init_all rodava uma vez apos hydrate.
Componentes montados via Suspense/async nao eram inicializados.
Resultado: Select, Combobox, etc nasciam sem estado, sem eventos.

---

## Mudanca Estrutural

ANTES:
  hydrate -> init_all() -> scan DOM -> acabou

AGORA:
  hydrate -> init_all() -> scan DOM + ativa MutationObserver
  DOM muda -> observer -> init_element(node) -> incremental

---

## 1. init_element incremental

Antes: init_element inicializava apenas o no recebido.
Agora: inicializa o no E escaneia filhos via query_within.

  pub fn init_element(el: &Element) {
      if el.has_attribute("data-rs-interaction") {
          if registry::should_init(el) {
              dispatcher::dispatch(el);
          }
      }
      for child in scanner::query_within(el, "[data-rs-interaction]") {
          if registry::should_init(&child) {
              dispatcher::dispatch(&child);
          }
      }
  }

Resolve: componentes dentro de Suspense agora sao inicializados.

---

## 2. Registry UID-based (idempotente real)

Antes: data-rs-initialized no DOM — fragil, some no remount.
Agora: HashSet<uid> via thread_local — idempotente garantido.

  thread_local! {
      static INITED: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
  }

Resultado: init_all pode ser chamado N vezes sem efeito colateral.

---

## 3. MutationObserver em Rust

Ja existia em observer.rs mas era ineficaz porque init_element nao era recursivo.
Agora efetivo: qualquer no adicionado ao DOM dispara init_element incremental.

---

## 4. query_within

Nova funcao em scanner.rs — query_selector_all dentro de um subtree.

  pub fn query_within(root: &Element, selector: &str) -> Vec<Element>

Usada pelo init_element para capturar componentes dentro de containers async.

---

## Problemas conhecidos (roadmap)

1. query_within e O(n) por mutacao — pode ser O(n^2) em cenarios grandes
   Solucao futura: marcar nos ja processados antes de varrer filhos

2. Registry nunca limpa — HashSet cresce indefinidamente em apps longos
   Solucao futura: WeakMap / cleanup hook no unmount

3. MutationObserver e global e cego — processa todos os nodes
   Solucao futura: filtrar apenas nodes com atributos data-rs-*

4. Dispatcher string-based — nao extensivel via plugin
   Solucao futura: registry de handlers por grupo

---

## Score

  Runtime antes:  ~60%
  Runtime agora:  ~80-85%
  CanonRS total:  ~80%

---

## Regra Final

  Antes: componente devia nascer no momento certo
  Agora: componente pode nascer a qualquer momento

  O runtime nao depende mais de timing.
  Ele reage quando o DOM muda.
