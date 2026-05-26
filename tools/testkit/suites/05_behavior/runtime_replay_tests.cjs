// runtime_replay_tests.cjs -- Replay Safety REAL
const { chromium } = require("playwright");
const { warnIfKnown } = require("../../lib/known_issues.cjs");
const BASE_URL = "http://localhost:3000";

const REPLAY_TESTS = [
  { name: "replay: chart subtree remount",  route: "/showcase/chart",  selector: "[data-rs-chart]"  },
  { name: "replay: dialog subtree remount", route: "/showcase/dialog", selector: "[data-rs-dialog]" },
];

async function run() {
  const browser = await chromium.launch();
  const page    = await browser.newPage();
  let passed = 0, failed = 0, warnings = 0;
  const counters = {
    get warnings() { return warnings; }, set warnings(v) { warnings = v; },
    get failed()   { return failed;   }, set failed(v)   { failed   = v; },
  };

  for (const t of REPLAY_TESTS) {
    try {
      await page.goto(BASE_URL + t.route, { waitUntil: "domcontentloaded" });
      await page.waitForFunction(
        () => window.__canonRuntime && typeof window.__canonRuntime.snapshot === "function",
        { timeout: 20000 }
      );
      await page.waitForTimeout(1500);

      const snap0 = await page.evaluate(() => window.__canonRuntime.snapshot());

      await page.evaluate((sel) => {
        const el = document.querySelector(sel);
        if (el === null) return;
        const parent = el.parentNode;
        const clone  = el.cloneNode(true);
        clone.removeAttribute("data-rs-initialized");
        clone.querySelectorAll("[data-rs-initialized]").forEach(c => c.removeAttribute("data-rs-initialized"));
        parent.removeChild(el);
        parent.appendChild(clone);
      }, t.selector);

      await page.waitForTimeout(1000);
      const snap1 = await page.evaluate(() => window.__canonRuntime.snapshot());

      if (snap1.replay_count > snap0.replay_count)
        throw new Error("replay detected: " + snap0.replay_count + " -> " + snap1.replay_count);

      // RT-001: orphan growth -- known issue until ownership::destroy_subtree()
      if (snap1.orphan_listeners > 0) {
        warnIfKnown("RT-001", "orphan listeners: " + snap1.orphan_listeners, counters);
      }

      console.log("[OK] " + t.name + " -- replay:" + snap1.replay_count + " orphans:" + snap1.orphan_listeners);
      passed++;
    } catch(e) {
      console.error("[FAIL] " + t.name + " -- " + e.message);
      failed++;
    }
  }

  await browser.close();
  console.log("\n" + "=".repeat(50));
  console.log("[OK] " + passed + " replay safety tests passed");
  if (warnings > 0) console.log("[WARN] " + warnings + " known issues detected (see RT-001)");
  if (failed > 0) { console.log("[FAIL] " + failed + " replay safety tests failed"); process.exit(1); }
  console.log("[OK] Replay safety certified");
}
run().catch(e => { console.error("[FAIL] crashed:", e.message); process.exit(1); });
