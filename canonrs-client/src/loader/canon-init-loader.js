// canon-init-loader.js — Mirror of canon-loader for init components
// Loads WASM init modules and calls init_all() on DOMContentLoaded + MutationObserver

const INIT_GROUPS = ['init'];

async function loadInitGroup(group) {
  try {
    const mod = await import(`/wasm/${group}/canonrs_interactions_${group}.js`);
    await mod.default();
    mod.init_all();
    console.log(`[canon-init] loaded: ${group}`);
  } catch (e) {
    console.warn(`[canon-init] failed: ${group}`, e);
  }
}

function initAll() {
  INIT_GROUPS.forEach(loadInitGroup);
}

// DOMContentLoaded
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', initAll);
} else {
  initAll();
}

// MutationObserver — re-init em novos elementos
let rafPending = false;
const observer = new MutationObserver((mutations) => {
  const hasNew = mutations.some(m =>
    Array.from(m.addedNodes).some(n => n.nodeType === 1)
  );
  if (hasNew && !rafPending) {
    rafPending = true;
    requestAnimationFrame(() => {
      initAll();
      rafPending = false;
    });
  }
});

observer.observe(document.body, { childList: true, subtree: true });
