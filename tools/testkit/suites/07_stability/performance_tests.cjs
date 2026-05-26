// performance_tests.cjs — Runtime performance governance
const { chromium } = require('playwright');

const BASE_URL = 'http://localhost:3000';

// Thresholds
const MAX_INIT_TIME_MS      = 3000;  // bootstrap must complete in 3s
const MAX_OBSERVER_EVENTS   = 20;    // observer churn after stable
const MAX_ACTIVE_LISTENERS  = 1000;  // max listeners per page

const PERF_TESTS = [
  {
    name: 'perf: chart init time',
    route: '/showcase/chart',
    min_listeners: 1,
  },
  {
    name: 'perf: datatable init time',
    route: '/showcase/data-table',
    min_listeners: 10,
  },
  {
    name: 'perf: dialog init time',
    route: '/showcase/dialog',
    min_listeners: 1,
  },
];

async function runPerfTests() {
  const browser = await chromium.launch();
  const page    = await browser.newPage();
  let passed = 0, failed = 0;

  for (const t of PERF_TESTS) {
    try {
      const t0 = Date.now();
      await page.goto(`${BASE_URL}${t.route}`, { waitUntil: 'domcontentloaded' });

      const min_listeners = t.min_listeners;
      await page.waitForFunction(
        () => {
          if (!window.__canonRuntime) return false;
          return typeof window.__canonRuntime.snapshot === 'function';
        },
        { timeout: MAX_INIT_TIME_MS }
      );
      await page.waitForTimeout(500);

      const init_time = Date.now() - t0;

      // Wait for stability
      await page.waitForTimeout(1000);
      const snap = await page.evaluate(() => window.__canonRuntime.snapshot());

      // Validate init time
      if (init_time > MAX_INIT_TIME_MS)
        throw new Error(`init too slow: ${init_time}ms > ${MAX_INIT_TIME_MS}ms`);

      // Validate observer churn
      if (snap.observer_events > MAX_OBSERVER_EVENTS)
        throw new Error(`observer churn: ${snap.observer_events} events > max ${MAX_OBSERVER_EVENTS}`);

      // Validate listener count
      if (snap.active_listeners > MAX_ACTIVE_LISTENERS)
        throw new Error(`too many listeners: ${snap.active_listeners} > max ${MAX_ACTIVE_LISTENERS}`);

      // Validate no orphans
      if (snap.orphan_listeners > 0)
        throw new Error(`orphan listeners: ${snap.orphan_listeners}`);

      console.log(`[OK] ${t.name} — init:${init_time}ms listeners:${snap.active_listeners} observer_events:${snap.observer_events} orphans:${snap.orphan_listeners}`);
      passed++;
    } catch(e) {
      console.error(`[FAIL] ${t.name} — ${e.message}`);
      failed++;
    }
  }

  await browser.close();
  console.log('\n' + '='.repeat(50));
  console.log(`[OK] ${passed} performance tests passed`);
  if (failed > 0) {
    console.log(`[FAIL] ${failed} performance tests failed`);
    process.exit(1);
  }
  console.log('[OK] Performance governance compliant');
}

runPerfTests().catch(e => { console.error('[FAIL] perf runner crashed:', e.message); process.exit(1); });
