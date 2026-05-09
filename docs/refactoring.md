# Refactoring Status

This file tracks backend refactoring status. For forward-looking work, see `docs/roadmap.md`.

## Completed

- Stat IDs and point-pool IDs are centralized in `src/domain/stats.rs`.
- Raw editor helpers were extracted from `src/edit/editor.rs` into `src/edit/internal.rs`.
- Item property array handling is encapsulated by helper types in `src/edit/internal.rs`.
- Game-specific behavior lives in `src/domain/game.rs`.
- `EditError` uses `thiserror`.
- Missing stat-row templates have a dedicated `NoStatRowTemplate` error.
- GFF numeric conversion is centralized in `src/gff4/numeric.rs`; extraction and editing share compatible read helpers, range-checked write helpers, and explicit DA2 item-property power encoding.

## Remaining

No active backend refactoring items are tracked here. For forward-looking work, see `docs/roadmap.md`.

## Guardrails

- Preserve the `SaveEditor` invariant: raw GFF and domain state must stay in sync.
- Add or keep write/reload tests for persisted editor behavior.
- Run `npm run verify` after behavior changes.
