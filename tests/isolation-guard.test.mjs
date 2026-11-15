import {spawnSync} from 'node:child_process';
import {mkdtempSync, mkdirSync, writeFileSync, rmSync, symlinkSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join, dirname} from 'node:path';
import {fileURLToPath} from 'node:url';
import {test} from 'node:test';
import assert from 'node:assert/strict';

const repo = dirname(dirname(fileURLToPath(import.meta.url)));
const cli = join(repo, 'scripts', 'isolation-cli.ps1');

function run(args) {
  return spawnSync('powershell.exe', ['-NoProfile', '-ExecutionPolicy', 'Bypass', '-File', cli, ...args], {
    encoding: 'utf8',
    cwd: repo,
  });
}

function status(args) {
  const result = run(args);
  assert.equal(result.error, undefined, String(result.error));
  return result;
}

test('path checks reject traversal, similar prefixes, and junctions', () => {
  const base = mkdtempSync(join(tmpdir(), 'mm-path-'));
  const corpus = join(base, 'mm-cover-corpus');
  const evil = join(base, 'mm-cover-corpus-evil');
  const outside = join(base, 'outside');
  mkdirSync(join(corpus, 'sub'), {recursive: true});
  mkdirSync(evil, {recursive: true});
  mkdirSync(outside, {recursive: true});
  writeFileSync(join(corpus, 'sub', 'file.txt'), 'inside');
  writeFileSync(join(evil, 'file.txt'), 'evil');
  writeFileSync(join(outside, 'file.txt'), 'outside');
  symlinkSync(outside, join(corpus, 'link'), 'junction');
  try {
    assert.equal(status(['-Action', 'test-path', '-Candidate', join(corpus, 'sub', 'file.txt'), '-Root', corpus]).status, 0);
    assert.equal(status(['-Action', 'test-path', '-Candidate', join(evil, 'file.txt'), '-Root', corpus]).status, 1);
    assert.equal(status(['-Action', 'test-path', '-Candidate', `${corpus}\\sub\\..\\..\\outside`, '-Root', corpus]).status, 1);
    assert.equal(status(['-Action', 'test-path', '-Candidate', join(corpus, 'link', 'file.txt'), '-Root', corpus]).status, 1);
    assert.equal(status(['-Action', 'test-delete', '-Candidate', join(corpus, 'sub'), '-Root', corpus]).status, 0);
    const blocked = status(['-Action', 'test-delete', '-Candidate', corpus, '-Root', corpus]);
    assert.equal(blocked.status, 1, blocked.stdout);
    assert.match(blocked.stdout, /reparse point/);
  } finally {
    rmSync(base, {recursive: true, force: true});
  }
});

test('descendant collection includes grandchildren and skips other trees', () => {
  const dir = mkdtempSync(join(tmpdir(), 'mm-proc-'));
  const table = join(dir, 'procs.json');
  writeFileSync(table, JSON.stringify([
    {ProcessId: 1, ParentProcessId: 0},
    {ProcessId: 2, ParentProcessId: 1},
    {ProcessId: 3, ParentProcessId: 2},
    {ProcessId: 4, ParentProcessId: 9},
  ]));
  try {
    const result = status(['-Action', 'descendants', '-Table', table, '-ProcessId', '1']);
    assert.equal(result.status, 0, result.stdout + result.stderr);
    assert.deepEqual(result.stdout.trim().split(/\r?\n/).filter(Boolean), ['2', '3']);
  } finally {
    rmSync(dir, {recursive: true, force: true});
  }
});

function fakeRepo() {
  const fake = mkdtempSync(join(tmpdir(), 'mm-src-'));
  const built = join(fake, 'src-tauri', 'target', 'debug');
  mkdirSync(built, {recursive: true});
  mkdirSync(join(fake, 'src-tauri'), {recursive: true});
  writeFileSync(join(fake, 'src-tauri', 'tauri.conf.json'), '{"productName":"Music Mine"}\n');
  return {fake, built};
}

