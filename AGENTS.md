# Music Mine development

- Follow docs/RFC-v2.0.md. Stack: Rust, Tauri v2, Svelte 5 Runes, TypeScript, Vite, Tailwind v4.
- Only change this project. Never open a real library database for migration experiments.
- Frontend IPC belongs in src/lib/services/music.ts; components must not scatter invoke calls.
- Demo albums are not playable tracks. Never simulate IPC success or advance playback with UI timers.
- No AI integrations in V1.0. No decoder reimplementation.
- Preserve the old project. Extract audio code only in a separately authorized migration task.
- Check pnpm install, pnpm check, pnpm build, cargo fmt, cargo clippy, cargo test and an actual Windows window plus IPC before declaring the foundation validated.
- Desktop startup tests must use the built executable, not a persistent development server in Craft Anything.
