# CR-414 — Padrao Runtime por Grupo de Interacao

## Contexto

O selection runtime foi revisado e serve como modelo obrigatorio para todos os
grupos de interacao: nav, overlay, init, data, gesture, content.

---

## Estrutura obrigatoria

Todo grupo de interacao deve ter:

  src/
    lib.rs              — entry point com init_{grupo}(el: Element)
    runtime/
      mod.rs            — re-exporta modulos
      lifecycle.rs      — init guard via state
      state.rs          — add/remove/has tokens em data-rs-state
      context.rs        — propagate_owner + find_root
      attrs.rs          — helpers de atributos DOM
      uid.rs            — geracao de uid atomico
      popup.rs          — click outside (se necessario)

---

## 1. lifecycle.rs — padrao obrigatorio

lifecycle.rs NAO controla init global.
O registry do orchestrador (canonrs-interactions) e a unica fonte de verdade para init.

lifecycle.rs controla apenas estado interno do componente — se ja esta configurado
internamente (listeners, aria, state inicial). Nao deve ser usado como guard global.

  use crate::runtime::state;

  /// Verifica se o componente ja foi configurado internamente.
  /// NAO usar como substituto do registry global.
  pub fn init_guard(el: &Element) -> bool {
      if state::has(el, "initialized") { return false; }
      state::add(el, "initialized");
      true
  }

Fluxo correto:
  orchestrador registry::should_init()  ->  guard global (unica fonte de verdade)
  lifecycle::init_guard()               ->  guard interno do componente (dupla verificacao)

PROIBIDO:
  el.get_attribute("data-rs-initialized")  // DOM flag — fragil no remount
  el.set_attribute("data-rs-initialized")  // idem
  usar lifecycle como substituto do registry global

---

## 2. uid.rs — padrao obrigatorio

Contador atomico puro. Sem SystemTime (nao existe no WASM).

  use std::sync::atomic::{AtomicU64, Ordering};

  static CTR: AtomicU64 = AtomicU64::new(0);

  pub fn generate(prefix: &str) -> String {
      let ctr = CTR.fetch_add(1, Ordering::SeqCst);
      format!("{}-{:016x}-{:08x}", prefix, ctr << 16 | (ctr ^ 0xdeadbeef), ctr as u32)
  }

PROIBIDO:
  std::time::SystemTime  // nao existe no WASM target

---

## 3. context.rs — padrao obrigatorio

propagate_owner: query seletivo em vez de query_selector_all("*").
find_root: query_selector direto em vez de loop O(n).

  pub fn propagate_owner(root: &Element) {
      let Some(uid) = root.get_attribute("data-rs-uid") else { return };
      let Ok(nodes) = root.query_selector_all(
          "[data-rs-uid], [data-rs-interaction], [data-rs-state]"
      ) else { return };
      for i in 0..nodes.length() {
          if let Some(el) = nodes.item(i).and_then(|n| n.dyn_into::<Element>().ok()) {
              if el.get_attribute("data-rs-uid").is_none() {
                  let _ = el.set_attribute("data-rs-owner", &uid);
              }
          }
      }
  }

  pub fn find_root(target: &Element, root_selector: &str) -> Option<Element> {
      // fast path O(depth)
      if let Ok(Some(el)) = target.closest(root_selector) { return Some(el); }
      // fallback via data-rs-owner
      let uid = target.get_attribute("data-rs-owner")?;
      let doc = web_sys::window().and_then(|w| w.document())?;
      doc.query_selector(&format!("[data-rs-uid='{}']", uid)).ok().flatten()
  }

PROIBIDO:
  query_selector_all("*")                    // O(n) global — muito caro
  loop manual sobre NodeList para find_root  // substituido por query_selector direto

---

## 4. state.rs — padrao obrigatorio

Token-based. Comparacao por split_whitespace — nunca contains().

  pub fn has(el: &Element, token: &str) -> bool {
      el.get_attribute("data-rs-state")
          .unwrap_or_default()
          .split_whitespace()
          .any(|t| t == token)
  }

PROIBIDO:
  s.contains("selected")  // "unselected".contains("selected") == true

---

## 5. lib.rs — entry point padrao

  pub fn init_{grupo}(el: web_sys::Element) {
      if el.has_attribute("data-rs-{componente-a}") { componente_a::init(el.clone()); }
      if el.has_attribute("data-rs-{componente-b}") { componente_b::init(el.clone()); }
  }

O orchestrador (canonrs-interactions) chama init_{grupo} via dispatcher.
O grupo nao precisa saber nada do orchestrador.

---

## 6. Relacao com o orchestrador

O orchestrador (canonrs-interactions) e responsavel por:
  - registry global de uids (HashSet via thread_local) — UNICA fonte de verdade
  - MutationObserver global
  - deduplicacao de inits
  - GC reativo

O grupo e responsavel por:
  - logica interna do componente
  - eventos (click, keydown, etc)
  - state management (open/closed, selected, etc)
  - lifecycle interno (init_guard via state — dupla verificacao apenas)

Separacao de responsabilidades:
  orchestrador -> quando inicializar (registry = fonte de verdade)
  grupo        -> como inicializar (comportamento interno)

REGRA CRITICA:
  O grupo NUNCA decide se deve ou nao ser inicializado globalmente.
  Essa decisao pertence exclusivamente ao orchestrador via registry.
  lifecycle::init_guard e apenas protecao interna — nao substitui o registry.

---

## Checklist para novos grupos

  [ ] lifecycle.rs usa state::has("initialized")
  [ ] uid.rs sem SystemTime
  [ ] context.rs propagate_owner com query seletivo
  [ ] context.rs find_root via query_selector direto
  [ ] state.rs usa split_whitespace — nunca contains()
  [ ] lib.rs expoe init_{grupo}(el: Element)
  [ ] registrado no dispatcher do orchestrador
