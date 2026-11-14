# Music Mine development

- Follow docs/RFC-v2.0.md. Stack: Rust, Tauri v2, Svelte 5 Runes, TypeScript, Vite, Tailwind v4.
- Only change this project. Never open a real library database for migration experiments.
- Frontend IPC belongs in src/lib/services/music.ts; components must not scatter invoke calls.
- Playback state comes from Rust snapshots. Never simulate IPC success or advance playback with UI timers.
- No AI integrations in V1.0. No decoder reimplementation.
- Preserve the old project. Extract audio code only in a separately authorized migration task.
- Check pnpm install, pnpm check, pnpm test, pnpm test:ui, pnpm build, cargo fmt, cargo clippy, cargo test and an actual Windows window plus IPC before declaring a build validated.
- `pnpm test:ui` drives tests/harness against an in-memory library. Baseline screenshots in tests/ui/snapshots are Windows Chromium captures of that harness.
- Desktop startup tests must use the built executable, not a persistent development server and not the user's library database. On Windows, APPDATA does not redirect Tauri's data directory. Isolate with a temporary app.appDirectoriesOverride in tauri.conf.json, then remove it and rebuild before committing.
- after any modification, finish git commit