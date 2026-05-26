const { chromium } = require('playwright');

const BASE_URL = 'http://localhost:3000';

const CANVAS_TESTS = [
  { name: 'canvas: chart line',  route: '/showcase/chart' },
];

async function runCanvasTests() {
  const browser = await chromium.launch();
  const page    = await browser.newPage();
  let passed = 0, failed = 0;

  for (const t of CANVAS_TESTS) {
    try {
      await page.goto(`${BASE_URL}${t.route}`, { waitUntil: 'domcontentloaded' });
      await page.waitForFunction(
        () => window.__canonRuntime && typeof window.__canonRuntime.snapshot === 'function',
        { timeout: 20000 }
      );
      await page.waitForTimeout(1500);

      const result = await page.evaluate(() => {
        const canvases = [...document.querySelectorAll('canvas[data-rs-chart-canvas]')];
        if (canvases.length === 0) return { error: 'no canvas found' };
        const results = canvases.map(canvas => {
          const w = canvas.width;
          const h = canvas.height;
          if (w <= 300 && h <= 150) return { uid: canvas.closest('[data-rs-uid]')?.getAttribute('data-rs-uid'), error: `canvas default size ${w}x${h} — set_canvas_dpi did not run` };
          try {
            const ctx = canvas.getContext('2d');
            const data = ctx.getImageData(0, 0, w, h).data;
            let nonTransparent = 0;
            for (let i = 3; i < data.length; i += 4) { if (data[i] > 0) nonTransparent++; }
            const pct = Math.round(nonTransparent / (w * h) * 100);
            if (nonTransparent === 0) return { uid: canvas.closest('[data-rs-uid]')?.getAttribute('data-rs-uid'), error: `canvas is empty — draw_chart did not run (${w}x${h})` };
            return { uid: canvas.closest('[data-rs-uid]')?.getAttribute('data-rs-uid'), ok: true, size: `${w}x${h}`, pixels: pct + '%' };
          } catch(e) { return { error: e.message }; }
        });
        return results;
      });

      if (result.error) throw new Error(result.error);
      const failures = result.filter(r => r.error);
      if (failures.length > 0) throw new Error(failures.map(f => f.error).join(', '));
      console.log(`[OK] ${t.name} — ${result.map(r => `${r.uid} ${r.size} ${r.pixels}`).join(', ')}`);
      passed++;
    } catch(e) {
      console.error(`[FAIL] ${t.name} — ${e.message}`);
      failed++;
    }
  }

  await browser.close();
  console.log('\n' + '='.repeat(50));
  console.log(`[OK] ${passed} canvas tests passed`);
  if (failed > 0) { console.log(`[FAIL] ${failed} canvas tests failed`); process.exit(1); }
  console.log('[OK] Canvas render verified');
}

runCanvasTests().catch(e => { console.error('[FAIL] canvas runner crashed:', e.message); process.exit(1); });
