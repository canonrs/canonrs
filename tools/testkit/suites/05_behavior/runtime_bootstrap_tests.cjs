// runtime_bootstrap_tests.cjs — Runtime Bootstrap Integrity
const { chromium } = require('playwright');
const BASE_URL = 'http://localhost:3000';

async function run() {
  const browser = await chromium.launch();
  const page    = await browser.newPage();
  let passed = 0, failed = 0;

  const ROUTES = ['/showcase/chart', '/showcase/dialog', '/showcase/data-table'];

  for (const route of ROUTES) {
    try {
      await page.goto(`${BASE_URL}${route}`, { waitUntil: 'domcontentloaded' });
      await page.waitForFunction(
        () => window.__canonRuntime && typeof window.__canonRuntime.snapshot === 'function',
        { timeout: 20000 }
      );
      await page.waitForTimeout(1000);

      const result = await page.evaluate(() => {
        const rt = window.__canonRuntime;
        const gr = window.__canonGroups;
        return {
          has_runtime:        typeof rt === 'object' && rt !== null,
          has_snapshot:       typeof rt?.snapshot === 'function',
          has_timeline:       typeof rt?.timeline === 'function',
          has_trace:          typeof rt?.trace === 'function',
          has_lifecycle:      typeof rt?.lifecycle === 'function',
          has_ownership:      typeof rt?.ownership === 'function',
          has_groups:         typeof gr === 'object' && gr !== null,
          has_groups_load:    typeof gr?.load === 'function',
          has_groups_init:    typeof gr?.initGroup === 'function',
          boot_event:         rt?.events('boot').length > 0,
          active_listeners:   rt?.active_listeners ?? -1,
          init_count:         rt?.init_count ?? 0,
        };
      });

      const failures = Object.entries(result)
        .filter(([k, v]) => v === false || v === -1)
        .map(([k]) => k);

      if (failures.length > 0)
        throw new Error(`missing: ${failures.join(', ')}`);

      console.log(`[OK] bootstrap: ${route} — listeners:${result.active_listeners} inits:${result.init_count}`);
      passed++;
    } catch(e) {
      console.error(`[FAIL] bootstrap: ${route} — ${e.message}`);
      failed++;
    }
  }

  await browser.close();
  console.log('\n' + '='.repeat(50));
  console.log(`[OK] ${passed} bootstrap tests passed`);
  if (failed > 0) { console.log(`[FAIL] ${failed} bootstrap tests failed`); process.exit(1); }
  console.log('[OK] Runtime bootstrap integrity certified');
}
run().catch(e => { console.error('[FAIL] crashed:', e.message); process.exit(1); });
