# Design System — Rift

## 1. Visual Theme & Atmosphere

Rift is a dark-mode-native database client built for developers who live in code editors. The aesthetic channels the precision of TablePlus, the minimalism of Linear, and the density of a terminal window. There is no light mode — Rift exists entirely in the void.

The interface is data-dense and snappy. Every pixel serves a purpose. Animations are subtle and fast (<150ms). The design philosophy is "terminal evolved" — deep charcoal backgrounds with cool cyan accents that signal "this is a tool for databases."

**Key Characteristics:**
- Dark-mode-only: deep charcoal backgrounds (`#0c0c0c`, `#141414`)
- Cyan accent (`#00d4ff`) used sparingly as identity marker
- Inter font for UI — geometric, clean, legible at small sizes
- JetBrains Mono for code/query — developer-native monospace
- Border-defined depth — no box-shadows
- Data-dense layouts with minimal whitespace
- Snappy interactions, no slow animations

## 2. Color Palette & Roles

### Backgrounds
| Token | Hex | Role |
|-------|-----|------|
| `--bg-base` | `#0c0c0c` | Deepest layer |
| `--bg-surface` | `#141414` | Primary panel background |
| `--bg-elevated` | `#1a1a1a` | Cards, inputs, elevated surfaces |
| `--bg-hover` | `#1f1f1f` | Hover state |
| `--bg-active` | `#262626` | Active/selected state |
| `--bg-overlay` | `rgba(0, 0, 0, 0.7)` | Modal backdrop |

### Text
| Token | Hex | Role |
|-------|-----|------|
| `--text-primary` | `#e8e8e8` | Headings, primary labels |
| `--text-secondary` | `#a0a0a0` | Body text, descriptions |
| `--text-tertiary` | `#6b6b6b` | Metadata, timestamps, disabled |
| `--text-accent` | `#00d4ff` | Links, active states |
| `--text-success` | `#22c55e` | Success messages |
| `--text-warning` | `#f59e0b` | Warnings |
| `--text-error` | `#ef4444` | Errors |

### Borders
| Token | Hex | Role |
|-------|-----|------|
| `--border-subtle` | `#1e1e1e` | Barely visible dividers |
| `--border-default` | `#2a2a2a` | Standard borders |
| `--border-prominent` | `#333333` | Active/focused borders |
| `--border-accent` | `rgba(0, 212, 255, 0.3)` | Accent highlight borders |

### Data Type Colors (Result Grid)
| Type | Color |
|------|-------|
| String | `#a5d6ff` |
| Number | `#f9cc6c` |
| Boolean | `#ff7b72` |
| Null | `#6b6b6b` |
| JSON | `#d2a8ff` |
| Timestamp | `#7ee787` |

## 3. Typography Rules

### Font Families
- **Primary UI**: `Inter`, fallbacks: `-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto`
- **Monospace (code/query)**: `JetBrains Mono`, fallbacks: `"Fira Code", "Source Code Pro", Menlo, Monaco`

### Hierarchy
| Role | Font | Size | Weight | Line Height |
|------|------|------|--------|-------------|
| App Title | Inter | 14px | 600 | 1.2 |
| Panel Header | Inter | 13px | 500 | 1.2 |
| Section Label | Inter | 11px | 500 | 1.2 |
| Body | Inter | 13px | 400 | 1.4 |
| Tab Label | Inter | 12px | 500 | 1.0 |
| Button | Inter | 12px | 500 | 1.0 |
| Table Header | Inter | 11px | 500 | 1.0 |
| Table Cell | Inter | 12px | 400 | 1.3 |
| Query Editor | JetBrains Mono | 13px | 400 | 1.5 |
| Code Snippet | JetBrains Mono | 12px | 400 | 1.4 |
| Status Line | JetBrains Mono | 11px | 400 | 1.2 |

### Principles
- Weight 400 default. Weight 500 only for labels, headers, buttons.
- No bold (700). Hierarchy from size and color.
- Monospace only for editor, code, status bar, data values.

## 4. Component Stylings

### Connection Card (Sidebar)
- Background: `--bg-surface`
- Border: `1px solid --border-default`
- Border radius: 6px
- Padding: 10px 12px
- Hover: bg → `--bg-hover`, border → `--border-prominent`
- Active/connected: left border `3px solid --accent-cyan`, bg → `--bg-active`

