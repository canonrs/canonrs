// known_issues.cjs — Runtime Known Issues Registry
const fs   = require('fs');
const path = require('path');
const yaml = require('js-yaml');

const REGISTRY_PATH = path.join(__dirname, '..', 'runtime_known_issues.yaml');

let _registry = null;

function loadKnownIssues() {
  if (_registry) return _registry;
  try {
    const raw = fs.readFileSync(REGISTRY_PATH, 'utf8');
    _registry = yaml.load(raw);
  } catch(e) {
    console.warn('[WARN] known_issues registry not found: ' + e.message);
    _registry = {};
  }
  return _registry;
}

function knownIssue(id) {
  const registry = loadKnownIssues();
  return registry[id] || null;
}

function severity(id) {
  const issue = knownIssue(id);
  return issue ? issue.severity : 'fail';
}

function warnIfKnown(id, message, counters) {
  const issue = knownIssue(id);
  if (issue === null) {
    console.error('[FAIL] ' + id + ' not in registry: ' + message);
    counters.failed++;
    return;
  }
  if (issue.severity === 'warn') {
    console.warn('[WARN] ' + id + ': ' + message + ' fix: ' + (issue.fix || 'unknown'));
    counters.warnings++;
  } else {
    console.error('[FAIL] ' + id + ': ' + message);
    counters.failed++;
  }
}

module.exports = { loadKnownIssues, knownIssue, severity, warnIfKnown };
