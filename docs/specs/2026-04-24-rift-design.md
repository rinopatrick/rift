# Rift — Design Specification

## Project Identity

**Name**: Rift
**Tagline**: "The database client that should have been open source."
**Domain**: Open-source, cross-platform database client (PostgreSQL-first)
**Vibe**: Dark-native developer tool. Fast, precise, minimal. Like TablePlus met Linear.

---

## 1. Problem & Opportunity

### The Problem
Every developer needs a database client. The options:
- **TablePlus** — beautiful, fast, proprietary ($79)
- **DBeaver** — open source, UI jadul, bloated
- **pgAdmin** — khusus PostgreSQL, UI lebih jadul lagi
- **Command line** (`psql`) — cepat tapi nggak visual

Tidak ada database client yang: modern + open source + cross-platform + native feel.

### The Opportunity
Build PostgreSQL client yang:
- Experience mendekati TablePlus (fast, beautiful, intuitive)
- 100% open source (MIT license)
- Single binary download (~5MB via Tauri)
- Dark-native design yang bikin dev nyaman

### Target User
Backend developer, data engineer, full-stack developer yang:
- Kerja dengan PostgreSQL secara regular
- Frustrated dengan DBeaver/pgAdmin
- Nggak mau bayar TablePlus atau prefer open source
- Hargai kecepatan dan aesthetics

---

## 2. Visual Theme & Atmosphere

Rift adalah dark-mode-native developer tool yang terinspirasi dari TablePlus (precision) dan Linear (minimalism). Tidak ada light mode — Rift lahir di dark dan tetap di dark.

**Atmosfer:**
- Background dalam kayak void — nggak pure black tapi deep charcoal
- Accent color cool — cyan/teal yang terasa "database tool" (bukan emerald, bukan purple)
- Data density tinggi — tiap pixel dimanfaatkan, tapi nggak cluttered
- Animasi subtle dan cepat — nggak ada yang lambat, transition <150ms

**Key Characteristics:**
- Dark-mode-only: deep charcoal backgrounds (`#0c0c0c`, `#141414`)
- Cyan accent (`#00d4ff`, `#0ea5e9`) — database-y, cool, precise
- Inter font — geometric, clean, highly legible at small sizes
- JetBrains Mono untuk code/query — developer-native
- Border-defined depth — nggak pakai shadow
- Data-dense layouts — tiap panel punya purpose

---

## 3. Color Palette & Roles

### Backgrounds
| Token | Hex | Role |
|-------|-----|------|
| `--bg-base` | `#0c0c0c` | Deepest layer, behind everything |
| `--bg-surface` | `#141414` | Primary panel background |
| `--bg-elevated` | `#1a1a1a` | Cards, input fields, elevated surfaces |
| `--bg-hover` | `#1f1f1f` | Hover state untuk rows/items |
| `--bg-active` | `#262626` | Active/selected state |
| `--bg-overlay` | `rgba(0, 0, 0, 0.7)` | Modal backdrop |

### Text
| Token | Hex | Role |
|-------|-----|------|
| `--text-primary` | `#e8e8e8` | Headings, primary labels |
| `--text-secondary` | `#a0a0a0` | Body text, descriptions |
| `--text-tertiary` | `#6b6b6b` | Metadata, timestamps, disabled |
| `--text-accent` | `#00d4ff` | Links, active states, brand |
| `--text-success` | `#22c55e` | Success messages, healthy status |
| `--text-warning` | `#f59e0b` | Warnings |
| `--text-error` | `#ef4444` | Errors, failed queries |

### Borders
| Token | Hex | Role |
|-------|-----|------|
| `--border-subtle` | `#1e1e1e` | Barely visible dividers |
| `--border-default` | `#2a2a2a` | Standard borders |
| `--border-prominent` | `#333333` | Active/focused borders |
| `--border-accent` | `rgba(0, 212, 255, 0.3)` | Accent highlight borders |

