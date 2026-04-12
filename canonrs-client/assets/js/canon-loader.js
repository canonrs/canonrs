// CanonRS — Enterprise WASM Loader
window.__canonLoader = {
  loaded: new Set(),
  mods: {},
  initializedUids: new Set(),

  async loadGroup(group) {
    if (this.loaded.has(group)) {
      this.initGroup(group);
      return;
    }
    try {
      const mod = await import(`/wasm/${group}/canonrs_interactions_${group}.js`);
      await mod.default();
      this.loaded.add(group);
      this.mods[group] = mod;
      console.log('[canon] loaded:', group);
      this.initGroup(group);
    } catch (e) {
      console.warn('[canon] failed:', group, e);
    }
  },

  initGroup(group) {
    const mod = this.mods[group];
    if (!mod) return;
    const initFn = mod[`init_${group}`];
    if (typeof initFn !== 'function') return;
    document.querySelectorAll(`[data-rs-interaction="${group}"]`).forEach(el => {
      if (this._isInitialized(el)) return;
      this._markInitialized(el);
      try { initFn(el); } catch(e) { console.warn('[canon] init failed:', group, el, e); }
    });
  },

  initElement(el) {
    if (!(el instanceof Element)) return;
    if (this._isInitialized(el)) return;
    const group = el.getAttribute('data-rs-interaction');
    if (!group) return;
    const mod = this.mods[group];
    if (!mod) { this.loadGroup(group); return; }
    const initFn = mod[`init_${group}`];
    if (typeof initFn !== 'function') return;
    this._markInitialized(el);
    try { initFn(el); } catch(e) { console.warn('[canon] init failed:', group, el, e); }
  },

  _isInitialized(el) {
    const uid = el.getAttribute('data-rs-uid');
    if (uid) return this.initializedUids.has(uid);
    return false; // WASM owns initialization state
  },

  _markInitialized(el) {
    const uid = el.getAttribute('data-rs-uid');
    if (uid) { this.initializedUids.add(uid); return; }
    // WASM owns data-rs-initialized — loader does not set it
  }
};

// Init groups — carregados via init_all() sem data-rs-interaction
const INIT_GROUPS = ['init'];

async function loadInitGroups() {
  for (const group of INIT_GROUPS) {
    try {
      const mod = await import(`/wasm/${group}/canonrs_interactions_${group}.js`);
      await mod.default();
      if (typeof mod.init_all === 'function') {
        mod.init_all();
        console.log('[canon] loaded:', group);
      }
    } catch (e) {
      console.warn('[canon] failed:', group, e);
    }
  }
}

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', loadInitGroups);
} else {
  loadInitGroups();
}

// MutationObserver — apenas novos elementos
const startObserver = () => {
  const observer = new MutationObserver((mutations) => {
    for (const m of mutations) {
      for (const node of m.addedNodes) {
        if (!(node instanceof Element)) continue;
        if (node.hasAttribute('data-rs-interaction')) {
          window.__canonLoader.initElement(node);
        }
        node.querySelectorAll('[data-rs-interaction]').forEach(el => {
          window.__canonLoader.initElement(el);
        });
      }
      // Handle hidden attribute removal (tab activation)
      if (m.type === 'attributes' && m.attributeName === 'hidden') {
        const el = m.target;
        if (el instanceof Element && !el.hasAttribute('hidden')) {
          el.querySelectorAll('[data-rs-interaction]').forEach(child => {
            window.__canonLoader.initElement(child);
          });
        }
      }
    }
  });
  observer.observe(document.body, {
    childList: true,
    subtree: true,
    attributeFilter: ['hidden']
  });
};

if (document.body) {
  startObserver();
} else {
  document.addEventListener('DOMContentLoaded', startObserver);
}
