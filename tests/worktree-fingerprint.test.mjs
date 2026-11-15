import {spawnSync} from 'node:child_process';
import {mkdtempSync, writeFileSync, rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join, dirname} from 'node:path';
import {fileURLToPath} from 'node:url';
import {test} from 'node:test';
import assert from 'node:assert/strict';

const script = join(dirname(dirname(fileURLToPath(import.meta.url))), 'scripts', 'worktree-fingerprint.ps1');

function git(repo, args) {
  const result = spawnSync('git', args, {cwd: repo, encoding: 'utf8'});
  assert.equal(result.status, 0, result.stderr || result.stdout);
}

function fingerprint(repo, extra = []) {
  const result = spawnSync('powershell.exe', ['-NoProfile', '-ExecutionPolicy', 'Bypass', '-File', script, '-Repo', repo, ...extra], {encoding: 'utf8'});
  assert.equal(result.status, 0, result.stdout + result.stderr);
  return result.stdout.trim();
}

test('worktree fingerprint covers staged, unstaged, and included untracked files', () => {
  const repo = mkdtempSync(join(tmpdir(), 'mm-fp-'));
  try {
    git(repo, ['init']);
    git(repo, ['config', 'user.email', 'fingerprint@example.com']);
    git(repo, ['config', 'user.name', 'fingerprint']);
    git(repo, ['config', 'core.autocrlf', 'false']);
    writeFileSync(join(repo, '.gitignore'), '*.log\n');
    writeFileSync(join(repo, 'file.txt'), 'one\n');
    git(repo, ['add', '.gitignore', 'file.txt']);
    git(repo, ['commit', '-m', 'init']);
    const clean = fingerprint(repo);
    assert.equal(fingerprint(repo), clean);

    writeFileSync(join(repo, 'file.txt'), 'two\n');
    const unstaged = fingerprint(repo);
    assert.notEqual(unstaged, clean);
    git(repo, ['add', 'file.txt']);
    const staged = fingerprint(repo);
    assert.notEqual(staged, unstaged);

    writeFileSync(join(repo, 'noise.log'), 'ignored\n');
    assert.equal(fingerprint(repo), staged);
    writeFileSync(join(repo, 'keep.txt'), 'keep\n');
    const withKeep = fingerprint(repo);
    assert.notEqual(withKeep, staged);
    assert.equal(fingerprint(repo, ['-Exclude', 'keep.txt']), staged);
  } finally {
    rmSync(repo, {recursive: true, force: true});
  }
});