### Accent Spectrum
| Token | Hex | Role |
|-------|-----|------|
| `--accent-cyan` | `#00d4ff` | Primary brand accent |
| `--accent-blue` | `#0ea5e9` | Secondary accent, links |
| `--accent-cyan-dim` | `rgba(0, 212, 255, 0.15)` | Subtle cyan wash |
| `--accent-cyan-glow` | `rgba(0, 212, 255, 0.4)` | Focus glow |

### Data Type Colors (Result Grid)
| Type | Color | Example |
|------|-------|---------|
| String | `#a5d6ff` | `'hello'` |
| Number | `#f9cc6c` | `42` |
| Boolean | `#ff7b72` | `true` |
| Null | `#6b6b6b` | `NULL` |
| JSON | `#d2a8ff` | `{"key": "val"}` |
| Timestamp | `#7ee787` | `2024-01-15` |

---

## 4. Typography Rules

### Font Families
- **Primary UI**: `Inter`, fallbacks: `-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto`
- **Monospace (code/query)**: `JetBrains Mono`, fallbacks: `"Fira Code", "Source Code Pro", Menlo, Monaco`

### Hierarchy
| Role | Font | Size | Weight | Line Height | Letter Spacing |
|------|------|------|--------|-------------|----------------|
| App Title | Inter | 14px | 600 | 1.2 | -0.01em |
| Panel Header | Inter | 13px | 500 | 1.2 | 0 |
| Section Label | Inter | 11px | 500 | 1.2 | 0.05em |
| Body | Inter | 13px | 400 | 1.4 | 0 |
| Tab Label | Inter | 12px | 500 | 1.0 | 0 |
| Button | Inter | 12px | 500 | 1.0 | 0.01em |
| Table Header | Inter | 11px | 500 | 1.0 | 0.03em |
| Table Cell | Inter | 12px | 400 | 1.3 | 0 |
| Query Editor | JetBrains Mono | 13px | 400 | 1.5 | 0 |
| Code Snippet | JetBrains Mono | 12px | 400 | 1.4 | 0 |
| Status Line | JetBrains Mono | 11px | 400 | 1.2 | 0 |

### Principles
- Weight 400 untuk semua body text. Weight 500 hanya untuk labels, headers, buttons.
- Nggak ada bold (700) — hierarchy dari size dan color, bukan weight.
- Monospace hanya untuk: query editor, code snippets, status bar, data values di grid.

---

## 5. Component Stylings

### Connection Card (Sidebar)
- Background: `--bg-surface`
- Border: `1px solid --border-default`
- Border radius: 6px
- Padding: 10px 12px
- Hover: background → `--bg-hover`, border → `--border-prominent`
- Active/connected: left border `3px solid --accent-cyan`, background → `--bg-active`
- Contains: connection name (13px 500), host:port (11px 400, --text-tertiary), status dot

### Query Editor
- Background: `--bg-base`
- Font: JetBrains Mono 13px
- Line numbers: `--text-tertiary`, gutter width 48px
- Active line: background `--bg-hover`
- Selection: `rgba(0, 212, 255, 0.2)`
- Cursor: `--accent-cyan`, width 2px
- Syntax highlighting SQL (custom theme, lihat Color Palette)
- Auto-complete dropdown: `--bg-elevated`, border `--border-prominent`, shadow minimal

### Result Grid
- Background: `--bg-base`
- Header: background `--bg-surface`, text `--text-secondary`, border-bottom `1px solid --border-default`
- Row height: 32px
- Alternating rows: subtle `rgba(255,255,255,0.015)`
- Selected row: background `rgba(0, 212, 255, 0.1)`
- Cell border: `1px solid --border-subtle`
- Cell padding: 8px 12px
- Horizontal scroll: smooth, column resize handles
- Empty state: centered icon + "Run a query to see results"

### Schema Tree (Sidebar Panel)
- Indent: 16px per level
- Folder icon + label untuk schemas/tables
- Column icon + name + type untuk columns
- Primary key: key icon, cyan accent
- Foreign key: link icon, blue accent
- Index: lightning icon, muted
- Expanded: chevron rotated 90°, children visible
- Collapsed: chevron right

