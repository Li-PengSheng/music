/// <reference types="vite/client" />
// Development-only counters: never store song IDs, URLs, or local paths.
export const diagnostics = import.meta.env.DEV
  ? {
      snapshots: 0,
      trackChanges: 0,
      coverRequests: 0,
      urlChanges: 0,
      mounts: 0,
      loads: 0,
      errors: 0,
      startedAt: performance.now(),
    }
  : null;
if (diagnostics) Object.assign(globalThis, { __musicMineDiagnostics: diagnostics });
