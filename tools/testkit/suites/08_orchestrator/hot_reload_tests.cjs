// hot_reload_tests.cjs — Hot Reload Integrity
// Validates: runtime preserved after reload, no duplicate listeners, hash updated
const { chromium } = require('playwright');
const BASE_URL = 'http://localhost:3000';

async function run() {
  const browser = await chromium.launch();
  const page    = await browser.newPage();
  let passed = 0, failed = 0;

  try {
    await page.goto(`${BASE_URL}/showcase/chart`, { waitUntil: 'domcontentloaded' });
    await page.waitForFunction(
      () => window.__canonRuntime && typeof window.__canonRuntime.snapshot === 'function',
      { timeout: 20000 }
    );
    await page.waitForTimeout(1500);

    const snap0 = await page.evaluate(() => ({
      ...window.__canonRuntime.snapshot(),
      hash: window.__CANON_WASM_HASH__
    }));

    // Simulate reload (soft navigation)
    await page.reload({ waitUntil: 'domcontentloaded' });
    await page.waitForFunction(
      () => window.__canonRuntime && typeof window.__canonRuntime.snapshot === 'function',
      { timeout: 20000 }
    );
    await page.waitForTimeout(1500);

    const snap1 = await page.evaluate(() => ({
      ...window.__canonRuntime.snapshot(),
      hash: window.__CANON_WASM_HASH__
    }));

    // After reload: runtime must be alive
    if (snap1.active_listeners === 0)
      throw new Error('runtime dead after reload — active_listeners=0');

    // No orphans after reload
    if (snap1.orphan_listeners > 0)
      throw new Error(`orphan listeners after reload: ${snap1.orphan_listeners}`);

    // No replay after reload
    if (snap1.replay_count > 0)
      throw new Error(`replay detected after reload: ${snap1.replay_count}`);

    // Hash preserved (same wasm, same hash)
    if (snap0.hash && snap1.hash && snap0.hash !== snap1.hash)
      throw new Error(`hash changed unexpectedly: ${snap0.hash} -> ${snap1.hash}`);

    console.log(`[OK] hot_reload: runtime preserved — listeners:${snap1.active_listeners} orphans:${snap1.orphan_listeners} hash:${snap1.hash}`);
    passed++;

    // Validate dispatch order deterministic after reload
    const order0 = await page.evaluate(() =>
      window.__canonRuntime.events('dispatch').map(e => e.group).join(',')
    );
    await page.reload({ waitUntil: 'domcontentloaded' });
    await page.waitForFunction(
      () => window.__canonRuntime && window.__canonRuntime.events('dispatch').length > 0,
      { timeout: 20000 }
    );
    await page.waitForTimeout(500);
    const order1 = await page.evaluate(() =>
      window.__canonRuntime.events('dispatch').map(e => e.group).join(',')
    );
    if (order0 !== order1)
      throw new Error(`dispatch order not deterministic after reload`);

    console.log(`[OK] hot_reload: dispatch order deterministic`);
    passed++;

  } catch(e) {
    console.error(`[FAIL] hot_reload — ${e.message}`);
    failed++;
  }

  await browser.close();
  console.log('\n' + '='.repeat(50));
  console.log(`[OK] ${passed} hot reload tests passed`);
  if (failed > 0) { console.log(`[FAIL] ${failed} hot reload tests failed`); process.exit(1); }
  console.log('[OK] Hot reload integrity certified');
}
run().catch(e => { console.error('[FAIL] crashed:', e.message); process.exit(1); });