### Tabs
- Background: `--bg-base`
- Active tab: background `--bg-surface`, border-top `2px solid --accent-cyan`
- Inactive tab: background transparent, text `--text-tertiary`
- Hover inactive: text `--text-secondary`
- Close button: × icon, 10px, muncul on hover
- Tab overflow: scrollable with fade indicator

### Buttons
**Primary**
- Background: `--accent-cyan`
- Text: `#000000`
- Padding: 6px 16px
- Border radius: 4px
- Font: Inter 12px 500
- Hover: brightness 1.1
- Active: brightness 0.95

**Secondary**
- Background: `--bg-elevated`
- Text: `--text-primary`
- Border: `1px solid --border-default`
- Padding: 6px 16px
- Border radius: 4px
- Hover: border → `--border-prominent`, bg → `--bg-hover`

**Ghost/Icon**
- Background: transparent
- Text: `--text-secondary`
- Padding: 4px
- Border radius: 4px
- Hover: bg → `--bg-hover`, text → `--text-primary`

### Modal / Dialog
- Backdrop: `--bg-overlay` + blur 8px
- Panel: `--bg-surface`, border `1px solid --border-prominent`, radius 8px
- Shadow: minimal, `0 8px 32px rgba(0,0,0,0.4)`
- Width: max 480px, centered
- Header: title 14px 600, close button top-right
- Body: padding 20px
- Footer: action buttons right-aligned

### Toast Notification
- Background: `--bg-elevated`
- Border: `1px solid --border-default`
- Border-left: `3px solid` (color sesuai type)
- Success: left border `--text-success`
- Error: left border `--text-error`
- Info: left border `--accent-cyan`
- Position: bottom-right, stacked
- Auto-dismiss: 4 detik

---

## 6. Layout Principles

### App Shell (3-Panel Layout)
```
+----------------------------------------------------------+
|  [Sidebar]  |  [Main Content Area]                      |
|  240px      |                                             |
|             |  +------------------------------------+    |
|  Connections|  | Query Editor (resizable)            |    |
|  Schema     |  | ~40% height                         |    |
|  History    |  +------------------------------------+    |
|             |  | Tab Bar: Results | Structure |...    |    |
|             |  +------------------------------------+    |
|             |  | Result Grid / Panel                 |    |
|             |  | ~60% height                         |    |
|             |  +------------------------------------+    |
|             |  | Status Bar: rows | time | conn       |    |
|             |  +------------------------------------+    |
+----------------------------------------------------------+
```

### Sidebar
- Width: 240px, fixed
- Sections: Connections, Schema (collapsible), Query History
- Resizable: bisa di-collapse ke icon-only (48px)
- Divider: `1px solid --border-default`, draggable

### Main Content
- Query Editor di atas (resizable height)
- Tab bar untuk switch antara: Results, Structure, Indexes, etc
- Result Grid di bawah
- Status bar paling bawah (24px height)

### Status Bar
- Background: `--bg-base`
- Border-top: `1px solid --border-subtle`
- Left: connection name, database name
- Center: query execution time, row count
- Right: status indicator (idle/running/error)

### Spacing Scale
- Base unit: 4px
- Scale: 4, 8, 12, 16, 20, 24, 32, 48, 64

---

## 7. Depth & Elevation

Rift tidak pakai box-shadow untuk depth. Semua depth dari border contrast.

| Level | Treatment |
|-------|-----------|
| Base | `--bg-base`, no border |
| Surface | `--bg-surface`, border `--border-default` |
| Elevated | `--bg-elevated`, border `--border-prominent` |
| Active | `--bg-active`, border `--border-accent` |
| Overlay | `--bg-overlay` + backdrop blur |

---

## 8. Interactions & Animations

