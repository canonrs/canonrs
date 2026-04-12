// CanonRS Interaction Runtime Loader — v0.1.0
// Loads 1 WASM + delegates all dispatch to Rust runtime

(async () => {
  try {
    const base = '/wasm';
    const js  = `${base}/canonrs_interactions.js`;
    const mod = await import(js);
    await mod.default();
    mod.init_all();
    console.log('[canonrs] runtime ready — v0.1.0');
  } catch (e) {
    console.error('[canonrs] failed to load runtime', e);
  }
})();