test('launch source requires a hash-bound record and checks the two data-dir kinds separately', () => {
  const {fake, built} = fakeRepo();
  const exe = join(built, 'music-mine-desktop.exe');
  const other = join(built, 'other-desktop.exe');
  writeFileSync(exe, 'portable-bytes');
  writeFileSync(other, 'absolute-bytes');
  const portable = join(fake, 'src-tauri', 'tauri.portable.conf.json');
  writeFileSync(portable, '{"app":{"appDirectoriesOverride":"./app-data"}}\n');
  const testRoot = join(fake, 'empty-root');
  const data = join(testRoot, 'data');
  mkdirSync(data, {recursive: true});
  const outside = join(fake, 'outside');
  mkdirSync(outside, {recursive: true});
  const absoluteConfig = join(fake, 'override.json');
  writeFileSync(absoluteConfig, JSON.stringify({app: {appDirectoriesOverride: data}}));
  const escapeConfig = join(fake, 'escape.json');
  writeFileSync(escapeConfig, JSON.stringify({app: {appDirectoriesOverride: join(testRoot, 'data', '..', '..', 'outside')}}));
  try {
    const missing = status(['-Action', 'check-source', '-Repo', fake, '-Exe', exe]);
    assert.equal(missing.status, 1);
    assert.match(missing.stdout, /MISSING_RECORD/);

    const outsideExe = join(fake, 'stray.exe');
    writeFileSync(outsideExe, 'stray');
    const stray = status([
      '-Action', 'write-source', '-Repo', fake, '-BuiltExe', outsideExe,
      '-Config', 'src-tauri/tauri.portable.conf.json', '-Command', 'test',
      '-Kind', 'portable-relative', '-Expected', './app-data',
    ]);
    assert.equal(stray.status, 1, stray.stdout);

    const wrote = status([
      '-Action', 'write-source', '-Repo', fake, '-BuiltExe', exe,
      '-Config', 'src-tauri/tauri.portable.conf.json', '-Command', 'pnpm exec tauri build --config src-tauri/tauri.portable.conf.json',
      '-Kind', 'portable-relative', '-Expected', './app-data',
    ]);
    assert.equal(wrote.status, 0, wrote.stdout + wrote.stderr);
    assert.equal(status(['-Action', 'check-source', '-Repo', fake, '-Exe', exe]).status, 0);

    writeFileSync(portable, '{"app":{"appDirectoriesOverride":"C:\\\\somewhere"}}\n');
    const changed = status(['-Action', 'check-source', '-Repo', fake, '-Exe', exe]);
    assert.equal(changed.status, 1, changed.stdout);
    assert.match(changed.stdout, /CONFIG_HASH_MISMATCH|PORTABLE_OVERRIDE_NOT_RELATIVE/);

    writeFileSync(portable, '{"app":{"appDirectoriesOverride":"./app-data"}}\n');
    const absoluteOverride = status([
      '-Action', 'write-source', '-Repo', fake, '-BuiltExe', exe,
      '-Config', 'src-tauri/tauri.portable.conf.json', '-Command', 'test',
      '-Kind', 'portable-relative', '-Expected', './app-data',
    ]);
    assert.equal(absoluteOverride.status, 0, absoluteOverride.stdout + absoluteOverride.stderr);
    writeFileSync(portable, '{"app":{"appDirectoriesOverride":"C:\\\\abs"}}\n');
    const refusedKind = status([
      '-Action', 'write-source', '-Repo', fake, '-BuiltExe', other,
      '-Config', 'src-tauri/tauri.portable.conf.json', '-Command', 'test',
      '-Kind', 'portable-relative', '-Expected', './app-data',
    ]);
    assert.equal(refusedKind.status, 1, refusedKind.stdout);
    writeFileSync(portable, '{"app":{"appDirectoriesOverride":"./app-data"}}\n');

    const absolute = status([
      '-Action', 'write-source', '-Repo', fake, '-BuiltExe', other,
      '-Config', absoluteConfig, '-Command', 'pnpm exec tauri build --debug --no-bundle --config override.json',
      '-Kind', 'absolute-override', '-Expected', data, '-TestRoot', testRoot,
    ]);
    assert.equal(absolute.status, 0, absolute.stdout + absolute.stderr);
    assert.equal(status(['-Action', 'check-source', '-Repo', fake, '-Exe', other]).status, 0);

    const escaped = status([
      '-Action', 'write-source', '-Repo', fake, '-BuiltExe', other,
      '-Config', escapeConfig, '-Command', 'test',
      '-Kind', 'absolute-override', '-Expected', join(testRoot, 'data', '..', '..', 'outside'),
      '-TestRoot', testRoot,
    ]);
    assert.equal(escaped.status, 1, escaped.stdout);
    assert.match(escaped.stdout, /escapes the test root/);
  } finally {
    rmSync(fake, {recursive: true, force: true});
  }
});