### Timing
- Micro-interactions: 100ms (hover, focus)
- Panel transitions: 150ms (expand/collapse)
- Modal: 200ms (fade + slight scale)
- Toast: 300ms (slide in/out)

### Easing
- Standard: `cubic-bezier(0.4, 0, 0.2, 1)`
- Enter: `cubic-bezier(0, 0, 0.2, 1)`
- Exit: `cubic-bezier(0.4, 0, 1, 1)`

### Specific Behaviors
- **Query execution**: spinner cyan di status bar, editor tab indicator pulsing
- **Result loading**: skeleton rows (3-5 bar) dengan shimmer subtle
- **Row selection**: click → instant highlight, cmd/ctrl+click multi-select, shift range select
- **Column resize**: drag handle muncul on hover, real-time resize
- **Schema tree expand**: instant (no animation — dev tools harus snappy)

---

## 9. Key Screens

### Screen 1: Empty State (No Connection)
- Centered illustration/icon (database + sparkle)
- Headline: "Connect to your database"
- Subtext: "Add a PostgreSQL connection to get started"
- Primary button: "New Connection"
- Secondary link: "Import from URL"

### Screen 2: Connected + Query
- Sidebar: active connection highlighted, schema tree expanded
- Editor: empty atau query terakhir
- Results: empty atau hasil query terakhir
- Status bar: connected, idle

### Screen 3: Query Running
- Editor: active line highlighted
- Results: skeleton loading state
- Status bar: spinner, "Executing...", timer running
- Cancel button muncul di status bar

### Screen 4: Query Complete with Results
- Results grid populated
- Status bar: "12,340 rows in 1.24s"
- Pagination jika >1000 rows (virtualized scroll)
- Export buttons active (CSV, JSON, SQL)

### Screen 5: Connection Modal
- Form fields: Name, Host, Port, Database, User, Password
- SSL toggle
- Test Connection button
- Save button

---

## 10. Architecture Overview

```
Tauri App (single binary, ~5MB)
├── Frontend: Svelte 5 + Tailwind CSS
│   ├── App Shell (layout, sidebar, panels)
│   ├── ConnectionManager (CRUD, test, SSL)
│   ├── SchemaExplorer (tree, lazy load)
│   ├── QueryEditor (Monaco/CodeMirror)
│   ├── ResultGrid (TanStack Table, virtualized)
│   ├── QueryHistory (search, favorite, rerun)
│   └── ExportEngine (CSV, JSON, SQL)
├── IPC Bridge (Tauri commands)
└── Backend: Rust
    ├── Connection pool (deadpool-postgres)
    ├── Query executor (tokio-postgres, async, cancelable)
    ├── Schema introspection (information_schema)
    ├── Result serializer (streaming → JSON)
    └── Local state (rusqlite: connections, history, favorites)
```

### Extensibility
Rust backend menggunakan trait-based architecture:
```rust
trait DatabaseDriver {
    async fn connect(&self, config: ConnectionConfig) -> Result<Pool, Error>;
    async fn execute(&self, pool: &Pool, sql: &str) -> Result<ResultSet, Error>;
    async fn introspect_schema(&self, pool: &Pool) -> Result<Schema, Error>;
}
```
PostgreSQL implement trait ini. SQLite/MySQL/etc = implement baru tanpa touch frontend.

---

## 11. Tech Stack

| Layer | Technology | Reason |
|-------|-----------|--------|
| Desktop Framework | Tauri v2 | Native binary, small size (~5MB), Rust backend, cross-platform |
| Frontend Framework | Svelte 5 | Minimal bundle, reactive, fast renders for data grid |
| Styling | Tailwind CSS | Utility-first, dark mode easy, consistent spacing |
| Query Editor | Monaco Editor | SQL highlighting, autocomplete, minimap, familiar |
| Result Grid | Custom virtualized + TanStack | Handle 100k+ rows smoothly |
| Backend | Rust + tokio | Async, memory-safe, fast, small binary |
| PostgreSQL Driver | tokio-postgres + deadpool | Async, connection pooling |
| Local Storage | rusqlite | Zero-config, embedded, SQL for metadata |
| Icons | Lucide | Clean, consistent, tree-shakeable |
| State Management | Svelte 5 runes | Built-in, nggak perlu library tambahan |

