// CanonRS Interaction Runtime Loader — v0.1.0
// Cache bust via window.__CANON_WASM_HASH__ (never mutate this file)

// ─── Runtime Instrumentation ─────────────────────────────────────────────────
window.__canonRuntime = {
  _init_count: 0, _replay_count: 0, _observer_events: 0,
  _inited_uids: new Set(), _mod: null,
  _trackInit(uid) {
    if (uid && this._inited_uids.has(uid)) this._replay_count++;
    if (uid) this._inited_uids.add(uid);
    this._init_count++;
  },
  _trackObserver() { this._observer_events++; },
  _setMod(mod) { this._mod = mod; },
  get init_count()      { return this._init_count; },
  get replay_count()    { return this._replay_count; },
  get observer_events() { return this._observer_events; },
  get active_listeners()  { try { return this._mod?.runtime_active_listeners?.() ?? -1; } catch(e) { return -1; } },
  get orphan_listeners()  { try { return this._mod?.runtime_orphan_listeners?.() ?? -1; } catch(e) { return -1; } },
  get initialized_count() { try { return this._mod?.runtime_initialized_count?.() ?? -1; } catch(e) { return -1; } },
  get namespaces() { try { return Array.from(this._mod?.runtime_namespaces?.() ?? []); } catch(e) { return []; } },
  snapshot() {
    return {
      init_count: this.init_count, replay_count: this.replay_count,
      observer_events: this.observer_events, active_listeners: this.active_listeners,
      orphan_listeners: this.orphan_listeners, initialized_count: this.initialized_count,
      namespaces: this.namespaces,
    };
  }
};

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
    window.__canonRuntime._setMod(mod);
    window.__canonrs_init_all__ = () => { mod.init_all(); window.__canonRuntime._trackInit(null); };
    // GC periodico — limpa uids de elementos desconectados
    setInterval(() => { if (mod.gc) mod.gc(); }, 30000);
    console.log(`[canonrs] runtime ready — v0.1.0 hash=${hash}`);
  } catch (e) {
    console.error('[canonrs] failed to load runtime', e);
  }
})();

// MutationObserver — re-init componentes montados via Suspense/async
(function() {
  let rafPending = false;
  const observer = new MutationObserver((mutations) => {
    window.__canonRuntime._trackObserver();
    const hasNew = mutations.some(m =>
      Array.from(m.addedNodes).some(n => n.nodeType === 1 &&
        !(n.closest && n.closest('[data-rs-inline-editing]'))
      )
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
