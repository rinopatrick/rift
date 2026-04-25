# Rift — AGENTS.md

Modern open-source database client (TablePlus alternative). Built with Rust + Tauri v2 + Svelte 5.

## Stack
- **Backend**: Rust (tokio-postgres, deadpool, mysql_async, rusqlite)
- **Frontend**: Svelte 5 (runes: $state, $derived, $effect) + Tailwind CSS
- **Editor**: CodeMirror 6 with SQL autocomplete
- **Desktop**: Tauri v2

## Project Layout
- `src-tauri/src/` — Rust backend (commands, db drivers, state)
- `src/lib/` — Svelte frontend (components, stores, types)
- `docs/` — Landing page (GitHub Pages, no build step)

## Supported Databases
- PostgreSQL (full support including EXPLAIN JSON, ER diagram)
- SQLite (file-based)
- MySQL (via mysql_async)

## Key Files
- `src-tauri/src/commands.rs` — All Tauri commands
- `src-tauri/src/db/driver.rs` — DriverWrapper enum with execute/update_cell/get_schema
- `src-tauri/src/state/mod.rs` — SQLite state (connections, history, bookmarks, settings)
- `src/lib/components/` — Svelte UI components

## Dev Commands
```bash
# Frontend only
npm run dev

# Full Tauri (needs display server)
npm run tauri dev

# Build release
npm run tauri build
```

## Release Checklist
1. Update `src-tauri/Cargo.toml` version
2. Update `package.json` version
3. Update `src-tauri/tauri.conf.json` version
4. Update `docs/index.html` version badge + download button
5. Update `README.md` features/roadmap
6. `npm run build` + `cargo check`
7. `git commit` + `git tag vX.Y.Z` + `git push origin main --tags`

## Notes
- Landing page: https://rinopatrick.github.io/rift/
- GitHub releases auto-built via GitHub Actions
- WSL headless cannot run `cargo tauri dev` (no display server)