---

## 12. Do's and Don'ts

### Do
- Gunakan dark mode exclusively — nggak ada light mode
- Cyan accent sparingly — untuk active states, links, primary actions
- Depth dari border contrast, bukan shadow
- Snappy interactions — nggak ada animasi lambat
- Virtualized grid untuk large result sets
- Keyboard shortcuts di mana-mana (Cmd+Enter run query, Cmd+T new tab, dll)
- Stream query results — jangan tunggu semua row

### Don't
- Jangan pakai light mode atau auto-detect OS theme
- Jangan pakai box-shadow untuk depth
- Jangan load semua row ke memory — virtualize selalu
- Jangan buat UI cluttered — tiap element harus punya purpose
- Jangan pakai font weight >500
- Jangan blocking UI saat query running — selalu bisa cancel
- Jangan request unnecessary permissions (Tauri allowlist minimal)

---

## 13. Agent Prompt Guide

### Quick Color Reference
- Background: `#0c0c0c` (deepest), `#141414` (surface), `#1a1a1a` (elevated)
- Text: `#e8e8e8` (primary), `#a0a0a0` (secondary), `#6b6b6b` (tertiary)
- Accent: `#00d4ff` (cyan), `#0ea5e9` (blue)
- Borders: `#1e1e1e` (subtle), `#2a2a2a` (default), `#333333` (prominent)

### Example Prompts
- "Create a connection card: `#141414` background, `1px solid #2a2a2a` border, 6px radius. Title Inter 13px 500 `#e8e8e8`. Host Inter 11px 400 `#6b6b6b`. Active state: left border 3px `#00d4ff`."
- "Design a result grid: `#0c0c0c` background. Header `#141414` with `#a0a8e8` text 11px 500. Rows 32px height. Cell border `1px solid #1e1e1e`. Selected row `rgba(0, 212, 255, 0.1)`."
- "Build a tab bar: `#0c0c0c` background. Active tab: `#141414` bg, top border 2px `#00d4ff`. Inactive: transparent, `#6b6b6b` text. Hover inactive: `#a0a0a0` text."
- "Create a status bar: `#0c0c0c` background, top border `1px solid #1e1e1e`. Left: connection name JetBrains Mono 11px `#a0a0a0`. Center: "12,340 rows in 1.24s" `#e8e8e8`. Right: cyan dot + 'Connected'."

---

## 14. Success Criteria

### MVP (v0.1)
- [ ] Connect ke PostgreSQL via connection string/form
- [ ] Schema explorer (schemas → tables → columns)
- [ ] Query editor dengan syntax highlighting
- [ ] Execute query dan tampilkan result grid
- [ ] Query history (per session)
- [ ] Export result ke CSV
- [ ] Dark theme konsisten
- [ ] Single binary build (Windows, macOS, Linux)

### v0.2
- [ ] Connection manager (save/load/edit connections)
- [ ] Query history persisten (local SQLite)
- [ ] Favorite queries
- [ ] Result filter dan sort client-side
- [ ] Export JSON dan SQL INSERT
- [ ] Table structure viewer (DDL)

### v0.3
- [ ] Multi-tab query editor
- [ ] Auto-complete berdasarkan schema
- [ ] Query execution plan (EXPLAIN)
- [ ] Keyboard shortcuts comprehensive
- [ ] Import CSV ke table

### Viral Checklist
- [ ] README dengan GIF demo <10 detik
- [ ] Landing page static (GitHub Pages)
- [ ] Auto-detect OS download button
- [ ] GitHub releases dengan 3 binary
- [ ] Hacker News launch ready

---

*Spec version: 1.0*
*Date: 2026-04-24*
*Status: Draft — pending review*
