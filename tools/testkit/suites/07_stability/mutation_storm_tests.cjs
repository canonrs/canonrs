// mutation_storm_tests.cjs — Mutation Storm Certification
const { chromium } = require('playwright');
const BASE_URL = 'http://localhost:3000';

async function run() {
  const browser = await chromium.launch();
  const page    = await browser.newPage();
  let passed = 0, failed = 0;

  try {
    await page.goto(`${BASE_URL}/showcase/data-table`, { waitUntil: 'domcontentloaded' });
    await page.waitForFunction(
      () => window.__canonRuntime && typeof window.__canonRuntime.snapshot === 'function',
      { timeout: 20000 }
    );
    await page.waitForTimeout(1500);

    const snap0 = await page.evaluate(() => window.__canonRuntime.snapshot());

    // Flood DOM with 500 non-interaction mutations
    await page.evaluate(() => {
      const frag = document.createDocumentFragment();
      for (let i = 0; i < 500; i++) {
        const div = document.createElement('div');
        div.setAttribute('data-mutation-storm', String(i));
        div.textContent = `storm ${i}`;
        frag.appendChild(div);
      }
      document.body.appendChild(frag);
    });
    await page.waitForTimeout(500);

    const snap1 = await page.evaluate(() => window.__canonRuntime.snapshot());

    // No replay under mutation storm
    if (snap1.replay_count > snap0.replay_count)
      throw new Error(`replay under mutation storm: ${snap0.replay_count}->${snap1.replay_count}`);

    // Listeners stable — no growth from non-interaction mutations
    const listener_growth = snap1.active_listeners - snap0.active_listeners;
    if (listener_growth > 5)
      throw new Error(`listener growth under mutation storm: +${listener_growth}`);

    // No orphans
    if (snap1.orphan_listeners > 0)
      throw new Error(`orphan listeners: ${snap1.orphan_listeners}`);

    console.log(`[OK] mutation_storm: 500 mutations stable — replay:${snap1.replay_count} listener_growth:${listener_growth} orphans:${snap1.orphan_listeners}`);
    passed++;

    // Cleanup
    await page.evaluate(() => {
      document.querySelectorAll('[data-mutation-storm]').forEach(el => el.remove());
    });

    // Validate runtime still alive after cleanup
    const snap2 = await page.evaluate(() => window.__canonRuntime.snapshot());
    if (snap2.active_listeners === 0)
      throw new Error('runtime dead after mutation storm cleanup');
    console.log(`[OK] mutation_storm: runtime alive after cleanup — listeners:${snap2.active_listeners}`);
    passed++;

  } catch(e) {
    console.error(`[FAIL] mutation_storm — ${e.message}`);
    failed++;
  }

  await browser.close();
  console.log('\n' + '='.repeat(50));
  console.log(`[OK] ${passed} mutation storm tests passed`);
  if (failed > 0) { console.log(`[FAIL] ${failed} mutation storm tests failed`); process.exit(1); }
  console.log('[OK] Mutation storm certified');
}
run().catch(e => { console.error('[FAIL] crashed:', e.message); process.exit(1); });
