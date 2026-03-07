#!/usr/bin/env node
const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

console.log('🔧 CanonRS Token Engine - Full Pipeline\n');

// 1. GENERATE FAMILIES (Rust)
console.log('1️⃣ Generating families from Rust...');
try {
  execSync('cargo run --manifest-path /opt/docker/monorepo/packages-rust/rs-canonrs/canonrs-tools/family-engine/Cargo.toml', { 
    cwd: '/opt/docker/monorepo/packages-rust/rs-canonrs',
    stdio: 'inherit' 
  });
  console.log('✅ Families generated\n');
} catch (e) {
  console.error('❌ Family generation failed');
  process.exit(1);
}

// 2. GENERATE CANONRS.CSS
console.log('2️⃣ Generating canonrs.css...');

const STYLES = 'styles';
const entry = ['/* 🔒 AUTO-GENERATED */\n'];

entry.push('/* 1. CORE */');
entry.push('@import "./.generated/core.css";\n');

entry.push('/* 2. FAMILIES */');
fs.readdirSync(`${STYLES}/.generated`)
  .filter(f => f.startsWith('family-'))
  .sort()
  .forEach(f => entry.push(`@import "./.generated/${f}";`));
entry.push('');

entry.push('/* 3. BASE TOKENS */');
['globals', 'core', 'ui', 'layout', 'blocks'].forEach(t => {
  const p = `${STYLES}/tokens/base/${t}.css`;
  if (fs.existsSync(p)) entry.push(`@import "./tokens/base/${t}.css";`);
});
entry.push('');

entry.push('/* 4. THEMES */');
if (fs.existsSync(`${STYLES}/themes/light/ui.css`)) entry.push('@import "./themes/light/ui.css";');
if (fs.existsSync(`${STYLES}/themes/dark/ui.css`)) entry.push('@import "./themes/dark/ui.css";');
entry.push('');

entry.push('/* 5. VARIANTS */');
if (fs.existsSync(`${STYLES}/variants`)) {
  fs.readdirSync(`${STYLES}/variants`).sort().forEach(f => 
    entry.push(`@import "./variants/${f}";`)
  );
}
entry.push('');

entry.push('/* 6. UI */');
if (fs.existsSync(`${STYLES}/ui`)) {
  fs.readdirSync(`${STYLES}/ui`).sort().forEach(f => 
    entry.push(`@import "./ui/${f}";`)
  );
}

fs.writeFileSync(`${STYLES}/canonrs.css`, entry.join('\n'));
console.log('✅ canonrs.css\n');

// 3. BUNDLE
console.log('3️⃣ Bundling...');
function resolveImports(file, base = STYLES) {
  const content = fs.readFileSync(path.join(base, file), 'utf8');
  return content.replace(/@import\s+["']([^"']+)["'];/g, (match, imp) => {
    const resolved = path.join(path.dirname(file), imp);
    if (fs.existsSync(path.join(base, resolved))) {
      return `/* Bundled: ${imp} */\n${resolveImports(resolved, base)}`;
    }
    return match;
  });
}

fs.writeFileSync(`${STYLES}/canonrs.bundle.css`, resolveImports('canonrs.css'));
console.log('✅ Done!\n');
