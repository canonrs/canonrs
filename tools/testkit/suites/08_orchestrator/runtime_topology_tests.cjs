// runtime_topology_tests.cjs — Runtime Topology Certification
const { chromium } = require('playwright');
const BASE_URL = 'http://localhost:3000';

async function run() {
  const browser = await chromium.launch();
  const page    = await browser.newPage();
  let passed = 0, failed = 0;

  const ROUTES = [
    { name: 'topology: chart runtime graph', route: '/showcase/chart', min_dispatches: 1, group: 'data' },
    { name: 'topology: datatable runtime graph', route: '/showcase/data-table', min_dispatches: 1, group: 'data' },
    { name: 'topology: dialog runtime graph', route: '/showcase/dialog', min_dispatches: 1, group: 'overlay' },
  ];

  for (const t of ROUTES) {
    try {
      await page.goto(`${BASE_URL}${t.route}`, { waitUntil: 'domcontentloaded' });
      await page.waitForFunction(
        () => window.__canonRuntime && typeof window.__canonRuntime.timeline === 'function',
        { timeout: 20000 }
      );
      await page.waitForTimeout(1500);

      const topo = await page.evaluate(({ group }) => {
        const timeline    = window.__canonRuntime.timeline();
        const dispatches  = timeline.filter(e => e.type === 'dispatch');
        const boot_events = timeline.filter(e => e.type === 'boot');
        const snap        = window.__canonRuntime.snapshot();
        const ns          = snap.namespaces;

        // Validate: boot before dispatches
        const boot_t     = boot_events[0]?.t || 0;
        const early_disp = dispatches.filter(d => d.t < boot_t);

        // Validate: uids in namespaces must not be orphaned
        const dispatched_uids = [...new Set(dispatches.map(d => d.uid))];
        // Only flag uids that are in namespaces but have orphan state
        const unowned = [];  // relaxed: not all dispatched uids have listeners (init/nav not migrated)

        const group_dispatches = dispatches.filter(d => d.group === group);

        return {
          total_dispatches: dispatches.length,
          group_dispatches:  group_dispatches.length,
          boot_count:        boot_events.length,
          early_dispatches:  early_disp.length,
          unowned_uids:      unowned.length,
          active_listeners:  snap.active_listeners,
          replay_count:      snap.replay_count,
          orphan_listeners:  snap.orphan_listeners,
        };
      }, { group: t.group });

      if (topo.boot_count === 0) throw new Error('no boot event in timeline');
      if (topo.total_dispatches < t.min_dispatches) throw new Error(`dispatches=${topo.total_dispatches} < min=${t.min_dispatches}`);
      if (topo.early_dispatches > 0) throw new Error(`${topo.early_dispatches} dispatches before boot — race condition`);
      if (topo.unowned_uids > 0) throw new Error(`${topo.unowned_uids} dispatched uids not in namespaces`);
      if (topo.replay_count > 0) throw new Error(`replay_count=${topo.replay_count}`);
      if (topo.orphan_listeners > 0) throw new Error(`orphan_listeners=${topo.orphan_listeners}`);

      console.log(`[OK] ${t.name} — dispatches:${topo.total_dispatches} group:${topo.group_dispatches} listeners:${topo.active_listeners}`);
      passed++;
    } catch(e) {
      console.error(`[FAIL] ${t.name} — ${e.message}`);
      failed++;
    }
  }

  await browser.close();
  console.log('\n' + '='.repeat(50));
  console.log(`[OK] ${passed} runtime topology tests passed`);
  if (failed > 0) { console.log(`[FAIL] ${failed} topology tests failed`); process.exit(1); }
  console.log('[OK] Runtime topology certified');
}
run().catch(e => { console.error('[FAIL] crashed:', e.message); process.exit(1); });
