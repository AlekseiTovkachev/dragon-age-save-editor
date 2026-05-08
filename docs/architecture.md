# Architecture

Dragon Age Save Editor is a local React, Rust, and Tauri application for editing copied DAO, DAO Awakening-style, and DA2 saves.

## Layers

```
React UI
  -> frontend/src/api.ts
  -> Tauri invoke commands
  -> src/app SaveCommand
  -> src/edit SaveEditor
  -> src/gff4 binary read/write
```

## Frontend

- `frontend/src/App.tsx`: app shell and top-level orchestration.
- `frontend/src/api.ts`: Tauri `invoke` wrapper, smoke-test mock switch, and `expectResult` result narrowing.
- `frontend/src/features/app`: document lifecycle, summary, assets, commit/save orchestration.
- `frontend/src/features/characters`: character drafts, ability planning, character panel.
- `frontend/src/features/inventory`: inventory drafts, item command planning, inventory panel.
- `frontend/src/features/crafting`: recipe drafts and panel.
- `frontend/src/features/plotFlags`: DA2 plot flag drafts and panel.
- `frontend/src/components`: reusable UI controls and item editor primitives.
- `frontend/src/lib`: pure formatting, grouping, navigation, and draft helpers.
- `frontend/src/test`: factories, setup, and mocked backend used by unit and smoke tests.

## Rust Library

- `src/gff4`: binary format reader/writer, value model, numeric conversion/range helpers, schema, headers, and field ID constants.
- `src/domain`: extracted save model and game data concepts.
- `src/domain/game.rs`: game-specific behavior.
- `src/domain/stats.rs`: stat IDs and point-pool helpers.
- `src/edit`: `SaveEditor`, edit request types, edit errors, target access helpers, and raw mutation helpers.
- `src/app`: command layer, DTOs, document wrapper, catalogs, and command errors.
- `src/validate`: structural validation.

## Tauri

- `src-tauri/src/main.rs` exposes desktop commands for opening documents, checking document state, and executing save commands.
- `src-tauri/tauri.conf.json` owns window and bundle settings.
- `src-tauri/capabilities/default.json` controls command permissions.

## Data

`data/gamedata.db` is generated from `data/schema.sql` and seed CSVs under `data/seeds/`. Runtime code should use the `GameDataLookup` trait rather than querying SQLite directly from UI or editor logic.

## Core Invariants

- Edit copied saves only.
- Keep `.das`, `.das.met`, and `screen.dds` together for manual game validation.
- Every `SaveEditor` edit updates both raw GFF and extracted domain state.
- Game-specific behavior should be explicit and covered for DAO-family vs DA2 differences.
- Generated data and schema artifacts should be reproducible from source files and commands.
