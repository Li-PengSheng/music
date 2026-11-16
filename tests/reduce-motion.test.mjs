import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

const css = readFileSync('src/app.css', 'utf8').replace(/\/\*[\s\S]*?\*\//g, '');

/** Rules whose selector list targets every element, including the reduce-motion variant. */
function universalRules(source) {
  const rules = [];
  for (const match of source.matchAll(/([^{}]+)\{([^{}]*)\}/g)) {
    const selectors = match[1].split(',').map((part) => part.trim());
    const universal = selectors.some(
      (selector) =>
        selector === '*' ||
        selector === '*::before' ||
        selector === '*::after' ||
        selector === '.reduce-motion *' ||
        selector === '.reduce-motion *::before' ||
        selector === '.reduce-motion *::after',
    );
    if (universal) rules.push({ selectors, body: match[2] });
  }
  return rules;
}

test('a universal reduce-motion duration does not transition every property', () => {
  const rules = universalRules(css);
  const timed = rules.filter((rule) => /transition-duration\s*:/.test(rule.body));
  assert.ok(
    timed.length >= 2,
    'both the media query and the in-app class set a universal duration',
  );
  for (const rule of timed) {
    assert.match(
      rule.body,
      /transition-property\s*:\s*none\s*!important/,
      rule.selectors.join(', '),
    );
  }
});
