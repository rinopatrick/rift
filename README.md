<div align="center">

<img src="https://raw.githubusercontent.com/rinopatrick/rift/main/src-tauri/icons/icon.png" width="120" alt="Rift" />

# Rift

**The database client that should have been open source.**

A modern, dark-native PostgreSQL client. Built with Rust + Tauri. Single binary. Free forever.

[![GitHub stars](https://img.shields.io/github/stars/rinopatrick/rift?style=flat-square&logo=github&color=00d4ff)](https://github.com/rinopatrick/rift/stargazers)
[![License](https://img.shields.io/github/license/rinopatrick/rift?style=flat-square&color=00d4ff)](LICENSE)
[![Release](https://img.shields.io/github/v/release/rinopatrick/rift?style=flat-square&color=00d4ff&logo=github)](https://github.com/rinopatrick/rift/releases)
[![Downloads](https://img.shields.io/github/downloads/rinopatrick/rift/total?style=flat-square&color=00d4ff)](https://github.com/rinopatrick/rift/releases)

[Download](https://github.com/rinopatrick/rift/releases) · [Roadmap](#roadmap) · [Contributing](#contributing) · [Sponsor](#sponsor)

<img src="https://raw.githubusercontent.com/rinopatrick/rift/main/docs/screenshots/rift-hero.png" width="90%" alt="Rift Screenshot" />

</div>

---

## Why Rift?

Every developer needs a database client. The options today:

| Tool | Open Source | Modern UI | Native | Size |
|------|:-----------:|:---------:|:------:|:----:|
| **TablePlus** | No | Yes | Yes | ~40MB |
| **DBeaver** | Yes | No | Yes | ~300MB |
| **pgAdmin** | Yes | No | No | ~200MB |
| **Beekeeper** | Yes | Yes | No | ~150MB |
| **Rift** | **Yes** | **Yes** | **Yes** | **~5MB** |

Rift is the open source alternative that doesn't compromise.

- **Tiny binary** — ~5MB vs Electron apps at 150MB+
- **Truly native** — Rust backend, no bundled Chromium
- **Dark by default** — not an afterthought, it's the foundation
- **PostgreSQL-first** — deep PG support, extensible to MySQL/SQLite later

---

## Features

- [x] **PostgreSQL connections** — SSL, custom port, save credentials locally
- [x] **Schema explorer** — browse schemas, tables, columns, primary keys
- [x] **Query editor** — multi-tab, keyboard shortcuts (`Ctrl+Enter` to run)
- [x] **Result grid** — data type formatting, NULL handling, alternating rows
- [x] **Query history** — auto-saved per connection, searchable
- [x] **Export** — CSV, JSON, SQL INSERT statements
- [x] **Dark-native UI** — designed in the dark, not adapted to it
- [ ] **SQL autocomplete** — schema-aware suggestions
- [ ] **Query execution plan** — EXPLAIN visualizer
- [ ] **Table data editor** — inline cell editing
- [ ] **MySQL & SQLite** — multi-database support
- [ ] **Connection pooling** — persistent background pools

---

## Download

### macOS
```bash
# Homebrew (coming soon)
brew install --cask rift

# Or download DMG from releases
```

### Linux
```bash
# AppImage
wget https://github.com/rinopatrick/rift/releases/latest/download/rift-x86_64.AppImage
chmod +x rift-x86_64.AppImage
./rift-x86_64.AppImage
```

### Windows
```bash
# winget (coming soon)
winget install Rift

# Or download .msi from releases
```

> See all releases: [github.com/rinopatrick/rift/releases](https://github.com/rinopatrick/rift/releases)

---

## Development

```bash
# Clone
git clone https://github.com/rinopatrick/rift.git
cd rift

# Install dependencies
npm install

# Run in dev mode
npm run tauri dev

# Build for production
npm run tauri build
```

### Tech Stack

| Layer | Technology |
|-------|-----------|
| Desktop | Tauri v2 (Rust) |
| Frontend | Svelte 5 + Tailwind CSS |
| Database | tokio-postgres + deadpool |
| Storage | rusqlite (embedded) |

---

## Roadmap

### v0.2 — Polish & Productivity
- [ ] Monaco Editor integration (syntax highlighting)
- [ ] Auto-complete from schema introspection
- [ ] Query bookmarks / favorites
- [ ] Keyboard shortcut customization
- [ ] Table creation wizard

### v0.3 — Power User
- [ ] EXPLAIN plan visualizer
- [ ] Query performance profiler
- [ ] Data import (CSV → table)
- [ ] Connection groups / folders
- [ ] Multi-resultset support

### v0.4 — Multi-Database
- [ ] SQLite driver
- [ ] MySQL driver
- [ ] Database migration diff view

### Future
- [ ] Collaboration / shared connections
- [ ] Plugin system (WASM)
- [ ] Cloud sync for connections

---

## Contributing

We welcome contributions! Please read our [Contributing Guide](CONTRIBUTING.md) first.

```bash
# Fork, then:
git clone https://github.com/YOURNAME/rift.git
cd rift
git checkout -b feat/your-feature
```

- `good first issue` — beginner friendly
- `help wanted` — community contributions needed
- `bug` — confirmed issues

---

## Sponsor

Rift is free and open source. If it saves you time or money, consider supporting development:

[![Saweria](https://img.shields.io/badge/Support-Saweria-FF5722?style=flat-square&logo=wallet)](https://saweria.co/rinopatrick)
[![Ko-fi](https://img.shields.io/badge/Support-Ko--fi-FF5E5B?style=flat-square&logo=kofi)](https://ko-fi.com/rinopatrick)

Your support keeps the project alive and helps us ship faster.

---

## License

[MIT](LICENSE) — Patrick Rino

---

<div align="center">

Made with Rust and caffeine.

</div>
