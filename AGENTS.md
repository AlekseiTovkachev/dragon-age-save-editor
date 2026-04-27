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
npm run verify       # full verification gate
npm run smoke        # browser smoke tests with mocked Tauri backend
npm run coverage     # frontend + Rust coverage reports
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

- `skills/codebase-map/SKILL.md` — module structure, ownership boundaries, verification commands
- `skills/save-format/SKILL.md` — GFF4 structure, domain extraction, game-specific save behavior
- `skills/editing-patterns/SKILL.md` — edit flow, validation, command contracts, test expectations
- `skills/testing-and-coverage/SKILL.md` — test placement, smoke coverage, coverage commands
- `skills/command-contract/SKILL.md` — Rust/TypeScript command DTO and result contract
- `skills/gamedata-pipeline/SKILL.md` — seed data, generated DB, verifier rules
- `skills/frontend-workflows/SKILL.md` — React hooks, panels, Apply/Reset and dirty-state flows
- `skills/tauri-desktop/SKILL.md` — Tauri shell, capabilities, icons, native desktop behavior
- `skills/fixture-safety/SKILL.md` — local sample-save and copied-save safety

## Current Docs

- `docs/architecture.md` — current system architecture
- `docs/testing.md` — verification, smoke tests, and coverage
- `docs/command-contract.md` — Rust/TypeScript command DTO contract
- `docs/fixtures.md` — local sample-save guidance
- `docs/roadmap.md` — current known gaps and near-term work
- `docs/refactoring.md` and `docs/frontend-refactoring.md` — completed refactoring status plus remaining notes

## Editor Config

- ESLint ignores: `dist`, `src-tauri`, `target`, `node_modules`
- Tauri window: 1440x920, non-resizable
- Rust edition 2024, `rusqlite` with `bundled` feature
