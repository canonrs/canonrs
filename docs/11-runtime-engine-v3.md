# CR-413 — Runtime Engine v3: Padronizacao e Contratos

## Contexto

Evolucao do runtime CanonRS de snapshot-based para event-driven incremental.
Documenta os contratos estabelecidos durante o ciclo de refinamento do engine.

---

## 1. Modelo de Init

### Antes (snapshot)
  init_all() rodava uma vez apos hydrate.
  Componentes montados via Suspense/async nao eram inicializados.

### Agora (incremental + reativo)
  init_all() -> scan inicial + ativa MutationObserver
  MutationObserver -> init_element(node) -> incremental por evento

### Contrato
  Todo componente com data-rs-interaction pode nascer a qualquer momento.
  O runtime garante inicializacao independente de timing.

---

## 2. init_element — logica de despacho

  pub fn init_element(el: &Element) {
      if el.has_attribute("data-rs-interaction") {
          if registry::should_init(el) {
              dispatcher::dispatch(el);
              scan_children(el); // escaneia filhos apenas se inicializado agora
          }
          return;
      }
      scan_children(el); // container sem interaction — escaneia filhos
  }

### Contrato
  - init_element e idempotente — pode ser chamado N vezes no mesmo elemento
  - scan_children so roda quando necessario — evita O(n^2)
  - registry garante que cada uid e processado uma unica vez

---

## 3. Registry — lifecycle de uids

  should_init(el) -> bool   // registra uid, retorna false se ja existe
  cleanup(el)               // remove uid individual
  gc_elements(removed)      // remove uids de lista de elementos removidos
  gc()                      // fallback — remove uids desconectados do DOM

### Contrato
  - HashSet<uid> via thread_local — idempotente garantido
  - gc reativo via removed_nodes do MutationObserver — sem polling
  - gc() como fallback a cada 30s — captura casos nao detectados pelo observer
  - cleanup() para remounts controlados

---

## 4. Observer — processamento em batch

  MutationObserver captura added_nodes e removed_nodes.
  Agrupa todas as mutations de um burst antes de processar.
  Deduplica por uid antes de chamar init_element.

  Fluxo:
    burst de mutations ->
    deduplicar por uid ->
    filtrar nodes com data-rs-interaction ->
    init_element em batch ->
    gc_elements nos removidos

### Contrato
  - Um unico MutationObserver global por documento
  - Processamento em batch — nunca por mutation individual
  - Filtragem antes do init — evita custo em nodes irrelevantes
  - GC reativo integrado — removed_nodes limpa registry imediatamente

---

## 5. Dispatcher — extensivel

  HashMap<String, Handler> via thread_local.
  Permite registro dinamico de novos grupos.

  Grupos builtin:
    init, nav, data, gesture, overlay, selection, content

  Extensao:
    dispatcher::register("meu-grupo", minha_fn);

### Contrato
  - Nenhum match hardcoded — tudo via registry de handlers
  - Novos grupos nao requerem mudanca no core
  - Handler e fn(Element) — zero overhead de closure

---

## 6. Scanner — query helpers

  query(selector)            // busca no documento inteiro
  query_within(root, sel)    // busca dentro de um subtree

### Contrato
  - query_within e usada apenas quando necessario (nodes nao registrados)
  - registry garante que query_within nao processa nodes ja inicializados

---

## 7. GC integrado no loader JS

  // canonrs.bundle.js
  setInterval(() => { if (mod.gc) mod.gc(); }, 30000);

### Contrato
  - gc() como fallback — nao e dependencia critica
  - GC reativo via observer e o mecanismo principal
  - Polling de 30s captura edge cases de unmount nao detectados

---

## Score do Runtime

  v1 (snapshot):     ~60%
  v2 (incremental):  ~80%
  v3 (event-driven): ~88-90%

---

## Regras Finais

  1. Nunca chamar init_all() para re-inicializar — usar init_element(node)
  2. Nunca usar data-rs-initialized no DOM — usar registry HashSet
  3. Todo componente interativo deve ter data-rs-uid unico
  4. Novos grupos de interacao registrar via dispatcher::register()
  5. GC reativo e automatico — nao chamar cleanup() manualmente salvo remount controlado
