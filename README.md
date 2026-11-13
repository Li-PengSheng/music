# Music Mine Tauri

Rust + Tauri v2 + Svelte 5 + TypeScript + Vite + Tailwind CSS v4.

This is the task 02 foundation, with clearly marked demo albums and no audio playback or library access yet. Native connection status is shown only after a real Rust response.

Install Node and pnpm, then run `pnpm install`, `pnpm check`, `pnpm build`, and `pnpm tauri build --debug --no-bundle`. Open the generated Windows executable to test the native window and Settings → Verify connection. Browser preview cannot validate native IPC.

See [implementation and validation status](docs/INITIALIZATION.md), [RFC](docs/RFC-v2.0.md), and the supplied [migration audit](docs/MIGRATION-AUDIT.md). Dependency installation and desktop acceptance are currently blocked by registry access; this is not yet a verified runnable release.
