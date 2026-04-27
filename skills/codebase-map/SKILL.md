---
name: codebase-map
description: Use when working in this Dragon Age Save Editor repo and you need the current module layout, ownership boundaries, or verification commands before changing code.
metadata:
  short-description: Dragon Age Save Editor codebase map
---

# Codebase Map

## Start Here

- Read `AGENTS.md` first for repo commands and safety conventions.
- Use this skill before broad code changes, cross-platform work, or test additions.
- For binary save details, also read `skills/save-format/SKILL.md`.
- For mutation patterns, also read `skills/editing-patterns/SKILL.md`.

## Repository Layout

```
dragon-age-save-editor/
├── frontend/src/     # React + TypeScript UI
├── src/              # Rust library: GFF4, domain, editor, app commands
├── src-tauri/        # Tauri desktop shell
├── data/             # SQLite schema, seed CSVs, generated gamedata.db
├── tools/            # Cross-platform verification and data helpers
├── smoke/            # Playwright smoke tests with mocked Tauri backend
├── docs/             # Architecture, testing, QA, data, roadmap notes
└── sample_saves/     # Ignored local fixtures used for manual and smoke-like coverage
```

## Rust Library

- `src/gff4/`: low-level GFF4 read/write, schema, header, value, and field IDs.
- `src/domain/`: extracted save models, game behavior, stats, items, abilities, gamedata lookup.
- `src/edit/`: `SaveEditor` public mutation API plus private raw-GFF helpers in `internal.rs`.
- `src/app/`: command DTOs, document wrapper, command execution, catalogs, Tauri-facing errors.
- `src/validate/`: structural validation rules.
- `src/main.rs`: lightweight CLI/debug entry point.

Key invariant: `SaveEditor` owns both raw `GffFile` and extracted `SaveGame`; every edit mutates both.

## Frontend

- `frontend/src/App.tsx`: top-level orchestration.
- `frontend/src/features/*`: feature panels and state hooks for characters, inventory, crafting, plot flags, and app shell behavior.
- `frontend/src/components/*`: reusable UI and editor components.
- `frontend/src/lib/*`: pure grouping, formatting, navigation, and draft helpers.
- `frontend/src/test/*`: factories, setup, and mocked backend for unit/smoke tests.
- `frontend/src/api.ts`: Tauri invoke wrapper plus smoke-test mock switch.

## Tauri Shell

- `src-tauri/src/main.rs`: exposes desktop commands.
- `src-tauri/tauri.conf.json`: window and bundle configuration.
- `src-tauri/capabilities/default.json`: command permissions.
- Generated schema files under `src-tauri/gen/schemas/` may change when Tauri config changes.

## Game Data

- Edit seed CSVs and schema under `data/`; do not hand-edit `data/gamedata.db`.
- Build with `npm run data:build`.
- Verify with `npm run data:verify`.
- See `docs/data-pipeline.md` for row-count reporting and verifier scope.

## Verification

- Main gate: `npm run verify`.
- Frontend only: `npm run check`.
- Browser smoke tests: `npm run smoke`.
- Coverage: `npm run coverage`.
- Rust only: `cargo test` and `cargo check`.
- Tauri check: `cargo check --manifest-path src-tauri/Cargo.toml`.

See `docs/testing.md` before changing test conventions or coverage thresholds.
