// observer_graph_tests.cjs — Observer topology governance
const { chromium } = require('playwright');
const BASE_URL = 'http://localhost:3000';

const GRAPH_TESTS = [
  {
    name: 'observer-graph: chart dispatch chain',
    route: '/showcase/chart',
    test: async (page) => {
      const timeline = await page.evaluate(() => window.__canonRuntime.timeline());
      const dispatches = timeline.filter(e => e.type === 'dispatch' && e.group === 'data');
      if (dispatches.length === 0)
        throw new Error('no data dispatches found — chart init chain broken');
      const boot = timeline.find(e => e.type === 'boot');
      if (!boot) throw new Error('no boot event found');
      // All dispatches must happen after boot
      const invalid = dispatches.filter(d => d.t < boot.t);
      if (invalid.length > 0)
        throw new Error(`${invalid.length} dispatches before boot — race condition`);
      console.log(`  dispatches: ${dispatches.length}, uids: ${dispatches.map(d => d.uid).join(', ')}`);
    }
  },
  {
    name: 'observer-graph: no dispatch after stable',
    route: '/showcase/chart',
    test: async (page) => {
      // Wait for stable state
      await page.waitForTimeout(3000);
      const t_stable = Date.now();
      await page.waitForTimeout(1000);
      const timeline = await page.evaluate(() => window.__canonRuntime.timeline());
      // No new dispatches after stable point
      const late_dispatches = timeline.filter(e => e.type === 'dispatch');
      // All dispatches should be early (within first 2s of boot)
      const boot = timeline.find(e => e.type === 'boot');
      if (boot) {
        const late = late_dispatches.filter(d => (d.t - boot.t) > 2000);
        if (late.length > 0)
          throw new Error(`${late.length} late dispatches after 2s: ${late.map(d => d.uid).join(', ')}`);
      }
    }
  },
  {
    name: 'observer-graph: observer events bounded',
    route: '/showcase/data-table',
    test: async (page) => {
      await page.waitForTimeout(2000);
      const observers = await page.evaluate(() => window.__canonRuntime.observers());
      if (observers.length > 15)
        throw new Error(`too many observer events: ${observers.length} — possible churn`);
    }
  },
  {
    name: 'observer-graph: replay chain absent',
    route: '/showcase/chart',
    test: async (page) => {
      await page.waitForTimeout(2000);
      const replays = await page.evaluate(() => window.__canonRuntime.replays());
      if (replays.length > 0)
        throw new Error(`replay chain detected: ${replays.length} replays — ${replays.map(r => r.uid).join(', ')}`);
    }
  },
];

async function runGraphTests() {
  const browser = await chromium.launch();
  const page    = await browser.newPage();
  let passed = 0, failed = 0;
  for (const t of GRAPH_TESTS) {
    try {
      await page.goto(`${BASE_URL}${t.route}`, { waitUntil: 'domcontentloaded' });
      await page.waitForFunction(
        () => window.__canonRuntime && typeof window.__canonRuntime.timeline === 'function',
        { timeout: 20000 }
      );
      await page.waitForTimeout(1000);
      await t.test(page);
      console.log(`[OK] ${t.name}`);
      passed++;
    } catch(e) {
      console.error(`[FAIL] ${t.name} — ${e.message}`);
      failed++;
    }
  }
  await browser.close();
  console.log('\n' + '='.repeat(50));
  console.log(`[OK] ${passed} observer graph tests passed`);
  if (failed > 0) { console.log(`[FAIL] ${failed} observer graph tests failed`); process.exit(1); }
  console.log('[OK] Observer graph topology clean');
}
runGraphTests().catch(e => { console.error('[FAIL] graph runner crashed:', e.message); process.exit(1); });
