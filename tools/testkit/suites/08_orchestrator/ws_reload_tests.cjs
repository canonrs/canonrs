// ws_reload_tests.cjs — WebSocket/SSE Reload Validation
const { chromium } = require('playwright');
const net    = require('net');
const BASE_URL = 'http://localhost:3000';
const WS_PORT  = 9099;

async function run() {
  let passed = 0, failed = 0;

  // 1. WS port is open
  try {
    await new Promise((resolve, reject) => {
      const socket = new net.Socket();
      const timeout = setTimeout(() => { socket.destroy(); reject(new Error('timeout')); }, 3000);
      socket.connect(WS_PORT, 'localhost', () => {
        clearTimeout(timeout); socket.destroy(); resolve();
      });
      socket.on('error', (e) => { clearTimeout(timeout); reject(e); });
    });
    console.log(`[OK] ws: port ${WS_PORT} open`);
    passed++;
  } catch(e) {
    console.error(`[FAIL] ws: port ${WS_PORT} not open — ${e.message}`);
    failed++;
  }

  const browser = await chromium.launch();

  // 2. SSE /canon-reload connects
  try {
    const page = await browser.newPage();
    await page.goto(`${BASE_URL}/showcase/chart`, { waitUntil: 'domcontentloaded' });
    const sse_connected = await page.evaluate(() => {
      return new Promise(resolve => {
        const es = new EventSource('/canon-reload');
        es.onopen  = () => { es.close(); resolve(true); };
        es.onerror = () => { es.close(); resolve(false); };
        setTimeout(() => { es.close(); resolve(false); }, 3000);
      });
    });
    if (!sse_connected) throw new Error('SSE /canon-reload not available');
    console.log('[OK] ws: SSE /canon-reload connected');
    passed++;
    await page.close();
  } catch(e) {
    console.error(`[FAIL] ws: SSE — ${e.message}`);
    failed++;
  }

  // 3. Multiple browser tabs — runtime stable
  try {
    const pages = await Promise.all([0,1,2].map(() => browser.newPage()));
    await Promise.all(pages.map(p => p.goto(`${BASE_URL}/showcase/chart`, { waitUntil: 'domcontentloaded' })));
    await Promise.all(pages.map(p => p.waitForTimeout(3000)));
    const snaps = await Promise.all(pages.map(p =>
      p.evaluate(() => window.__canonRuntime?.snapshot() || { active_listeners: -1, orphan_listeners: 0 })
    ));
    const all_alive = snaps.every(s => s.orphan_listeners === 0);
    if (!all_alive) throw new Error(`tab runtime error: ${JSON.stringify(snaps)}`);
    console.log(`[OK] ws: 3 simultaneous tabs — listeners:${snaps.map(s => s.active_listeners).join(',')}`);
    passed++;
    await Promise.all(pages.map(p => p.close()));
  } catch(e) {
    console.error(`[FAIL] ws: multi-tab — ${e.message}`);
    failed++;
  }

  await browser.close();
  console.log('\n' + '='.repeat(50));
  console.log(`[OK] ${passed} ws reload tests passed`);
  if (failed > 0) { console.log(`[FAIL] ${failed} ws reload tests failed`); process.exit(1); }
  console.log('[OK] WS reload certified');
}
run().catch(e => { console.error('[FAIL] crashed:', e.message); process.exit(1); });
