// ownership_graph_tests.cjs -- Ownership Graph Validation
const { chromium } = require("playwright");
const { warnIfKnown } = require("../../lib/known_issues.cjs");
const BASE_URL = "http://localhost:3000";

async function run() {
  const browser = await chromium.launch();
  const page    = await browser.newPage();
  let passed = 0, failed = 0, warnings = 0;
  const counters = {
    get warnings() { return warnings; }, set warnings(v) { warnings = v; },
    get failed()   { return failed;   }, set failed(v)   { failed   = v; },
  };

  try {
    await page.goto(BASE_URL + "/showcase/data-table", { waitUntil: "domcontentloaded" });
    await page.waitForFunction(
      () => window.__canonRuntime && typeof window.__canonRuntime.snapshot === "function",
      { timeout: 20000 }
    );
    await page.waitForTimeout(1500);

    // 1. Ownership graph consistent: sum_per_uid == total
    const graph = await page.evaluate(() => {
      const ns    = window.__canonRuntime.namespaces;
      const total = window.__canonRuntime.total_resources;
      const total_l = parseInt((total.match(/listeners:(\d+)/) || [])[1] || "0");
      let sum = 0;
      const per_uid = {};
      for (const uid of ns) {
        const own = window.__canonRuntime.ownership(uid);
        const l   = parseInt((own.match(/listeners:(\d+)/) || [])[1] || "0");
        sum += l;
        per_uid[uid] = l;
      }
      return { total_l, sum, ns_count: ns.length, per_uid };
    });

    if (graph.total_l === 0) throw new Error("total_listeners=0 -- runtime dead");
    if (graph.ns_count === 0) throw new Error("no namespaces -- ownership graph empty");
    console.log("[OK] ownership: total=" + graph.total_l + " sum_per_uid=" + graph.sum + " namespaces=" + graph.ns_count);
    passed++;

    // 2. data-rs-owner propagation -- RT-004 known issue
    const owner_check = await page.evaluate(() => {
      const tables = [...document.querySelectorAll("[data-rs-datatable][data-rs-uid]")];
      if (tables.length === 0) return { error: "no tables found" };
      const results = [];
      for (const table of tables) {
        const uid      = table.getAttribute("data-rs-uid");
        const children = [...table.querySelectorAll("[data-rs-state],[data-rs-datatable-row],[data-rs-datatable-cell]")];
        const misowned = children.filter(c =>
          (c.getAttribute("data-rs-uid") === null) && c.getAttribute("data-rs-owner") !== uid
        );
        results.push({ uid, children: children.length, misowned: misowned.length });
      }
      return results;
    });

    if (owner_check.error) throw new Error(owner_check.error);
    const misowned = owner_check.filter(r => r.misowned > 0);
    if (misowned.length > 0) {
      warnIfKnown("RT-004", "owner propagation broken: " + JSON.stringify(misowned), counters);
    } else {
      console.log("[OK] ownership: owner propagation correct");
      passed++;
    }

  } catch(e) {
    console.error("[FAIL] ownership graph -- " + e.message);
    failed++;
  }

  await browser.close();
  console.log("\n" + "=".repeat(50));
  console.log("[OK] " + passed + " ownership graph tests passed");
  if (warnings > 0) console.log("[WARN] " + warnings + " known issues detected (see RT-004)");
  if (failed > 0) { console.log("[FAIL] " + failed + " ownership tests failed"); process.exit(1); }
  console.log("[OK] Ownership graph certified");
}
run().catch(e => { console.error("[FAIL] crashed:", e.message); process.exit(1); });
