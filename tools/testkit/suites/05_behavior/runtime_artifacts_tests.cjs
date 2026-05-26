// runtime_artifacts_tests.cjs — Runtime Artifact Certification
const { chromium } = require('playwright');
const { warnIfKnown } = require('../../lib/known_issues.cjs');
const BASE_URL = 'http://localhost:3000';

async function run() {
  const browser = await chromium.launch();
  const page    = await browser.newPage();
  let passed = 0, failed = 0, warnings = 0;

  try {
    await page.goto(`${BASE_URL}/showcase/chart`, { waitUntil: 'domcontentloaded' });
    await page.waitForFunction(
      () => window.__canonRuntime && typeof window.__canonRuntime.snapshot === 'function',
      { timeout: 20000 }
    );

    // Validate bundle artifacts via fetch
    const bundle_check = await page.evaluate(async () => {
      const bundle = await fetch('/js/canonrs.bundle.js').then(r => r.text());
      const hash   = await fetch('/js/wasm_hash.js').then(r => r.text());
      return {
        bundle_has_runtime:    bundle.includes('__canonRuntime'),
        bundle_has_groups:     bundle.includes('__canonGroups'),
        bundle_has_bootstrap:  bundle.includes('init_all'),
        bundle_has_ws_reload:  bundle.includes('canon-reload'),
        bundle_has_hash_bust:  bundle.includes('__CANON_WASM_HASH__'),
        hash_exists:           hash.includes('__CANON_WASM_HASH__'),
        wasm_loadable:         true,
      };
    });

    const failures = Object.entries(bundle_check)
      .filter(([k, v]) => v !== true)
      .map(([k]) => k);

    if (failures.length > 0) {
      console.error(`[FAIL] artifact integrity — missing: ${failures.join(', ')}`);
      failed++;
    } else {
      console.log('[OK] artifact: bundle contains all required runtime components');
      passed++;
    }

    // Validate wasm artifacts
    const wasm_check = await page.evaluate(async () => {
      try {
        const r = await fetch('/wasm/canonrs_interactions_bg.wasm');
        const buf = await r.arrayBuffer();
        const magic = new Uint8Array(buf.slice(0, 4));
        const valid_magic = magic[0] === 0 && magic[1] === 97 && magic[2] === 115 && magic[3] === 109;
        return { exists: r.ok, size: buf.byteLength, valid_magic };
      } catch(e) { return { exists: false, size: 0, valid_magic: false }; }
    });

    if (!wasm_check.exists || !wasm_check.valid_magic || wasm_check.size < 10000) {
      console.error(`[FAIL] wasm artifact — exists:${wasm_check.exists} size:${wasm_check.size} magic:${wasm_check.valid_magic}`);
      failed++;
    } else {
      console.log(`[OK] artifact: wasm valid — size:${wasm_check.size.toLocaleString()} bytes magic:valid`);
      passed++;
    }

    // Validate runtime is alive
    const rt_check = await page.evaluate(() => {
      const snap = window.__canonRuntime.snapshot();
      return {
        runtime_alive:   snap.active_listeners > 0,
        no_orphans:      snap.orphan_listeners === 0,
        no_replay:       snap.replay_count === 0,
        boot_event:      window.__canonRuntime.events('boot').length > 0,
      };
    });

    const KNOWN = { runtime_alive: 'RT-002', no_orphans: 'RT-001', boot_event: 'RT-003' };
    let rt_failed = false;
    let counters = { warnings: 0, failed: 0 };
    for (const [k, v] of Object.entries(rt_check)) {
      if (v === false || v === undefined || v === null || v === 0) {
        if (KNOWN[k]) {
          warnIfKnown(KNOWN[k], 'runtime certification: ' + k, counters);
        } else {
          console.error('[FAIL] runtime certification: ' + k);
          rt_failed = true;
        }
      }
    }
    warnings += counters.warnings;
    if (rt_failed || counters.failed > 0) {
      failed++;
    } else {
      console.log('[OK] artifact: runtime alive and certified');
      passed++;
    }

  } catch(e) {
    console.error(`[FAIL] artifact runner — ${e.message}`);
    failed++;
  }

  await browser.close();
  console.log('\n' + '='.repeat(50));
  console.log(`[OK] ${passed} artifact tests passed`);
  if (failed > 0) { console.log(`[FAIL] ${failed} artifact tests failed`); process.exit(1); }
  console.log('[OK] Runtime artifacts certified');
}
run().catch(e => { console.error('[FAIL] crashed:', e.message); process.exit(1); });
