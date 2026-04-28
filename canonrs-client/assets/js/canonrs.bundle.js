// CanonRS Interaction Runtime Loader — v0.1.0
// Cache bust via window.__CANON_WASM_HASH__ (never mutate this file)

(async () => {
  try {
    const base = '/wasm';
    await import('/js/wasm_hash.js').catch(() => {});
    const hash = window.__CANON_WASM_HASH__;
    if (!hash) throw new Error('[canonrs] wasm hash missing — run orchestrator first');
    const js   = `${base}/canonrs_interactions.js`;
    const wasm = `${base}/canonrs_interactions_bg.wasm?v=${hash}`;
    const mod  = await import(js);
    await mod.default({ module_or_path: wasm });
    mod.init_all();
    window.__canonrs_init_all__ = () => mod.init_all();
    console.log(`[canonrs] runtime ready — v0.1.0 hash=${hash}`);
  } catch (e) {
    console.error('[canonrs] failed to load runtime', e);
  }
})();

// MutationObserver — re-init componentes montados via Suspense/async
(function() {
  let rafPending = false;
  const observer = new MutationObserver((mutations) => {
    const hasNew = mutations.some(m =>
      Array.from(m.addedNodes).some(n => n.nodeType === 1)
    );
    if (hasNew && !rafPending) {
      rafPending = true;
      requestAnimationFrame(() => {
        if (window.__canonrs_init_all__) {
          window.__canonrs_init_all__();
          console.log('[canonrs] MutationObserver re-init');
        }
        rafPending = false;
      });
    }
  });
  if (document.body) {
    observer.observe(document.body, { childList: true, subtree: true });
  } else {
    document.addEventListener('DOMContentLoaded', () => {
      observer.observe(document.body, { childList: true, subtree: true });
    });
  }
})();

// Canon SSE reload — mesma porta, sem tunnel issue
(function canonReload(delay) {
  const es = new EventSource('/canon-reload');
  es.onmessage = () => { console.log('[canonrs] reload triggered'); location.reload(); };
  es.onopen    = () => { console.log('[canonrs] reload connected'); };
  es.onerror   = () => {
    es.close();
    const next = Math.min((delay || 1000) * 2, 30000);
    setTimeout(() => canonReload(next), next);
  };
})(1000);