### Query Editor
- Background: `--bg-base`
- Font: JetBrains Mono 13px
- Line numbers: `--text-tertiary`, gutter 48px
- Active line: bg `--bg-hover`
- Selection: `rgba(0, 212, 255, 0.2)`
- Cursor: `--accent-cyan`, 2px width

### Result Grid
- Background: `--bg-base`
- Header: bg `--bg-surface`, text `--text-secondary`, border-bottom `1px solid --border-default`
- Row height: 32px
- Alternating rows: subtle `rgba(255,255,255,0.015)`
- Selected row: bg `rgba(0, 212, 255, 0.1)`
- Cell border: `1px solid --border-subtle`
- Cell padding: 8px 12px

### Tabs
- Background: `--bg-base`
- Active tab: bg `--bg-surface`, border-top `2px solid --accent-cyan`
- Inactive tab: transparent, text `--text-tertiary`
- Hover inactive: text `--text-secondary`

### Buttons
**Primary**
- Background: `--accent-cyan`
- Text: `#000000`
- Padding: 6px 16px
- Border radius: 4px
- Font: Inter 12px 500

**Secondary**
- Background: `--bg-elevated`
- Text: `--text-primary`
- Border: `1px solid --border-default`
- Padding: 6px 16px
- Border radius: 4px

### Modal / Dialog
- Backdrop: `--bg-overlay` + blur 8px
- Panel: `--bg-surface`, border `1px solid --border-prominent`, radius 8px
- Shadow: `0 8px 32px rgba(0,0,0,0.4)` (minimal, for overlay only)

## 5. Layout Principles

### App Shell (3-Panel)
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
- Width: 240px, fixed, collapsible to icon-only (48px)
- Sections: Connections, Schema (collapsible), Query History

### Status Bar
- Background: `--bg-base`
- Border-top: `1px solid --border-subtle`
- Left: connection name, database name
- Center: query execution time, row count
- Right: status indicator (idle/running/error)

### Spacing Scale
- Base unit: 4px
- Scale: 4, 8, 12, 16, 20, 24, 32, 48, 64

## 6. Depth & Elevation

No box-shadows for depth. All depth from border contrast.

| Level | Treatment |
|-------|-----------|
| Base | `--bg-base`, no border |
| Surface | `--bg-surface`, border `--border-default` |
| Elevated | `--bg-elevated`, border `--border-prominent` |
| Active | `--bg-active`, border `--border-accent` |
| Overlay | `--bg-overlay` + backdrop blur |

## 7. Do's and Don'ts

### Do
- Use dark mode exclusively — no light mode
- Apply cyan accent sparingly — identity marker, not decoration
- Create depth through border contrast (`#1e1e1e` → `#2a2a2a` → `#333333`)
- Keep interactions snappy (<150ms)
- Virtualize grid for large result sets
- Keyboard shortcuts everywhere

### Don't
- Don't add box-shadows — invisible on dark, break border system
- Don't use bold (700) text weight
- Don't load all rows to memory — always virtualize
- Don't clutter UI — every element needs purpose
- Don't block UI during query execution

## 8. Responsive Behavior

Rift is a desktop app. The layout is fixed for desktop screens (minimum 1024x768). Sidebar dapat di-collapse. Editor dan result panel dapat di-resize vertically.

## 9. Agent Prompt Guide

### Quick Color Reference
- Background: `#0c0c0c`, `#141414`, `#1a1a1a`
- Text: `#e8e8e8`, `#a0a0a0`, `#6b6b6b`
- Accent: `#00d4ff`
- Borders: `#1e1e1e`, `#2a2a2a`, `#333333`

### Example Prompts
- "Create a connection card: `#141414` background, `1px solid #2a2a2a` border, 6px radius. Title Inter 13px 500 `#e8e8e8`. Active state: left border 3px `#00d4ff`."
- "Design a result grid: `#0c0c0c` background. Header `#141414` with `#a0a0a0` text. Rows 32px. Cell border `1px solid #1e1e1e`. Selected row `rgba(0, 212, 255, 0.1)`."
- "Build a tab bar: `#0c0c0c` background. Active tab: `#141414` bg, top border 2px `#00d4ff`. Inactive: transparent, `#6b6b6b` text."
