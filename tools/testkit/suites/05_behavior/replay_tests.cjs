const { chromium } = require('playwright');

const BASE_URL = 'http://localhost:3000';

// check_replay — validates init_count is stable over time (no replay loop)
// Uses window.__canonRuntime.snapshot() for evidence-based validation

const REPLAY_TESTS = [
  {
    name: 'replay: chart stable',
    route: '/showcase/chart',
    component: 'chart',
  },
  {
    name: 'replay: data_table stable',
    route: '/showcase/data-table',
    component: 'data_table',
  },
  {
    name: 'replay: dialog stable',
    route: '/showcase/dialog',
    component: 'dialog',
  },
  {
    name: 'replay: virtual_list stable',
    route: '/showcase/virtual-list',
    component: 'virtual_list',
  },
];

async function runReplayTests() {
  const browser = await chromium.launch();
  const page    = await browser.newPage();
  let passed = 0, failed = 0;
  const errors = [];

  for (const t of REPLAY_TESTS) {
    try {
      await page.goto(`${BASE_URL}${t.route}`, { waitUntil: 'domcontentloaded' });

      // Wait for runtime to initialize
      await page.waitForFunction(
        () => window.__canonRuntime && typeof window.__canonRuntime.snapshot === 'function',
        { timeout: 20000 }
      );
      await page.waitForTimeout(2000);

      // Snapshot T0
      const snap0 = await page.evaluate(() => window.__canonRuntime.snapshot());

      // Wait 3 seconds — replay loop would show up here
      await page.waitForTimeout(3000);

      // Snapshot T1
      const snap1 = await page.evaluate(() => window.__canonRuntime.snapshot());

      // Validate: init_count must not grow after initial bootstrap
      const init_delta    = snap1.init_count    - snap0.init_count;
      const replay_delta  = snap1.replay_count  - snap0.replay_count;
      const observer_delta = snap1.observer_events - snap0.observer_events;

      if (replay_delta > 0) {
        throw new Error(
          `replay_count grew by ${replay_delta} in 3s — replay loop detected\n` +
          `  T0: ${JSON.stringify(snap0)}\n` +
          `  T1: ${JSON.stringify(snap1)}`
        );
      }

      // observer_events growing is ok (user interactions) but init_count should not
      if (init_delta > 2) {
        throw new Error(
          `init_count grew by ${init_delta} in 3s without interaction — possible replay loop\n` +
          `  T0: ${JSON.stringify(snap0)}\n` +
          `  T1: ${JSON.stringify(snap1)}`
        );
      }

      // Validate: orphan_listeners should be 0
      if (snap1.orphan_listeners > 0) {
        throw new Error(
          `orphan_listeners=${snap1.orphan_listeners} — listener leak detected`
        );
      }

      console.log(`[OK] ${t.name} — init:${snap1.init_count} replay:${snap1.replay_count} observers:${snap1.observer_events} listeners:${snap1.active_listeners}`);
      passed++;
    } catch(e) {
      console.error(`[FAIL] ${t.name} — ${e.message}`);
      errors.push({ name: t.name, error: e.message });
      failed++;
    }
  }

  await browser.close();

  console.log('\n' + '='.repeat(50));
  console.log(`[OK] ${passed} replay tests passed`);
  if (failed > 0) {
    console.log(`[FAIL] ${failed} replay tests failed`);
    process.exit(1);
  }
  console.log('[OK] No replay loops detected');
}

runReplayTests().catch(e => {
  console.error('[FAIL] replay test runner crashed:', e.message);
  process.exit(1);
});
