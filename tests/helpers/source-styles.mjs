import { readdirSync, readFileSync } from 'node:fs';
import { join } from 'node:path';

/** Read actual CSS, including Svelte style blocks, independently of where a rule lives. */
export function sourceStyles({ excludeTokens = false } = {}) {
  function walk(dir) {
    return readdirSync(dir, { withFileTypes: true }).flatMap((entry) => {
      const path = join(dir, entry.name);
      if (entry.isDirectory()) return walk(path);
      if (excludeTokens && path.replaceAll('\\', '/') === 'src/lib/styles/tokens.css') return [];
      const source = readFileSync(path, 'utf8');
      if (path.endsWith('.css')) return [source];
      if (path.endsWith('.svelte'))
        return [...source.matchAll(/<style\b[^>]*>([\s\S]*?)<\/style>/g)].map((m) => m[1]);
      return [];
    });
  }
  return walk('src')
    .join('\n')
    .replace(/\/\*[\s\S]*?\*\//g, '');
}
