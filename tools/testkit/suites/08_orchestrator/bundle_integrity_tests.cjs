// bundle_integrity_tests.cjs — Bundle Integrity Post-Orchestrator
// Validates bundle served by server has all runtime components and no syntax errors
const { chromium } = require('playwright');
const BASE_URL = 'http://localhost:3000';

const REQUIRED = [
  '__canonRuntime',
  '__canonGroups',
  'init_all',
  'canon-reload',
  '__CANON_WASM_HASH__',
  'snapshot',
  'timeline',
  'trace',
];

async function run() {
  const browser = await chromium.launch();
  const page    = await browser.newPage();
  let passed = 0, failed = 0;

  try {
    await page.goto(`${BASE_URL}/showcase/chart`, { waitUntil: 'domcontentloaded' });

    // 1. Fetch bundle and check content
    const bundle_result = await page.evaluate(async (required) => {
      const r   = await fetch('/js/canonrs.bundle.js');
      const src = await r.text();
      const missing = required.filter(k => !src.includes(k));
      return { size: src.length, missing };
    }, REQUIRED);

    if (bundle_result.missing.length > 0) {
      console.error(`[FAIL] bundle content — missing: ${bundle_result.missing.join(', ')}`);
      failed++;
    } else {
      console.log(`[OK] bundle: all runtime components present (${bundle_result.size} chars)`);
      passed++;
    }

    // 2. Validate no syntax errors — __canonRuntime must be defined
    const errors = [];
    page.on('pageerror', e => errors.push(e.message));
    await page.reload({ waitUntil: 'domcontentloaded' });
    await page.waitForTimeout(2000);

    const runtime_defined = await page.evaluate(() =>
      typeof window.__canonRuntime === 'object' && window.__canonRuntime !== null
    );

    if (!runtime_defined) {
      console.error(`[FAIL] bundle syntax — __canonRuntime not defined (syntax error?)`);
      if (errors.length > 0) console.error(`  errors: ${errors.slice(0,3).join(', ')}`);
      failed++;
    } else {
      console.log('[OK] bundle: no syntax errors, __canonRuntime defined');
      passed++;
    }

    // 3. Validate __canonGroups defined
    const groups_defined = await page.evaluate(() =>
      typeof window.__canonGroups === 'object' && typeof window.__canonGroups.load === 'function'
    );

    if (!groups_defined) {
      console.error('[FAIL] bundle: __canonGroups not defined');
      failed++;
    } else {
      console.log('[OK] bundle: __canonGroups defined with load()');
      passed++;
    }

    // 4. Validate runtime boots correctly
    await page.waitForFunction(
      () => window.__canonRuntime && typeof window.__canonRuntime.snapshot === 'function',
      { timeout: 20000 }
    );
    await page.waitForTimeout(1500);
    const snap = await page.evaluate(() => window.__canonRuntime.snapshot());
    if (snap.replay_count > 0) {
      console.error(`[FAIL] bundle: replay detected on boot: ${snap.replay_count}`);
      failed++;
    } else {
      console.log(`[OK] bundle: clean boot — listeners:${snap.active_listeners} replay:${snap.replay_count}`);
      passed++;
    }

  } catch(e) {
    console.error(`[FAIL] bundle_integrity — ${e.message}`);
    failed++;
  }

  await browser.close();
  console.log('\n' + '='.repeat(50));
  console.log(`[OK] ${passed} bundle integrity tests passed`);
  if (failed > 0) { console.log(`[FAIL] ${failed} bundle integrity tests failed`); process.exit(1); }
  console.log('[OK] Bundle integrity certified');
}
run().catch(e => { console.error('[FAIL] crashed:', e.message); process.exit(1); });
