import { readFileSync, readdirSync, statSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { test } from 'node:test';
import assert from 'node:assert/strict';

const root = dirname(dirname(fileURLToPath(import.meta.url)));
const expectedUnused = ['library_cover', 'library_favorite'];

function extractInvokeNames(source) {
  const names = new Set();
  const length = source.length;
  let i = 0;
  let mode = 'code';
  while (i < length) {
    const char = source[i];
    const next = source[i + 1];
    if (mode === 'code') {
      if (char === '/' && next === '/') {
        mode = 'line';
        i += 2;
        continue;
      }
      if (char === '/' && next === '*') {
        mode = 'block';
        i += 2;
        continue;
      }
      if (char === "'") {
        mode = 'sq';
        i++;
        continue;
      }
      if (char === '"') {
        mode = 'dq';
        i++;
        continue;
      }
      if (char === '`') {
        mode = 'tpl';
        i++;
        continue;
      }
      if (
        source.startsWith('invoke', i) &&
        (i === 0 || !/[A-Za-z0-9_]/.test(source[i - 1])) &&
        !/[A-Za-z0-9_]/.test(source[i + 6] ?? '')
      ) {
        const name = readInvokeName(source, i + 6);
        if (name) names.add(name);
      }
      i++;
      continue;
    }
    if (mode === 'line') {
      if (char === '\n') mode = 'code';
      i++;
      continue;
    }
    if (mode === 'block') {
      if (char === '*' && next === '/') {
        mode = 'code';
        i += 2;
        continue;
      }
      i++;
      continue;
    }
    if (mode === 'sq' || mode === 'dq' || mode === 'tpl') {
      const quote = mode === 'sq' ? "'" : mode === 'dq' ? '"' : '`';
      if (char === '\\') {
        i += 2;
        continue;
      }
      if (char === quote) mode = 'code';
      i++;
    }
  }
  return names;
}

function readInvokeName(source, start) {
  let j = start;
  while (/\s/.test(source[j] ?? '')) j++;
  if (source[j] === '<') {
    let depth = 0;
    for (; j < source.length; j++) {
      if (source[j] === '<') depth++;
      else if (source[j] === '>') {
        depth--;
        if (depth === 0) {
          j++;
          break;
        }
      }
    }
    while (/\s/.test(source[j] ?? '')) j++;
  }
  if (source[j] !== '(') return '';
  j++;
  while (/\s/.test(source[j] ?? '')) j++;
  const quote = source[j];
  if (quote !== "'" && quote !== '"') return '';
  const end = source.indexOf(quote, j + 1);
  if (end < 0) return '';
  const name = source.slice(j + 1, end);
  return /^[a-z0-9_]+$/.test(name) ? name : '';
}

function stripComments(source) {
  let out = '';
  let i = 0;
  let mode = 'code';
  while (i < source.length) {
    const char = source[i];
    const next = source[i + 1];
    if (mode === 'code') {
      if (char === '/' && next === '/') {
        mode = 'line';
        i += 2;
        continue;
      }
      if (char === '/' && next === '*') {
        mode = 'block';
        i += 2;
        continue;
      }
      if (char === "'") {
        mode = 'sq';
        out += char;
        i++;
        continue;
      }
      if (char === '"') {
        mode = 'dq';
        out += char;
        i++;
        continue;
      }
      if (char === '`') {
        mode = 'tpl';
        out += char;
        i++;
        continue;
      }
      out += char;
      i++;
      continue;
    }
    if (mode === 'line') {
      if (char === '\n') {
        mode = 'code';
        out += char;
      }
      i++;
      continue;
    }
    if (mode === 'block') {
      if (char === '*' && next === '/') {
        mode = 'code';
        i += 2;
        continue;
      }
      i++;
      continue;
    }
    out += char;
    const quote = mode === 'sq' ? "'" : mode === 'dq' ? '"' : '`';
    if (char === '\\') {
      out += source[i + 1] ?? '';
      i += 2;
      continue;
    }
    if (char === quote) mode = 'code';
    i++;
  }
  return out;
}

function extractRegistered(source) {
  const names = new Set();
  const text = stripComments(source);
  const pattern = /generate_handler!\s*\[([\s\S]*?)\]/g;
  let match;
  while ((match = pattern.exec(text))) {
    for (const part of match[1].split(',')) {
      const token = part.trim();
      if (!token) continue;
      const name = token.split('::').pop().trim();
      if (/^[A-Za-z0-9_]+$/.test(name)) names.add(name);
    }
  }
  return names;
}

function extractHandlerKeys(source) {
  const text = stripComments(source);
  const at = text.indexOf('const handlers');
  if (at < 0) throw new Error('handlers object not found');
  let i = text.indexOf('{', at);
  let depth = 0;
  const keys = new Set();
  let mode = 'code';
  for (; i < text.length; i++) {
    const char = text[i];
    const next = text[i + 1];
    if (mode === 'code') {
      if (char === "'") {
        mode = 'sq';
        continue;
      }
      if (char === '"') {
        mode = 'dq';
        continue;
      }
      if (char === '`') {
        mode = 'tpl';
        continue;
      }
      if (char === '{') {
        depth++;
        continue;
      }
      if (char === '}') {
        depth--;
        if (depth === 0) break;
        continue;
      }
      if (depth === 1 && /[A-Za-z_]/.test(char)) {
        const found = /^[A-Za-z_][A-Za-z0-9_]*/.exec(text.slice(i));
        const name = found[0];
        let j = i + name.length;
        while (/\s/.test(text[j] ?? '')) j++;
        if (text[j] === ':') keys.add(name);
        i = j - 1;
      }
      continue;
    }
    const quote = mode === 'sq' ? "'" : mode === 'dq' ? '"' : '`';
    if (char === '\\') {
      i++;
      continue;
    }
    if (char === quote) mode = 'code';
  }
  return keys;
}

function extractFence(markdown, info) {
  const pattern = new RegExp('```' + info + '\\r?\\n([\\s\\S]*?)```');
  const match = pattern.exec(markdown);
  assert.ok(match, `missing fence ${info}`);
  return new Set(
    match[1]
      .split(/\r?\n/)
      .map((line) => line.trim())
      .filter((line) => line.length > 0),
  );
}

function rustSources(dir, found = []) {
  for (const name of readdirSync(dir)) {
    const path = join(dir, name);
    if (statSync(path).isDirectory()) rustSources(path, found);
    else if (name.endsWith('.rs')) found.push(path);
  }
  return found;
}

function diff(left, right) {
  return [...left].filter((name) => !right.has(name)).sort();
}

test('invoke extraction ignores comments, strings, and repeated calls', () => {
  const sample = `
    // invoke('not_a_command')
    /* invoke("also_no") */
    invoke<Record<string, number>>(
      'library_cover_stats'
    );
    invoke('library_cover_stats');
    const sample = "invoke('inside_string')";
  `;
  assert.deepEqual([...extractInvokeNames(sample)], ['library_cover_stats']);
});

test('registration extraction ignores comments and keeps the command name', () => {
  const sample = `
    generate_handler![
      desktop::desktop_preferences,
      // player_fake,
      player_get_state,
    ]
  `;
  assert.deepEqual([...extractRegistered(sample)].sort(), [
    'desktop_preferences',
    'player_get_state',
  ]);
});

test('ipc names stay aligned across rust, frontend, harness, and the contract lists', () => {
  const registered = new Set();
  for (const path of rustSources(join(root, 'src-tauri', 'src'))) {
    for (const name of extractRegistered(readFileSync(path, 'utf8'))) registered.add(name);
  }
  const frontend = extractInvokeNames(
    readFileSync(join(root, 'src', 'lib', 'services', 'music.ts'), 'utf8'),
  );
  const harness = extractHandlerKeys(
    readFileSync(join(root, 'tests', 'harness', 'mock-ipc.js'), 'utf8'),
  );
  const contract = readFileSync(join(root, 'docs', 'ipc-contract.md'), 'utf8');
  const documented = extractFence(contract, 'ipc-commands');
  const unused = extractFence(contract, 'ipc-unused');
  const unusedActual = diff(registered, frontend);

  assert.deepEqual(diff(frontend, registered), [], 'frontend command is not registered');
  assert.deepEqual(diff(frontend, harness), [], 'frontend command has no harness handler');
  assert.deepEqual(
    diff(documented, registered),
    [],
    'contract lists a command that is not registered',
  );
  assert.deepEqual(
    diff(registered, documented),
    [],
    'registered command is missing from the contract list',
  );
  assert.deepEqual(unusedActual, [...expectedUnused].sort());
  assert.deepEqual([...unused].sort(), [...expectedUnused].sort());
});
