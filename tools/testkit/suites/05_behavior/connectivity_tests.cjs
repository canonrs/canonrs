const { chromium } = require('playwright');

const BASE_URL = 'http://localhost:3000';

const CONNECTIVITY_TESTS = [
  { name: 'connectivity: chart', route: '/showcase/chart', attr: 'data-rs-chart', group: 'data', min_listeners: 1 },
  { name: 'connectivity: datatable', route: '/showcase/data-table', attr: 'data-rs-datatable', group: 'data', min_listeners: 10 },
  { name: 'connectivity: dialog', route: '/showcase/dialog', attr: 'data-rs-dialog', group: 'overlay', min_listeners: 1 },
  { name: 'connectivity: virtual_list', route: '/showcase/virtual-list', attr: 'data-rs-virtual-list', group: 'data', min_listeners: 1 },
];

async function runConnectivityTests() {
  const browser = await chromium.launch();
  const page    = await browser.newPage();
  let passed = 0, failed = 0;

  for (const t of CONNECTIVITY_TESTS) {
    try {
      await page.goto(`${BASE_URL}${t.route}`, { waitUntil: 'domcontentloaded' });
      await page.waitForFunction(
        () => window.__canonRuntime && typeof window.__canonRuntime.snapshot === 'function',
        { timeout: 20000 }
      );
      await page.waitForTimeout(1500);

      // 1. DOM: element exists
      const check = await page.evaluate(({ attr, group }) => {
        const el = document.querySelector(`[${attr}]`);
        if (!el) return { error: `DOM: [${attr}] not found` };
        const interaction = el.getAttribute('data-rs-interaction');
        if (interaction !== group) return { error: `interaction: data-rs-interaction="${interaction}" != "${group}"` };
        const uid = el.getAttribute('data-rs-uid');
        if (!uid) return { error: 'uid: data-rs-uid missing' };
        const initialized = el.hasAttribute('data-rs-initialized');
        if (!initialized) return { error: 'init: data-rs-initialized missing — dispatch did not run' };
        return { uid };
      }, { attr: t.attr, group: t.group });
      if (check.error) throw new Error(check.error);

      // 5. runtime alive — listeners registered
      const snap = await page.evaluate(() => window.__canonRuntime.snapshot());
      if (snap.active_listeners < t.min_listeners)
        throw new Error(`listeners: active=${snap.active_listeners} < min=${t.min_listeners}`);

      // 6. uid in namespaces — owned by runtime
      const in_ns = snap.namespaces.includes(check.uid);
      if (!in_ns) throw new Error(`ownership: uid=${check.uid} not in runtime namespaces — listeners not owned`);

      console.log(`[OK] ${t.name} — DOM✓ interaction✓ uid✓ init✓ listeners:${snap.active_listeners} owned✓`);
      passed++;
    } catch(e) {
      console.error(`[FAIL] ${t.name} — ${e.message}`);
      failed++;
    }
  }

  await browser.close();
  console.log('\n' + '='.repeat(50));
  console.log(`[OK] ${passed} connectivity tests passed`);
  if (failed > 0) { console.log(`[FAIL] ${failed} connectivity tests failed`); process.exit(1); }
  console.log('[OK] Interaction connectivity verified');
}

runConnectivityTests().catch(e => { console.error('[FAIL] connectivity runner crashed:', e.message); process.exit(1); });
