// listener_topology_tests.cjs — Runtime Listener Topology
const { chromium } = require('playwright');
const BASE_URL = 'http://localhost:3000';

async function run() {
  const browser = await chromium.launch();
  const page    = await browser.newPage();
  let passed = 0, failed = 0;

  const TESTS = [
    { name: 'topology: chart listener ownership', route: '/showcase/chart', selector: '[data-rs-chart]' },
    { name: 'topology: datatable listener ownership', route: '/showcase/data-table', selector: '[data-rs-datatable]' },
    { name: 'topology: dialog listener ownership', route: '/showcase/dialog', selector: '[data-rs-dialog]' },
  ];

  for (const t of TESTS) {
    try {
      await page.goto(`${BASE_URL}${t.route}`, { waitUntil: 'domcontentloaded' });
      await page.waitForFunction(
        () => window.__canonRuntime && typeof window.__canonRuntime.snapshot === 'function',
        { timeout: 20000 }
      );
      await page.waitForTimeout(1500);

      const result = await page.evaluate((sel) => {
        const el = document.querySelector(sel);
        if (!el) return { error: `${sel} not found` };
        const uid = el.getAttribute('data-rs-uid');
        if (!uid) return { error: 'no uid' };
        const snap      = window.__canonRuntime.snapshot();
        const lifecycle = window.__canonRuntime.lifecycle(uid);
        const ownership = window.__canonRuntime.ownership(uid);
        const listeners = parseInt(ownership.match(/listeners:(\d+)/)?.[1] || '0');
        const in_ns     = snap.namespaces.includes(uid);
        return { uid, lifecycle, ownership, listeners, in_ns, total: snap.active_listeners, orphans: snap.orphan_listeners };
      }, t.selector);

      if (result.error) throw new Error(result.error);
      if (result.lifecycle !== 'active') throw new Error(`lifecycle=${result.lifecycle} expected active`);
      if (result.listeners === 0) throw new Error(`listeners=0 — component has no listeners`);
      if (!result.in_ns) throw new Error(`uid=${result.uid} not in namespaces — not owned`);
      if (result.orphans > 0) throw new Error(`orphan_listeners=${result.orphans}`);

      console.log(`[OK] ${t.name} — uid=${result.uid} lifecycle=${result.lifecycle} ${result.ownership}`);
      passed++;
    } catch(e) {
      console.error(`[FAIL] ${t.name} — ${e.message}`);
      failed++;
    }
  }

  await browser.close();
  console.log('\n' + '='.repeat(50));
  console.log(`[OK] ${passed} listener topology tests passed`);
  if (failed > 0) { console.log(`[FAIL] ${failed} topology tests failed`); process.exit(1); }
  console.log('[OK] Listener topology certified');
}
run().catch(e => { console.error('[FAIL] crashed:', e.message); process.exit(1); });
