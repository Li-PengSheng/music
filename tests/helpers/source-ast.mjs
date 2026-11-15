// Pull a function out of Svelte or TypeScript source so a test can run it.
// Whitespace and line breaks are allowed. This is a bounded scan, not a parser.

export function extractFunction(source, name, nextName) {
  const start = source.search(new RegExp(`function\\s+${name}\\s*\\(`));
  const end = source.search(new RegExp(`(?:async\\s+)?function\\s+${nextName}\\s*\\(`));
  if (start < 0 || end <= start) {
    throw new Error(`failed to extract function ${name}`);
  }
  return source
    .slice(start, end)
    .replace(new RegExp(`function\\s+${name}\\s*\\([^)]*\\)`), `function ${name}(e)`)
    .replace(/\s+as\s+[A-Za-z0-9_]+/g, '');
}
