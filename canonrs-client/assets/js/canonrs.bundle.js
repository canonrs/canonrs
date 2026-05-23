// CanonRS Interaction Runtime Loader — v0.1.0
// Cache bust via window.__CANON_WASM_HASH__ (never mutate this file)

// ─── Runtime Instrumentation ─────────────────────────────────────────────────
window.__canonRuntime = {
  _init_count: 0, _replay_count: 0, _observer_events: 0,
  _inited_uids: new Set(), _mod: null,
  _timeline: [],
  _max_timeline: 200,

  _event(type, data) {
    const entry = { t: Date.now(), type, ...data };
    this._timeline.push(entry);
    if (this._timeline.length > this._max_timeline)
      this._timeline.shift();
  },

  _trackInit(uid) {
    const is_replay = uid && this._inited_uids.has(uid);
    if (is_replay) this._replay_count++;
    if (uid) this._inited_uids.add(uid);
    this._init_count++;
    this._event('init', { uid: uid || 'global', replay: is_replay });
  },

  _trackObserver(added_count) {
    this._observer_events++;
    this._event('observer', { added: added_count || 0 });
  },

  _trackDispatch(group, uid) {
    this._event('dispatch', { group, uid });
  },

  _trackListener(ns, event) {
    this._event('listener', { ns, event });
  },

  _trackRaf(label) {
    this._event('raf', { label: label || 'unknown' });
  },

  _setMod(mod) {
    this._mod = mod;
    this._event('boot', { msg: 'wasm loaded' });
  },
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
  },

  // Full event timeline — last 200 events
  timeline() { return [...this._timeline]; },

  // Filtered views
  events(type) { return type ? this._timeline.filter(e => e.type === type) : [...this._timeline]; },
  replays()    { return this._timeline.filter(e => e.type === 'init' && e.replay); },
  observers()  { return this._timeline.filter(e => e.type === 'observer'); },
  listeners()  { return this._timeline.filter(e => e.type === 'listener'); },
  inits()      { return this._timeline.filter(e => e.type === 'init'); },

  // Init storm detection — inits in last N ms
  init_frequency(window_ms) {
    const now = Date.now();
    return this._timeline.filter(e => e.type === 'init' && (now - e.t) < window_ms).length;
  },

  // Causal summary — last N events as readable trace
  trace(n) {
    const events = this._timeline.slice(-(n || 20));
    return events.map(e => {
      const dt = e.t - (this._timeline[0]?.t || e.t);
      switch(e.type) {
        case 'boot':     return `+${dt}ms [BOOT] ${e.msg}`;
        case 'init':     return `+${dt}ms [INIT] uid=${e.uid}${e.replay ? ' REPLAY!' : ''}`;
        case 'dispatch': return `+${dt}ms [DISPATCH] group=${e.group} uid=${e.uid}`;
        case 'observer': return `+${dt}ms [OBSERVER] added=${e.added}`;
        case 'listener': return `+${dt}ms [LISTENER] ns=${e.ns} event=${e.event}`;
        case 'raf':      return `+${dt}ms [RAF] ${e.label}`;
        default:         return `+${dt}ms [${e.type.toUpperCase()}] ${JSON.stringify(e)}`;
      }
    }).join('\n');
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
    // Observe data-rs-initialized to track individual component inits
    const initObserver = new MutationObserver((mutations) => {
      for (const m of mutations) {
        if (m.type === 'attributes' && m.attributeName === 'data-rs-initialized') {
          const uid = m.target.getAttribute('data-rs-uid') || 'unknown';
          const group = m.target.getAttribute('data-rs-interaction') || 'unknown';
          window.__canonRuntime._trackDispatch(group, uid);
        }
      }
    });
    initObserver.observe(document.body, { subtree: true, attributes: true, attributeFilter: ['data-rs-initialized'] });
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
    const added_count = mutations.reduce((n, m) => n + m.addedNodes.length, 0);
    window.__canonRuntime._trackObserver(added_count);
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
