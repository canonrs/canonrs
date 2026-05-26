// memory_leak_tests.cjs — Listener growth stability under repeated init
const { chromium } = require('playwright');

const BASE_URL = 'http://localhost:3000';

const LEAK_TESTS = [
  {
    name: 'leak: dialog open/close 10x',
    route: '/showcase/dialog',
    test: async (page) => {
      await page.waitForSelector('[data-rs-dialog-trigger]', { timeout: 10000 });
      const snap0 = await page.evaluate(() => window.__canonRuntime.snapshot());

      for (let i = 0; i < 10; i++) {
        // open
        await page.evaluate(() => {
          const trigger = document.querySelector('[data-rs-dialog-trigger]');
          trigger?.dispatchEvent(new MouseEvent('click', { bubbles: true }));
        });
        await page.waitForTimeout(150);
        // close via escape
        await page.keyboard.press('Escape');
        await page.waitForTimeout(150);
      }

      const snap1 = await page.evaluate(() => window.__canonRuntime.snapshot());
      const listener_growth = snap1.active_listeners - snap0.active_listeners;
      const orphan_growth   = snap1.orphan_listeners - snap0.orphan_listeners;

      if (listener_growth > 5)
        throw new Error(`listener leak: grew by ${listener_growth} after 10 open/close cycles`);
      if (orphan_growth > 0)
        throw new Error(`orphan leak: grew by ${orphan_growth}`);

      return { listener_growth, orphan_growth, snap1 };
    }
  },
  {
    name: 'leak: datatable filter 20x',
    route: '/showcase/data-table',
    test: async (page) => {
      await page.waitForSelector('[data-rs-datatable-filter]', { timeout: 10000 });
      const snap0 = await page.evaluate(() => window.__canonRuntime.snapshot());

      for (let i = 0; i < 20; i++) {
        await page.evaluate((idx) => {
          const input = document.querySelector('[data-rs-datatable-filter]');
          if (input) {
            input.value = idx % 2 === 0 ? 'test' : '';
            input.dispatchEvent(new Event('input', { bubbles: true }));
          }
        }, i);
        await page.waitForTimeout(50);
      }

      const snap1 = await page.evaluate(() => window.__canonRuntime.snapshot());
      const listener_growth = snap1.active_listeners - snap0.active_listeners;
      const orphan_growth   = snap1.orphan_listeners - snap0.orphan_listeners;

      if (listener_growth > 5)
        throw new Error(`listener leak: grew by ${listener_growth} after 20 filter cycles`);
      if (orphan_growth > 0)
        throw new Error(`orphan leak: grew by ${orphan_growth}`);

      return { listener_growth, orphan_growth, snap1 };
    }
  },
  {
    name: 'leak: chart resize 5x',
    route: '/showcase/chart',
    test: async (page) => {
      await page.waitForSelector('canvas[data-rs-chart-canvas]', { timeout: 10000 });
      const snap0 = await page.evaluate(() => window.__canonRuntime.snapshot());

      for (let i = 0; i < 5; i++) {
        await page.setViewportSize({ width: 800 + i * 100, height: 600 });
        await page.waitForTimeout(200);
      }
      await page.setViewportSize({ width: 1280, height: 800 });

      const snap1 = await page.evaluate(() => window.__canonRuntime.snapshot());
      const listener_growth = snap1.active_listeners - snap0.active_listeners;
      const orphan_growth   = snap1.orphan_listeners - snap0.orphan_listeners;

      if (listener_growth > 5)
        throw new Error(`listener leak: grew by ${listener_growth} after 5 resize cycles`);
      if (orphan_growth > 0)
        throw new Error(`orphan leak: grew by ${orphan_growth}`);

      return { listener_growth, orphan_growth, snap1 };
    }
  },
];

async function runLeakTests() {
  const browser = await chromium.launch();
  const page    = await browser.newPage();
  let passed = 0, failed = 0;

  for (const t of LEAK_TESTS) {
    try {
      await page.goto(`${BASE_URL}${t.route}`, { waitUntil: 'domcontentloaded' });
      await page.waitForFunction(
        () => window.__canonRuntime && typeof window.__canonRuntime.snapshot === 'function',
        { timeout: 20000 }
      );
      await page.waitForTimeout(1500);

      const result = await t.test(page);
      console.log(`[OK] ${t.name} — listener_growth:${result.listener_growth} orphans:${result.orphan_growth} active:${result.snap1.active_listeners}`);
      passed++;
    } catch(e) {
      console.error(`[FAIL] ${t.name} — ${e.message}`);
      failed++;
    }
  }

  await browser.close();
  console.log('\n' + '='.repeat(50));
  console.log(`[OK] ${passed} memory leak tests passed`);
  if (failed > 0) {
    console.log(`[FAIL] ${failed} memory leak tests failed`);
    process.exit(1);
  }
  console.log('[OK] No listener leaks detected');
}

runLeakTests().catch(e => { console.error('[FAIL] leak runner crashed:', e.message); process.exit(1); });
