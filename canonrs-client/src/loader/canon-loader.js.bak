// CanonRS — Bootstrap Engine (CR-401)
// Pipeline determinístico: DOMContentLoaded → stabilize → init

window.__canonLoader = {
  initializedUids: new Set(),
  mod: null,

  async load() {
    if (this.mod) return this.mod;
    const mod = await import('/wasm/canonrs_interactions.js');
    await mod.default();
    this.mod = mod;
    return mod;
  },

  async initElement(el) {
    if (!(el instanceof Element)) return;
    if (this._isInitialized(el)) return;
    const group = el.getAttribute('data-rs-interaction');
    if (!group) return;
    const mod = await this.load();
    if (typeof mod.init_subtree !== 'function') return;
    this._markInitialized(el);
    try { mod.init_subtree(el); } catch(e) { console.warn('[canon] init failed:', group, el, e); }
  },

  _isInitialized(el) {
    const uid = el.getAttribute('data-rs-uid');
    if (uid) return this.initializedUids.has(uid);
    return el.hasAttribute('data-rs-initialized');
  },

  _markInitialized(el) {
    const uid = el.getAttribute('data-rs-uid');
    if (uid) { this.initializedUids.add(uid); }
    else { el.setAttribute('data-rs-initialized', 'true'); }
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
    try {
      const mod = await window.__canonLoader.load();
      // init_all — scan completo do DOM
      if (typeof mod.init_all === 'function') mod.init_all();
      // inicializa todos os elementos com data-rs-interaction
      document.querySelectorAll('[data-rs-interaction]').forEach(el => {
        window.__canonLoader.initElement(el);
      });
    } catch(e) {
      console.warn('[canon] bootstrap failed:', e);
      setTimeout(() => bootstrap.run(), 200);
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
