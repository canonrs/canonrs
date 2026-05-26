// chaos_tests.cjs — Chaos runtime testing
// Tests: rapid remount, DOM flood, resize storm, delayed interactions
const { chromium } = require('playwright');
const BASE_URL = 'http://localhost:3000';

const CHAOS_TESTS = [
  {
    name: 'chaos: rapid DOM mutation flood',
    route: '/showcase/data-table',
    test: async (page) => {
      const snap0 = await page.evaluate(() => window.__canonRuntime.snapshot());
      // Flood DOM with non-interaction mutations
      await page.evaluate(() => {
        for (let i = 0; i < 100; i++) {
          const div = document.createElement('div');
          div.setAttribute('data-chaos', i);
          div.textContent = 'chaos ' + i;
          document.body.appendChild(div);
        }
      });
      await page.waitForTimeout(500);
      const snap1 = await page.evaluate(() => window.__canonRuntime.snapshot());
      // observer may fire but replay must not grow
      if (snap1.replay_count > snap0.replay_count)
        throw new Error(`replay grew under DOM flood: ${snap0.replay_count}->${snap1.replay_count}`);
      if (snap1.orphan_listeners > 0)
        throw new Error(`orphan listeners after DOM flood: ${snap1.orphan_listeners}`);
      // cleanup
      await page.evaluate(() => {
        document.querySelectorAll('[data-chaos]').forEach(el => el.remove());
      });
    }
  },
  {
    name: 'chaos: resize storm 20x',
    route: '/showcase/chart',
    test: async (page) => {
      const snap0 = await page.evaluate(() => window.__canonRuntime.snapshot());
      for (let i = 0; i < 20; i++) {
        await page.setViewportSize({ width: 600 + (i % 5) * 100, height: 600 });
        await page.waitForTimeout(30);
      }
      await page.setViewportSize({ width: 1280, height: 800 });
      await page.waitForTimeout(300);
      const snap1 = await page.evaluate(() => window.__canonRuntime.snapshot());
      if (snap1.replay_count > snap0.replay_count)
        throw new Error(`replay grew under resize storm: ${snap0.replay_count}->${snap1.replay_count}`);
      if (snap1.orphan_listeners > 0)
        throw new Error(`orphan listeners after resize storm: ${snap1.orphan_listeners}`);
    }
  },
  {
    name: 'chaos: rapid overlay open/close storm',
    route: '/showcase/dialog',
    test: async (page) => {
      await page.waitForSelector('[data-rs-dialog-trigger]', { timeout: 10000 });
      const snap0 = await page.evaluate(() => window.__canonRuntime.snapshot());
      for (let i = 0; i < 15; i++) {
        await page.evaluate(() => {
          document.querySelector('[data-rs-dialog-trigger]')
            ?.dispatchEvent(new MouseEvent('click', { bubbles: true }));
        });
        await page.waitForTimeout(50);
        await page.keyboard.press('Escape');
        await page.waitForTimeout(50);
      }
      await page.waitForTimeout(300);
      const snap1 = await page.evaluate(() => window.__canonRuntime.snapshot());
      if (snap1.replay_count > snap0.replay_count)
        throw new Error(`replay grew under overlay storm: ${snap0.replay_count}->${snap1.replay_count}`);
      if (snap1.orphan_listeners > snap0.orphan_listeners)
        throw new Error(`orphan leak under overlay storm: ${snap0.orphan_listeners}->${snap1.orphan_listeners}`);
    }
  },
  {
    name: 'chaos: init storm detection',
    route: '/showcase/chart',
    test: async (page) => {
      await page.waitForTimeout(2000);
      const freq = await page.evaluate(() => window.__canonRuntime.init_frequency(2000));
      if (freq > 5)
        throw new Error(`init storm: ${freq} inits in last 2s — expected <= 5`);
    }
  },
];

async function runChaosTests() {
  const browser = await chromium.launch();
  const page    = await browser.newPage();
  let passed = 0, failed = 0;
  for (const t of CHAOS_TESTS) {
    try {
      await page.goto(`${BASE_URL}${t.route}`, { waitUntil: 'domcontentloaded' });
      await page.waitForFunction(
        () => window.__canonRuntime && typeof window.__canonRuntime.snapshot === 'function',
        { timeout: 20000 }
      );
      await page.waitForTimeout(1500);
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
  console.log(`[OK] ${passed} chaos tests passed`);
  if (failed > 0) { console.log(`[FAIL] ${failed} chaos tests failed`); process.exit(1); }
  console.log('[OK] Chaos runtime stable');
}
runChaosTests().catch(e => { console.error('[FAIL] chaos runner crashed:', e.message); process.exit(1); });
