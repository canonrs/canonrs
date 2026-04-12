// canonrs.bundle.js — Single entrypoint for CanonRS
// Reads manifest.json and loads all groups dynamically

(async function () {
  const CANONRS_VERSION = '0.1.0';
  const MANIFEST_URL = '/js/canonrs-manifest.json';
  const INIT_GROUPS = ['init'];

  // ── load manifest ──────────────────────────────────────────────────────────
  let manifest;
  try {
    const res = await fetch(MANIFEST_URL);
    manifest = await res.json();
  } catch (e) {
    console.warn('[canonrs] failed to load manifest', e);
    return;
  }

  // ── loader state ───────────────────────────────────────────────────────────
  window.__canonLoader = {
    version: manifest.version,
    loaded: new Set(),
    loading: new Map(),
    mods: {},
    initializedUids: new Set(),

    async loadGroup(group) {
      if (this.loaded.has(group)) { this.initGroup(group); return; }
      if (this.loading.has(group)) { await this.loading.get(group); return; }
      const entry = manifest.groups[group];
      if (!entry) return;
      const promise = (async () => {
        const mod = await import(entry.js);
        await mod.default();
        this.loaded.add(group);
        this.loading.delete(group);
        this.mods[group] = mod;
        console.log('[canonrs] loaded:', group);
        if (INIT_GROUPS.includes(group)) {
          if (typeof mod.init_all === 'function') mod.init_all();
        } else {
          this.initGroup(group);
        }
      })();
      this.loading.set(group, promise);
      try { await promise; } catch (e) { console.warn('[canonrs] failed:', group, e); }
    },

    initGroup(group) {
      const mod = this.mods[group];
      if (!mod) return;
      const initFn = mod[`init_${group}`];
      if (typeof initFn !== 'function') return;
      document.querySelectorAll(`[data-rs-interaction="${group}"]`).forEach(el => {
        if (this._isInitialized(el)) return;
        this._markInitialized(el);
        try { initFn(el); } catch (e) { console.warn('[canonrs] init failed:', group, el, e); }
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
      try { initFn(el); } catch (e) { console.warn('[canonrs] init failed:', group, el, e); }
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

  // ── load all groups from manifest ──────────────────────────────────────────
  const groups = Object.keys(manifest.groups);
  await Promise.all(groups.map(g => window.__canonLoader.loadGroup(g)));

  // ── MutationObserver ───────────────────────────────────────────────────────
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

  if (document.body) {
    observer.observe(document.body, { childList: true, subtree: true, attributeFilter: ['hidden'] });
  } else {
    document.addEventListener('DOMContentLoaded', () => {
      observer.observe(document.body, { childList: true, subtree: true, attributeFilter: ['hidden'] });
    });
  }

  console.log('[canonrs] bundle ready — v' + manifest.version);
})();
