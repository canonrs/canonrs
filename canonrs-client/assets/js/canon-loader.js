// CanonRS — Bootstrap Engine (CR-401)
// Pipeline determinístico: DOMContentLoaded → stabilize → init

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
      this.initGroup(group);
    } catch (e) {
      console.warn('[canon] failed:', group, e);
      setTimeout(() => this.loadGroup(group), 100);
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
    return el.hasAttribute('data-rs-initialized');
  },

  _markInitialized(el) {
    const uid = el.getAttribute('data-rs-uid');
    if (uid) {
      this.initializedUids.add(uid);
    } else {
      el.setAttribute('data-rs-initialized', 'true');
    }
  }
};

// ─── Bootstrap Engine ────────────────────────────────────────────────────────

const bootstrap = {
  ran: false,

  stabilize() {
    return new Promise(resolve => {
      let frames = 0;
      const tick = () => {
        if (++frames > 3) { resolve(); return; }
        requestAnimationFrame(tick);
      };
      requestAnimationFrame(tick);
    });
  },

  async run() {
    if (this.ran) return;
    this.ran = true;
    await this.stabilize();
    await this.loadAll();
  },

  async loadAll() {
    // overlay — scan direto após estabilização
    await window.__canonLoader.loadGroup('overlay');

    // init — usa init_all para scan completo
    try {
      const mod = await import('/wasm/init/canonrs_interactions_init.js');
      await mod.default();
      window.__canonLoader.loaded.add('init');
      window.__canonLoader.mods['init'] = mod;
      if (typeof mod.init_all === 'function') {
        mod.init_all();
      }
    } catch (e) {
      console.warn('[canon] failed: init', e);
      setTimeout(() => bootstrap.loadAll(), 100);
    }

    // selection — usa init_all para scan completo
    try {
      const mod = await import('/wasm/selection/canonrs_interactions_selection.js');
      await mod.default();
      window.__canonLoader.loaded.add('selection');
      window.__canonLoader.mods['selection'] = mod;
      if (typeof mod.init_all === 'function') {
        mod.init_all();
      }
    } catch (e) {
      console.warn('[canon] failed: selection', e);
    }
  }
};

// ─── MutationObserver — elementos dinâmicos pós-bootstrap ────────────────────

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
      // tab activation
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

// ─── Entry point ─────────────────────────────────────────────────────────────

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', () => bootstrap.run());
} else {
  bootstrap.run();
}

if (document.body) {
  startObserver();
} else {
  document.addEventListener('DOMContentLoaded', startObserver);
}
