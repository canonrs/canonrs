const { chromium } = require('playwright');

const BASE_URL = 'http://localhost:3000';

const ALIVE_TESTS = [
  { name: 'runtime: chart listeners alive', route: '/showcase/chart', min_listeners: 1, attr: 'data-rs-chart' },
  { name: 'runtime: datatable listeners alive', route: '/showcase/data-table', min_listeners: 10, attr: 'data-rs-datatable' },
  { name: 'runtime: dialog listeners alive', route: '/showcase/dialog', min_listeners: 1, attr: 'data-rs-dialog' },
];

async function runAliveTests() {
  const browser = await chromium.launch();
  const page    = await browser.newPage();
  let passed = 0, failed = 0;

  for (const t of ALIVE_TESTS) {
    try {
      await page.goto(`${BASE_URL}${t.route}`, { waitUntil: 'domcontentloaded' });
      await page.waitForFunction(
        () => window.__canonRuntime && typeof window.__canonRuntime.snapshot === 'function',
        { timeout: 20000 }
      );
      await page.waitForTimeout(1500);

      const snap = await page.evaluate(() => window.__canonRuntime.snapshot());

      // Validate component exists in DOM
      const exists = await page.evaluate((attr) => {
        return document.querySelector(`[${attr}]`) !== null;
      }, t.attr);
      if (!exists) throw new Error(`[${t.attr}] not found in DOM`);

      // Validate active listeners
      if (snap.active_listeners < t.min_listeners) {
        throw new Error(`active_listeners=${snap.active_listeners} < min=${t.min_listeners} — runtime may be dead`);
      }

      // Validate no orphans
      if (snap.orphan_listeners > 0) {
        throw new Error(`orphan_listeners=${snap.orphan_listeners} — listener leak detected`);
      }

      // Validate initialized_count > 0
      if (snap.initialized_count === 0) {
        throw new Error(`initialized_count=0 — no components initialized`);
      }

      console.log(`[OK] ${t.name} — listeners:${snap.active_listeners} initialized:${snap.initialized_count} orphans:${snap.orphan_listeners}`);
      passed++;
    } catch(e) {
      console.error(`[FAIL] ${t.name} — ${e.message}`);
      failed++;
    }
  }

  await browser.close();
  console.log('\n' + '='.repeat(50));
  console.log(`[OK] ${passed} runtime alive tests passed`);
  if (failed > 0) { console.log(`[FAIL] ${failed} runtime alive tests failed`); process.exit(1); }
  console.log('[OK] Runtime alive verified');
}

runAliveTests().catch(e => { console.error('[FAIL] alive runner crashed:', e.message); process.exit(1); });
