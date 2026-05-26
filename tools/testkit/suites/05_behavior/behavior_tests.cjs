const { chromium } = require('playwright');

const BASE_URL = 'http://localhost:3000';

const BEHAVIOR_TESTS = [
  {
    name: 'behavior: datatable filter responds',
    route: '/showcase/data-table',
    test: async (page) => {
      await page.waitForSelector('[data-rs-datatable-filter]', { timeout: 10000 });
      const rows_before = await page.$$eval('[data-rs-datatable-row]:not([hidden])', els => els.length);
      await page.fill('[data-rs-datatable-filter]', 'zzzznotfound');
      await page.waitForTimeout(300);
      const rows_after = await page.$$eval('[data-rs-datatable-row]:not([hidden])', els => els.length);
      if (rows_after >= rows_before) throw new Error(`filter did not reduce rows: ${rows_before} -> ${rows_after}`);
    }
  },
  {
    name: 'behavior: datatable sort responds',
    route: '/showcase/data-table',
    test: async (page) => {
      await page.waitForSelector('[data-rs-datatable-head-cell][data-rs-sort-key]', { timeout: 10000 });
      const sorted = await page.evaluate(() => {
        const head = document.querySelector('[data-rs-datatable-head-cell][data-rs-sort-key]');
        if (!head) return 'no head cell';
        head.dispatchEvent(new MouseEvent('click', { bubbles: true, cancelable: true }));
        return null;
      });
      if (sorted) throw new Error(sorted);
      await page.waitForTimeout(300);
      const sort_asc = await page.$eval('[data-rs-datatable]', el => el.getAttribute('data-rs-sort-asc'));
      if (!sort_asc) throw new Error('sort state not set after click');
    }
  },
  {
    name: 'behavior: datatable selection responds',
    route: '/showcase/data-table',
    test: async (page) => {
      await page.waitForSelector('[data-rs-datatable-row]', { timeout: 10000 });
      const sel_result = await page.evaluate(() => {
        const table = document.querySelector('[data-rs-datatable][data-rs-selectable="true"]');
        if (!table) return { error: 'no selectable datatable' };
        const row = table.querySelector('[data-rs-datatable-body] [data-rs-datatable-row]');
        if (!row) return { error: 'no row in body' };
        row.dispatchEvent(new MouseEvent('click', { bubbles: true, cancelable: true }));
        return { table_uid: table.getAttribute('data-rs-uid') };
      });
      if (sel_result.error) throw new Error(sel_result.error);
      await page.waitForTimeout(300);
      const selected_ids = await page.evaluate((uid) => {
        const el = document.querySelector(`[data-rs-uid="${uid}"]`);
        return el?.getAttribute('data-rs-selected-ids') || '';
      }, sel_result.table_uid);
      if (!selected_ids || selected_ids === '') throw new Error(`selection state not set after click on uid=${sel_result.table_uid}`);
    }
  },
  {
    name: 'behavior: chart canvas renders pixels',
    route: '/showcase/chart',
    test: async (page) => {
      await page.waitForSelector('canvas[data-rs-chart-canvas]', { timeout: 10000 });
      await page.waitForTimeout(1000);
      const ok = await page.evaluate(() => {
        const canvas = document.querySelector('canvas[data-rs-chart-canvas]');
        if (!canvas || canvas.width <= 300) return false;
        const ctx = canvas.getContext('2d');
        const data = ctx.getImageData(0, 0, canvas.width, canvas.height).data;
        for (let i = 3; i < data.length; i += 4) { if (data[i] > 0) return true; }
        return false;
      });
      if (!ok) throw new Error('canvas has no pixels — chart did not render');
    }
  },
  {
    name: 'behavior: dialog opens on trigger',
    route: '/showcase/dialog',
    test: async (page) => {
      await page.waitForSelector('[data-rs-dialog-trigger]', { timeout: 10000 });
      await page.click('[data-rs-dialog-trigger]');
      await page.waitForTimeout(300);
      const state = await page.$eval('[data-rs-dialog]', el => el.getAttribute('data-rs-state') || '');
      if (!state.includes('open')) throw new Error(`dialog state after click: "${state}" — expected open`);
    }
  },
];

async function runBehaviorTests() {
  const browser = await chromium.launch();
  const page    = await browser.newPage();
  let passed = 0, failed = 0;

  for (const t of BEHAVIOR_TESTS) {
    try {
      await page.goto(`${BASE_URL}${t.route}`, { waitUntil: 'domcontentloaded' });
      await page.waitForFunction(
        () => window.__canonRuntime && typeof window.__canonRuntime.snapshot === 'function',
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
  console.log(`[OK] ${passed} behavior tests passed`);
  if (failed > 0) { console.log(`[FAIL] ${failed} behavior tests failed`); process.exit(1); }
  console.log('[OK] Component behavior verified');
}

runBehaviorTests().catch(e => { console.error('[FAIL] behavior runner crashed:', e.message); process.exit(1); });
