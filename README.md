# Rift

> The database client that should have been open source.

A modern, dark-native PostgreSQL client built with Tauri, Svelte 5, and Rust.

## Why Rift?

Every developer needs a database client. The options today:
- **TablePlus** — beautiful, fast, proprietary ($79)
- **DBeaver** — open source, outdated UI, bloated
- **pgAdmin** — khusus PostgreSQL, UI lebih jadul lagi
- **Command line** — cepat tapi nggak visual

**Rift** is the open source alternative that doesn't compromise. Modern UI, native performance, single binary.

## Features

- [x] Connect to PostgreSQL
- [x] Query editor with keyboard shortcuts
- [x] Result grid with data type formatting
- [x] Multi-tab query interface
- [x] Connection manager with persistent storage
- [x] Dark-native UI
- [ ] Schema explorer (v0.2)
- [ ] Query history (v0.2)
- [ ] Export CSV/JSON/SQL (v0.2)
- [ ] Auto-complete (v0.3)
- [ ] SQLite / MySQL support (future)

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Desktop Framework | Tauri v2 |
| Frontend | Svelte 5 + Tailwind CSS |
| Backend | Rust + tokio |
| Database Driver | tokio-postgres + deadpool |
| Local Storage | rusqlite (embedded SQLite) |

## Development

```bash
# Install dependencies
npm install

# Run in dev mode
npm run tauri dev

# Build for production
npm run tauri build
```

## License

MIT
