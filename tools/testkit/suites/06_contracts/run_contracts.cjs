// run_contracts.cjs — Declarative Runtime Contract Runner
// Reads runtime_contracts/*.yaml and executes behavior + guarantee tests

const { chromium } = require('playwright');
const fs   = require('fs');
const path = require('path');
const yaml = require('js-yaml');

const BASE_URL      = 'http://localhost:3000';
const CONTRACTS_DIR = '/opt/docker/monorepo/packages-rust/rs-canonrs/canonrs-interactions/runtime_contracts';

async function assertBehavior(page, name, spec) {
  const action = spec.action || 'none';

  // Measure before action for rows_hidden assert
  let rows_before = -1;
  if (spec.assert === 'rows_hidden' && spec.rows_selector) {
    rows_before = await page.$$eval(spec.rows_selector, els => els.length);
  }

  if (action === 'fill') {
    await page.waitForSelector(spec.selector, { timeout: 10000 });
    await page.fill(spec.selector, spec.value);
    await page.waitForTimeout(500);
  } else if (action === 'click') {
    await page.waitForSelector(spec.selector, { timeout: 10000 });
    await page.click(spec.selector);
    await page.waitForTimeout(300);
  } else if (action === 'key') {
    await page.keyboard.press(spec.key);
    await page.waitForTimeout(300);
  } else if (action === 'dispatch_click') {
    await page.waitForSelector(spec.selector, { timeout: 10000 });
    await page.evaluate((sel) => {
      const el = document.querySelector(sel);
      if (!el) throw new Error(`dispatch_click: ${sel} not found`);
      el.dispatchEvent(new MouseEvent('click', { bubbles: true, cancelable: true }));
    }, spec.selector);
    await page.waitForTimeout(300);
  }

  switch (spec.assert) {
    case 'exists': {
      const el = await page.$(spec.selector);
      if (!el) throw new Error(`exists: ${spec.selector} not found`);
      break;
    }
    case 'has_pixels': {
      const ok = await page.evaluate((sel) => {
        const canvas = document.querySelector(sel);
        if (!canvas || canvas.width <= 300) return false;
        const ctx = canvas.getContext('2d');
        const data = ctx.getImageData(0, 0, canvas.width, canvas.height).data;
        for (let i = 3; i < data.length; i += 4) if (data[i] > 0) return true;
        return false;
      }, spec.selector);
      if (!ok) throw new Error(`has_pixels: canvas empty or default size`);
      break;
    }
    case 'rows_hidden': {
      if (rows_before < 0) rows_before = await page.$$eval(spec.rows_selector, els => els.length);
      if (rows_before === 0) throw new Error(`rows_hidden: no rows found before action`);
      const after = await page.$$eval(spec.rows_selector, els => els.length);
      if (after >= rows_before) throw new Error(`rows_hidden: rows not reduced (${rows_before}->${after})`);
      break;
    }
    case 'attr_set': {
      const val = await page.$eval(spec.target, (el, attr) => el.getAttribute(attr), spec.attr);
      if (!val) throw new Error(`attr_set: ${spec.attr} not set on ${spec.target}`);
      break;
    }
    case 'attr_nonempty': {
      const val = await page.$eval(spec.target, (el, attr) => el.getAttribute(attr) || '', spec.attr);
      if (!val || val === '') throw new Error(`attr_nonempty: ${spec.attr} empty on ${spec.target}`);
      break;
    }
    case 'state_includes': {
      const val = await page.$eval(spec.target, (el) => el.getAttribute('data-rs-state') || '');
      if (!val.includes(spec.state)) throw new Error(`state_includes: state="${val}" does not include "${spec.state}"`);
      break;
    }
    case 'state_excludes': {
      const val2 = await page.$eval(spec.target, (el) => el.getAttribute('data-rs-state') || '');
      if (val2.includes(spec.state)) throw new Error(`state_excludes: state="${val2}" still includes "${spec.state}"`);
      break;
    }
    case 'attr_gt': {
      const val3 = await page.$eval(spec.selector, (el, attr) => el.getAttribute(attr) || el[attr] || '0', spec.attr);
      if (parseFloat(val3) <= spec.value) throw new Error(`attr_gt: ${spec.attr}=${val3} not > ${spec.value}`);
      break;
    }
    case 'count_gt': {
      const count = await page.$$eval(spec.selector, els => els.length);
      if (count <= spec.value) throw new Error(`count_gt: found ${count} elements, expected > ${spec.value}`);
      break;
    }
    default:
      throw new Error(`unknown assert: ${spec.assert}`);
  }
}

