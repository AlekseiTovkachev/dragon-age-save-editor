# Refactoring Status

This file tracks backend refactoring status. For forward-looking work, see `docs/roadmap.md`.

## Completed

- Stat IDs and point-pool IDs are centralized in `src/domain/stats.rs`.
- Raw editor helpers were extracted from `src/edit/editor.rs` into `src/edit/internal.rs`.
- Item property array handling is encapsulated by helper types in `src/edit/internal.rs`.
- Game-specific behavior lives in `src/domain/game.rs`.
- `EditError` uses `thiserror`.
- Missing stat-row templates have a dedicated `NoStatRowTemplate` error.

## Remaining

### Consolidate GFF Value Conversion Helpers

Numeric conversion is still partly duplicated between extraction and editing code.

Current goal:

- keep coercion behavior consistent for unsigned integers, signed integers, and floats;
- reject invalid values such as negative unsigned values and non-finite floats;
- keep DA2 item-property power encoding separate from ordinary numeric conversion;
- prefer small inherent methods on `gff4::Value` or one shared helper module over ad hoc local functions.

Likely touch points:

- `src/gff4/value.rs`
- `src/domain/save.rs`
- `src/edit/internal.rs`

## Guardrails

- Preserve the `SaveEditor` invariant: raw GFF and domain state must stay in sync.
- Add or keep write/reload tests for persisted editor behavior.
- Run `npm run verify` after behavior changes.
