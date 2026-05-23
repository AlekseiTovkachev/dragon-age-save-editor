---
name: editing-patterns
description: Use when adding or changing save-editing behavior in the Rust SaveEditor, app commands, validation, or frontend command flows for this Dragon Age Save Editor repo.
metadata:
  short-description: Save editing invariants and command flow
---

# Editing Patterns

## Core Invariant

`SaveEditor` owns two synchronized representations:

```rust
pub struct SaveEditor {
    raw: GffFile,
    save: SaveGame,
}
```

Every edit must update:

- the raw GFF tree, so `write_to_path` persists the change;
- the domain model, so subsequent commands and UI refreshes see the change immediately.

Add a write/reload test for edits that should persist to disk.

## Edit Flow

```rust
let db = SqliteGameData::open("data/gamedata.db")?;
let mut editor = SaveEditor::from_path_with_lookup("copy.das", Some(&db), None)?;
editor.set_money(99999)?;
editor.write_to_path("copy-edited.das")?;
```

Application commands go through `SaveDocument::execute(SaveCommand)` and return a tagged `SaveCommandResult`. Frontend callers should use `expectResult` from `frontend/src/api.ts` when a specific result shape is expected.

## Raw Mutation Pattern

1. Locate the raw `GffStruct` or `Value` by field ID/name.
2. Validate type, index, game support, and numeric range.
3. Mutate raw value with the existing helper style from `src/edit/internal.rs`.
4. Mutate the matching field in `self.save`.
5. Refresh command results or summaries where applicable.

Keep raw helpers private to `src/edit/internal.rs` unless the public API truly needs them.

## Game Differences

- DAO and DAO Awakening use GFF4 `V1.1`; DA2 uses `V2.0`.
- DAO Awakening-style saves are inferred from campaign resources.
- DAO has separate skill, talent, and spell lists.
- DA2 uses combined ability storage and supports plot flags.
- DA2 item property powers use the game-specific encoding helpers in `domain/game.rs`.
- Stat IDs and point pools live in `domain/stats.rs`.

## Common Edit Surfaces

- Characters: stats, level, experience, approval, point pools, abilities.
- Inventory: backpack items, equipment items, metadata, stack size, item replacement, properties.
- Crafting: recipe ID list.
- Plot flags: DA2 boolean and integer world-vault entries only.
- Money: party-list money field and summary dirty state.

## Validation Expectations

- Preserve unknown fields and unrelated save data.
- Return typed errors from `EditError`/`CommandError`; avoid string-only failure modes.
- Keep unsupported game behavior explicit, especially DAO vs DA2 plot flags and ability storage.
- Maintain `ITEM_PROPERTIES` / `ITEM_PROPERTY_POWERS` parity.
- Validate item stackability, resref replacement, ability kind, and core ability requirements.

## Test Expectations

- Pure frontend helpers: Vitest unit tests.
- Command DTO/frontend contract: `frontend/src/api.contract.test.ts`.
- Rust extraction/editing: `cargo test`.
- Real persistence: write/reload tests using copied or temporary save paths.
- End-to-end UI smoke: Playwright tests in `verification/smoke/` with `VITE_E2E_MOCK=1`.

Run `npm run verify` before handing off broad behavior changes.
