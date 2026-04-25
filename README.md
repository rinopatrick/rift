<div align="center">

<img src="https://raw.githubusercontent.com/rinopatrick/rift/main/src-tauri/icons/icon.png" width="120" alt="Rift" />

# Rift

**The database client that should have been open source.**

A modern, dark-native PostgreSQL client. Built with Rust + Tauri. Single binary. Free forever.

[![GitHub stars](https://img.shields.io/github/stars/rinopatrick/rift?style=flat-square&logo=github&color=00d4ff)](https://github.com/rinopatrick/rift/stargazers)
[![License](https://img.shields.io/github/license/rinopatrick/rift?style=flat-square&color=00d4ff)](LICENSE)
[![Release](https://img.shields.io/github/v/release/rinopatrick/rift?style=flat-square&color=00d4ff&logo=github)](https://github.com/rinopatrick/rift/releases)
[![Downloads](https://img.shields.io/github/downloads/rinopatrick/rift/total?style=flat-square&color=00d4ff)](https://github.com/rinopatrick/rift/releases)
[![Website](https://img.shields.io/badge/Website-rinopatrick.github.io/rift-00d4ff?style=flat-square&logo=github)](https://rinopatrick.github.io/rift/)

[Website](https://rinopatrick.github.io/rift/) · [Download](https://github.com/rinopatrick/rift/releases) · [Roadmap](#roadmap) · [Contributing](#contributing) · [Sponsor](#sponsor)

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
- [x] **SQL autocomplete** — schema-aware suggestions from CodeMirror 6
- [x] **Query bookmarks** — save and organize favorite queries
- [x] **Query cancellation** — abort long-running queries
- [x] **Result charts** — bar and line charts from query results
- [x] **CSV import** — drag and drop CSV to create tables
- [x] **Inline cell editing** — double-click to edit with parameterized updates
- [x] **EXPLAIN plan visualizer** — interactive tree diagram for PostgreSQL
- [x] **ER Diagram** — auto-generated SVG diagram from schema
- [x] **Settings panel** — font size, theme, keybinding cheat sheet
- [x] **MySQL support** — full MySQL driver with schema exploration
- [x] **Connection groups** — organize connections into folders
- [x] **SSH tunnel** — connect through bastion hosts
- [x] **Query profiler** — performance analysis with timing breakdown
- [ ] **Multi-resultset support** — multiple SELECT results
- [ ] **Database migration diff view** — compare schemas

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
- [x] Monaco Editor integration (syntax highlighting)
- [x] Auto-complete from schema introspection
- [x] Query bookmarks / favorites
- [x] Keyboard shortcut customization
- [x] Query cancellation
- [x] Result charts
- [x] CSV import

### v0.3 — Power User
- [x] EXPLAIN plan visualizer
- [x] ER Diagram
- [x] Settings panel
- [x] Inline cell editing
- [x] Connection groups / folders
- [x] Query performance profiler
- [ ] Multi-resultset support

### v0.4 — Multi-Database
- [x] SQLite driver
- [x] MySQL driver
- [x] SSH tunnel support
- [ ] Database migration diff view

### v0.5 — Performance & Polish
- [x] Query profiler with bottleneck detection
- [ ] Multi-resultset support
- [ ] Database migration diff view
- [ ] Connection timeout / SSL error human-readable messages

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
