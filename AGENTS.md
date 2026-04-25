# AGENTS.md

## Dev Commands

```bash
# Frontend (React + Vite + Tauri)
npm run dev          # dev server with hot reload
npm run build        # production build (tsc + vite)
npm run typecheck    # TypeScript check only
npm run lint         # ESLint on frontend/src

# Rust lib
cargo test           # all tests
cargo check         # typecheck only

# Tauri app
cd src-tauri && cargo check

# Game data
npm run data:build   # rebuild data/gamedata.db from seeds/*.csv
npm run data:verify   # verify gamedata integrity
```

## Package Structure

- `frontend/src/` — React UI (TypeScript)
- `src/` — Rust library: `gff4/` (binary format), `domain/` (models), `edit/` (save editor), `app/` (commands)
- `src-tauri/` — Tauri app wrapper
- `data/gamedata.db` — SQLite catalog (items, abilities, materials). Do not edit directly; rebuild from `data/seeds/*.csv` via `npm run data:build`.

## Key Conventions

- **GameId**: `Dao`, `DaoAwakening`, `Da2`. Inferred from GFF4 header version (`V1.1` = DAO, `V2.0` = DA2) and campaign resource string.
- **Edit pattern**: `SaveEditor` holds `raw: GffFile` + `save: SaveGame`. Every edit mutates both.
- **Always edit copies** of saves. Keep original `.das`, `.das.met`, `screen.dds` files.

## Skills

- `skills/codebase-map/` — module structure and conventions
- `skills/save-format/` — GFF4 structure and domain models
- `skills/editing-patterns/` — edit flow, validation, error types

## Refactoring Plan

See `docs/refactoring.md` for planned work (stat ID deduplication, value helpers, editor helper extraction).

## Editor Config

- ESLint ignores: `dist`, `src-tauri`, `target`, `node_modules`
- Tauri window: 1440x920, non-resizable
- Rust edition 2024, `rusqlite` with `bundled` feature
