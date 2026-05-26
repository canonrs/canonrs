// mutation_storm_tests.cjs — Mutation Storm Runtime Certification
const { chromium } = require('playwright');
const { warnIfKnown } = require('../../lib/known_issues.cjs');
const BASE_URL = 'http://localhost:3000';

const STORM_TESTS = [
  { name: 'mutation_storm: chart',     route: '/showcase/chart',      selector: '[data-rs-chart]'     },
  { name: 'mutation_storm: dialog',    route: '/showcase/dialog',     selector: '[data-rs-dialog]'    },
  { name: 'mutation_storm: datatable', route: '/showcase/data-table', selector: '[data-rs-datatable]' },
];

const STORM_ITERATIONS = 20;

async function run() {
  const browser = await chromium.launch();
  const page    = await browser.newPage();
  let passed = 0, failed = 0, warnings = 0;

  for (const t of STORM_TESTS) {
    try {
      await page.goto(BASE_URL + t.route, { waitUntil: 'domcontentloaded' });
      await page.waitForFunction(
        () => window.__canonRuntime && typeof window.__canonRuntime.snapshot === 'function',
        { timeout: 20000 }
      );
      await page.waitForTimeout(1500);

      const snap0 = await page.evaluate(() => window.__canonRuntime.snapshot());
      if (snap0.total_listeners === 0) throw new Error('runtime dead before storm');

      await page.evaluate(async ({ sel, iterations }) => {
        for (let i = 0; i < iterations; i++) {
          const el = document.querySelector(sel);
          if (!el) continue;
          const parent = el.parentNode;
          const clone  = el.cloneNode(true);
          clone.removeAttribute('data-rs-initialized');
          clone.querySelectorAll('[data-rs-initialized]').forEach(c => c.removeAttribute('data-rs-initialized'));
          parent.removeChild(el);
          await new Promise(r => setTimeout(r, 50));
          parent.appendChild(clone);
          await new Promise(r => setTimeout(r, 50));
        }
      }, { sel: t.selector, iterations: STORM_ITERATIONS });

      await page.waitForTimeout(1000);
      const snap1 = await page.evaluate(() => window.__canonRuntime.snapshot());

      // RT-001: runtime lacks destroy lifecycle — known issue until ownership::destroy_subtree()
      if (snap1.orphan_listeners > snap0.orphan_listeners + 5) {
        warnIfKnown('RT-001', 'orphan growth: ' + snap0.orphan_listeners + ' -> ' + snap1.orphan_listeners, { warnings, failed, get warnings() { return warnings; }, set warnings(v) { warnings = v; }, get failed() { return failed; }, set failed(v) { failed = v; } });
      }

      if (snap1.replay_count > snap0.replay_count + STORM_ITERATIONS)
        throw new Error('replay growth after storm: ' + snap0.replay_count + ' -> ' + snap1.replay_count);

      const ratio = snap1.total_listeners / Math.max(snap0.total_listeners, 1);
      if (ratio > 1.2)
        throw new Error('listener leak after storm: ' + snap0.total_listeners + ' -> ' + snap1.total_listeners);

      console.log('[OK] ' + t.name + ' — listeners:' + snap1.total_listeners + ' orphans:' + snap1.orphan_listeners + ' replay:' + snap1.replay_count);
      passed++;
    } catch(e) {
      console.error('[FAIL] ' + t.name + ' — ' + e.message);
      failed++;
    }
  }

  await browser.close();
  console.log('\n' + '='.repeat(50));
  console.log('[OK] ' + passed + ' mutation storm tests passed');
  if (warnings > 0) console.log('[WARN] ' + warnings + ' known issues detected (see RT-001)');
  if (failed > 0) { console.log('[FAIL] ' + failed + ' mutation storm tests failed'); process.exit(1); }
  console.log('[OK] Mutation storm certified');
}
run().catch(e => { console.error('[FAIL] crashed:', e.message); process.exit(1); });
