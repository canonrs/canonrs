// CanonRS — Enterprise WASM Loader
window.__canonLoader = {
  loaded: new Set(),

  async loadGroup(group) {
    if (this.loaded.has(group)) return;

    try {
      const mod = await import(`/wasm/${group}/canonrs_interactions_${group}.js`);
      await mod.default();
      this.loaded.add(group);
      console.log('[canon] loaded:', group);

      // init all elements in this group
      const initFn = mod[`init_${group}`];
      if (typeof initFn === 'function') {
        document.querySelectorAll(`[data-rs-interaction="${group}"]`).forEach(el => {
          try { initFn(el); } catch(e) { console.warn('[canon] init failed:', group, el, e); }
        });
      }
    } catch (e) {
      console.warn('[canon] failed:', group, e);
    }
  }
};
