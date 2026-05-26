// hot_reload_tests.cjs — Hot Reload Runtime Certification
const { chromium } = require('playwright');
const BASE_URL = 'http://localhost:3000';

const HOT_RELOAD_TESTS = [
  { name: 'hot_reload: chart runtime survives',     route: '/showcase/chart',      selector: '[data-rs-chart]'     },
  { name: 'hot_reload: dialog runtime survives',    route: '/showcase/dialog',     selector: '[data-rs-dialog]'    },
  { name: 'hot_reload: datatable runtime survives', route: '/showcase/data-table', selector: '[data-rs-datatable]' },
];

async function run() {
  const browser = await chromium.launch();
  const page    = await browser.newPage();
  let passed = 0, failed = 0;

  for (const t of HOT_RELOAD_TESTS) {
    try {
      await page.goto(BASE_URL + t.route, { waitUntil: 'domcontentloaded' });
      await page.waitForFunction(
        () => window.__canonRuntime && typeof window.__canonRuntime.snapshot === 'function',
        { timeout: 20000 }
      );
      await page.waitForTimeout(1500);

      const snap0 = await page.evaluate(() => window.__canonRuntime.snapshot());
      if (snap0.total_listeners === 0) throw new Error('runtime dead before reload');

      await page.goto(BASE_URL + '/', { waitUntil: 'domcontentloaded' });
      await page.waitForTimeout(500);
      await page.goto(BASE_URL + t.route, { waitUntil: 'domcontentloaded' });
      await page.waitForFunction(
        () => window.__canonRuntime && typeof window.__canonRuntime.snapshot === 'function',
        { timeout: 20000 }
      );
      await page.waitForTimeout(1500);

      const snap1 = await page.evaluate(() => window.__canonRuntime.snapshot());

      if (snap1.orphan_listeners > 0)
        throw new Error('orphan listeners after reload: ' + snap1.orphan_listeners);

      if (snap1.replay_count > snap0.replay_count + 2)
        throw new Error('replay grew: ' + snap0.replay_count + ' -> ' + snap1.replay_count);

      const ratio = snap1.total_listeners / Math.max(snap0.total_listeners, 1);
      if (ratio > 1.1 || ratio < 0.5)
        throw new Error('listeners unstable: ' + snap0.total_listeners + ' -> ' + snap1.total_listeners);

      console.log('[OK] ' + t.name + ' — listeners:' + snap1.total_listeners + ' orphans:' + snap1.orphan_listeners + ' replay:' + snap1.replay_count);
      passed++;
    } catch(e) {
      console.error('[FAIL] ' + t.name + ' — ' + e.message);
      failed++;
    }
  }

  await browser.close();
  console.log('\n' + '='.repeat(50));
  console.log('[OK] ' + passed + ' hot reload tests passed');
  if (failed > 0) { console.log('[FAIL] ' + failed + ' hot reload tests failed'); process.exit(1); }
  console.log('[OK] Hot reload runtime certified');
}
run().catch(e => { console.error('[FAIL] crashed:', e.message); process.exit(1); });
