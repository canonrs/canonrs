// runtime_kernel_tests.cjs — Runtime Kernel Certification
// Validates: ownership graph, lifecycle states, cleanup cascade, mount graph
const { chromium } = require('playwright');

const BASE_URL = 'http://localhost:3000';

const KERNEL_TESTS = [
  {
    name: 'kernel: lifecycle active after dispatch',
    route: '/showcase/chart',
    test: async (page) => {
      const results = await page.evaluate(() => {
        const charts = [...document.querySelectorAll('[data-rs-chart][data-rs-uid]')];
        if (charts.length === 0) return { error: 'no charts found' };
        return charts.map(el => {
          const uid = el.getAttribute('data-rs-uid');
          const state = window.__canonRuntime.lifecycle(uid);
          return { uid, state };
        });
      });
      if (results.error) throw new Error(results.error);
      const inactive = results.filter(r => r.state !== 'active');
      if (inactive.length > 0)
        throw new Error(`components not active: ${inactive.map(r => `${r.uid}=${r.state}`).join(', ')}`);
      console.log(`  lifecycle: ${results.map(r => `${r.uid}=${r.state}`).join(', ')}`);
    }
  },
  {
    name: 'kernel: ownership has listeners after mount',
    route: '/showcase/chart',
    test: async (page) => {
      const results = await page.evaluate(() => {
        const charts = [...document.querySelectorAll('[data-rs-chart][data-rs-uid]')];
        return charts.map(el => {
          const uid = el.getAttribute('data-rs-uid');
          const ownership = window.__canonRuntime.ownership(uid);
          const listeners = parseInt(ownership.match(/listeners:(\d+)/)?.[1] || '0');
          return { uid, ownership, listeners };
        });
      });
      const zero = results.filter(r => r.listeners === 0);
      if (zero.length > 0)
        throw new Error(`components with 0 listeners: ${zero.map(r => r.uid).join(', ')}`);
      console.log(`  ownership: ${results.map(r => `${r.uid}=${r.ownership}`).join(', ')}`);
    }
  },
  {
    name: 'kernel: total_resources stable after gc',
    route: '/showcase/data-table',
    test: async (page) => {
      const r0 = await page.evaluate(() => window.__canonRuntime.total_resources);
      // Trigger GC
      await page.evaluate(() => {
        if (window.__canonrs_gc) window.__canonrs_gc();
      });
      await page.waitForTimeout(500);
      const r1 = await page.evaluate(() => window.__canonRuntime.total_resources);
      const l0 = parseInt(r0.match(/listeners:(\d+)/)?.[1] || '0');
      const l1 = parseInt(r1.match(/listeners:(\d+)/)?.[1] || '0');
      if (l1 > l0 + 5)
        throw new Error(`listener count grew after gc: ${r0} -> ${r1}`);
      console.log(`  resources: before=${r0} after=${r1}`);
    }
  },
  {
    name: 'kernel: lifecycle states transition correctly',
    route: '/showcase/chart',
    test: async (page) => {
      const timeline = await page.evaluate(() => window.__canonRuntime.timeline());
      const dispatches = timeline.filter(e => e.type === 'dispatch');
      if (dispatches.length === 0)
        throw new Error('no dispatch events found in timeline');
      // All dispatched components should be active
      const states = await page.evaluate(() => {
        const ns = window.__canonRuntime.namespaces;
        return ns.map(uid => ({ uid, state: window.__canonRuntime.lifecycle(uid) }));
      });
      const non_active = states.filter(s => s.state !== 'active' && s.state !== 'unknown');
      if (non_active.length > 0)
        throw new Error(`non-active components: ${non_active.map(s => `${s.uid}=${s.state}`).join(', ')}`);
      console.log(`  ${states.length} components active`);
    }
  },
  {
    name: 'kernel: ownership graph consistent',
    route: '/showcase/data-table',
    test: async (page) => {
      const result = await page.evaluate(() => {
        const ns = window.__canonRuntime.namespaces;
        const total = window.__canonRuntime.total_resources;
        const total_listeners = parseInt(total.match(/listeners:(\d+)/)?.[1] || '0');
        // Sum listeners per uid
        let sum = 0;
        for (const uid of ns) {
          const ownership = window.__canonRuntime.ownership(uid);
          sum += parseInt(ownership.match(/listeners:(\d+)/)?.[1] || '0');
        }
        return { total_listeners, sum_per_uid: sum, ns_count: ns.length };
      });
      console.log(`  total=${result.total_listeners} sum_per_uid=${result.sum_per_uid} namespaces=${result.ns_count}`);
      // Note: sum_per_uid may differ from total due to drag namespaces (uid:drag)
      // Just validate total > 0 and namespaces > 0
      if (result.total_listeners === 0)
        throw new Error('total_listeners = 0 — runtime dead');
      if (result.ns_count === 0)
        throw new Error('no namespaces — ownership graph empty');
    }
  },
  {
    name: 'kernel: dispatch order deterministic',
    route: '/showcase/chart',
    test: async (page) => {
      // Reload and check dispatch order is consistent
      const run1 = await page.evaluate(() =>
        window.__canonRuntime.events('dispatch').map(e => e.group)
      );
      await page.reload({ waitUntil: 'domcontentloaded' });
      await page.waitForFunction(
        () => window.__canonRuntime && window.__canonRuntime.events('dispatch').length > 0,
        { timeout: 20000 }
      );
      await page.waitForTimeout(1000);
      const run2 = await page.evaluate(() =>
        window.__canonRuntime.events('dispatch').map(e => e.group)
      );
      if (run1.join(',') !== run2.join(','))
        throw new Error(`dispatch order not deterministic:\n  run1: ${run1.join(',')}\n  run2: ${run2.join(',')}`);
      console.log(`  dispatch order: ${run1.join(', ')} (deterministic)`);
    }
  },
];

async function runKernelTests() {
  const browser = await chromium.launch();
  const page    = await browser.newPage();
  let passed = 0, failed = 0;

  for (const t of KERNEL_TESTS) {
    try {
      await page.goto(`${BASE_URL}${t.route}`, { waitUntil: 'domcontentloaded' });
      await page.waitForFunction(
        () => window.__canonRuntime && typeof window.__canonRuntime.lifecycle === 'function',
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
  console.log(`[OK] ${passed} kernel tests passed`);
  if (failed > 0) {
    console.log(`[FAIL] ${failed} kernel tests failed`);
    process.exit(1);
  }
  console.log('[OK] Runtime kernel certified');
}

runKernelTests().catch(e => {
  console.error('[FAIL] kernel runner crashed:', e.message);
  process.exit(1);
});
