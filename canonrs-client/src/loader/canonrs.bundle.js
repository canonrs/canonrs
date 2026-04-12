// CanonRS Interaction Runtime Loader — v__CANONRS_VERSION__
// Loads 1 WASM + delegates all dispatch to Rust runtime

(async () => {
  try {
    const base = '/wasm';
    const js  = `${base}/canonrs_interactions.js`;
    const mod = await import(js);
    await mod.default();
    mod.init_all();
    console.log('[canonrs] runtime ready — v__CANONRS_VERSION__');
  } catch (e) {
    console.error('[canonrs] failed to load runtime', e);
  }
})();