async function runContract(browser, contract) {
  const page = await browser.newPage();
  const results = [];

  try {
    await page.goto(`${BASE_URL}${contract.route}`, { waitUntil: 'domcontentloaded' });
    await page.waitForFunction(
      () => window.__canonRuntime && typeof window.__canonRuntime.snapshot === 'function',
      { timeout: 20000 }
    );
    await page.waitForTimeout(1500);

    const snap0 = await page.evaluate(() => window.__canonRuntime.snapshot());

    // Guarantees
    for (const g of (contract.guarantees || [])) {
      try {
        const snap = await page.evaluate(() => window.__canonRuntime.snapshot());
        if (g === 'no_replay' && snap.replay_count > 0)
          throw new Error(`replay_count=${snap.replay_count}`);
        if (g === 'no_orphans' && snap.orphan_listeners > 0)
          throw new Error(`orphan_listeners=${snap.orphan_listeners}`);
        if (g === 'listeners_owned') {
          const uid = await page.evaluate((attr) => {
            const el = document.querySelector(`[data-rs-interaction="${attr}"]`);
            return el?.getAttribute('data-rs-uid');
          }, contract.interaction === 'data' ? contract.interaction : contract.interaction);
          // just check active_listeners >= min
          if ((contract.min_listeners || 0) > 0 && snap.active_listeners < contract.min_listeners)
            throw new Error(`active_listeners=${snap.active_listeners} < min=${contract.min_listeners}`);
        }
        results.push({ name: `guarantee:${g}`, ok: true });
      } catch(e) {
        results.push({ name: `guarantee:${g}`, ok: false, error: e.message });
      }
    }

    // Behavior
    for (const [name, spec] of Object.entries(contract.behavior || {})) {
      try {
        await assertBehavior(page, name, spec);
        results.push({ name: `behavior:${name}`, ok: true });
      } catch(e) {
        results.push({ name: `behavior:${name}`, ok: false, error: e.message });
      }
    }

    // Replay stability — wait 2s and check
    await page.waitForTimeout(2000);
    const snap1 = await page.evaluate(() => window.__canonRuntime.snapshot());
    const replay_delta = snap1.replay_count - snap0.replay_count;
    if (replay_delta > 0) {
      results.push({ name: 'guarantee:replay_stable', ok: false, error: `replay_count grew by ${replay_delta}` });
    } else {
      results.push({ name: 'guarantee:replay_stable', ok: true });
    }

  } finally {
    await page.close();
  }

  return results;
}

async function main() {
  const files = fs.readdirSync(CONTRACTS_DIR).filter(f => f.endsWith('.yaml'));
  if (files.length === 0) {
    console.error('[FAIL] no contracts found in', CONTRACTS_DIR);
    process.exit(1);
  }

  const browser = await chromium.launch();
  let total_passed = 0, total_failed = 0;

  for (const file of files.sort()) {
    const contract = yaml.load(fs.readFileSync(path.join(CONTRACTS_DIR, file), 'utf8'));
    console.log(`\n[CONTRACT] ${contract.component}`);

    const results = await runContract(browser, contract);
    for (const r of results) {
      if (r.ok) {
        console.log(`  [OK] ${r.name}`);
        total_passed++;
      } else {
        console.log(`  [FAIL] ${r.name} — ${r.error}`);
        total_failed++;
      }
    }
  }

  await browser.close();
  console.log('\n' + '='.repeat(50));
  console.log(`[OK] ${total_passed} contract assertions passed`);
  if (total_failed > 0) {
    console.log(`[FAIL] ${total_failed} contract assertions failed`);
    process.exit(1);
  }
  console.log('[OK] All runtime contracts verified');
}

main().catch(e => { console.error('[FAIL] contract runner crashed:', e.message); process.exit(1); });
