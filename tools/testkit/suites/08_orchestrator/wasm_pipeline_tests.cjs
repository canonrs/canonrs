// wasm_pipeline_tests.cjs — WASM Pipeline Validation
const { chromium } = require('playwright');
const fs   = require('fs');
const path = require('path');
const BASE_URL   = 'http://localhost:3000';
const ASSETS_DIR = '/opt/docker/monorepo/packages-rust/rs-canonrs/canonrs-client/assets';

async function run() {
  const browser = await chromium.launch();
  const page    = await browser.newPage();
  let passed = 0, failed = 0;

  try {
    // 1. wasm served correctly
    await page.goto(`${BASE_URL}/showcase/chart`, { waitUntil: 'domcontentloaded' });
    const wasm_ok = await page.evaluate(async () => {
      const r = await fetch('/wasm/canonrs_interactions_bg.wasm');
      if (!r.ok) return false;
      const buf   = await r.arrayBuffer();
      const magic = new Uint8Array(buf.slice(0, 4));
      return magic[0] === 0 && magic[1] === 97 && magic[2] === 115 && magic[3] === 109;
    });
    if (!wasm_ok) throw new Error('wasm not served or invalid magic bytes');
    console.log('[OK] pipeline: wasm served with valid magic bytes');
    passed++;

    // 2. wasm_hash.js served and has valid hash
    const hash_ok = await page.evaluate(async () => {
      const r = await fetch('/js/wasm_hash.js');
      const t = await r.text();
      return t.includes('__CANON_WASM_HASH__') && t.length > 20;
    });
    if (!hash_ok) throw new Error('wasm_hash.js missing or invalid');
    console.log('[OK] pipeline: wasm_hash.js valid');
    passed++;

    // 3. hash matches between server and filesystem
    const server_hash = await page.evaluate(async () => {
      await import('/js/wasm_hash.js').catch(() => {});
      return window.__CANON_WASM_HASH__;
    });
    const hash_file = path.join(ASSETS_DIR, 'js/wasm_hash.js');
    const hash_content = fs.readFileSync(hash_file, 'utf8');
    const m = hash_content.match(/\'(\w+)\'/);
    const fs_hash = m ? m[1] : null;
    if (!server_hash) throw new Error('server hash not set in window');
    if (fs_hash && server_hash !== fs_hash) throw new Error(`hash mismatch: server=${server_hash} fs=${fs_hash}`);
    console.log(`[OK] pipeline: hash consistent — ${server_hash}`);
    passed++;

    // 4. bundle loads wasm successfully
    await page.waitForFunction(
      () => window.__canonRuntime && typeof window.__canonRuntime.snapshot === 'function',
      { timeout: 20000 }
    );
    await page.waitForTimeout(1500);
    const snap = await page.evaluate(() => window.__canonRuntime.snapshot());
    if (snap.initialized_count === 0 && snap.active_listeners === 0)
      throw new Error('wasm loaded but runtime empty');
    console.log(`[OK] pipeline: wasm init successful — listeners:${snap.active_listeners}`);
    passed++;

  } catch(e) {
    console.error(`[FAIL] wasm_pipeline — ${e.message}`);
    failed++;
  }

  await browser.close();
  console.log('\n' + '='.repeat(50));
  console.log(`[OK] ${passed} wasm pipeline tests passed`);
  if (failed > 0) { console.log(`[FAIL] ${failed} pipeline tests failed`); process.exit(1); }
  console.log('[OK] WASM pipeline certified');
}
run().catch(e => { console.error('[FAIL] crashed:', e.message); process.exit(1); });
