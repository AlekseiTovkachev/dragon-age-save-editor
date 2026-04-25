# Refactoring Plan

## Overview

This document outlines planned refactoring work to reduce duplication, improve maintainability, and prepare the codebase for future UI development.

## Priority 1: Eliminate Duplicate Stat ID Functions

### Problem

Stat ID lookup functions are duplicated across two files:

- `domain/save.rs` (lines 872–916)
- `edit/editor.rs` (lines 2037–2072)

| Function | Location |
|----------|----------|
| `core_stat_id(stat: CoreStat) -> u32` | Both |
| `level_stat_id(preferred_game: Option<GameId>) -> u32` | Both |
| `experience_stat_id(preferred_game: Option<GameId>) -> u32` | Both |
| `point_pool_stat_id(kind: PointPoolKind, preferred_game: Option<GameId>) -> Option<u32>` | Both |

### Solution

Move all stat ID constants and lookup functions to `domain/stats.rs`. The `edit/editor.rs` currently defines `PointPoolKind` locally (line 788) — this enum should also be in `domain/stats.rs`.

### Files Affected

- `domain/stats.rs` — add stat ID constants and `PointPoolKind` enum
- `domain/save.rs` — import from `domain/stats.rs` and remove local definitions
- `edit/editor.rs` — import from `domain/stats.rs` and remove local definitions

---

## Priority 2: Consolidate Value Conversion Helpers

### Problem

Numeric value conversion functions are duplicated:

- `domain/save.rs` (lines 767–835): `value_to_u32`, `value_to_u16`, `value_to_i32`, `value_to_f32`
- `edit/editor.rs` (lines 2005–2035): `value_to_u32`, `value_to_u16`, `value_to_i32`

Additionally, there are three separate setters with overlapping logic:

- `set_numeric_value` — u32 target
- `set_signed_numeric_value` — i32 target
- `set_float_value` — f32 target
- `set_property_power_value` — f32 with DA2 bitcast encoding

### Solution

1. Move `value_to_u32`, `value_to_u16`, `value_to_i32`, `value_to_f32` to `gff4/value.rs` as inherent methods on `Value`.

2. Consider extracting the numeric setter logic into a helper module or trait in `edit/internal.rs`.

### Files Affected

- `gff4/value.rs` — add conversion methods
- `domain/save.rs` — use `Value::value_to_u32()` etc.
- `edit/editor.rs` — use `Value::value_to_u32()` etc.

---

## Priority 3: Extract Editor Private Helpers

### Problem

`edit/editor.rs` is ~2100 lines. The bottom half (lines 1464–2093) contains private helper functions that handle raw GFF4 manipulation:

- Property array helpers: `property_lists_mut`, `ensure_property_lists_mut`, `append_property_id_value`, `append_property_power_value`, etc.
- Stat row helpers: `set_character_stat_row_value`, `set_or_insert_character_stat_row_value`, `insert_character_stat_row_value`
- Numeric setters: `set_numeric_value`, `set_signed_numeric_value`, `set_float_value`, `set_property_power_value`
- Value conversion: `value_to_u32`, `value_to_u16`, `value_to_i32`
- Misc: `clean_resref`, `CharacterAbilityAccess`, `NumericValueKind`, `FloatValueKind`

### Solution

Extract to `edit/internal.rs` (or `edit/raw_helpers.rs`). This module would be private to the edit crate, exposing only `SaveEditor` publicly.

### Files Affected

- New file: `edit/internal.rs`
- `edit/editor.rs` — reduce to public API only
- `edit/mod.rs` — no change (keeps `SaveEditor` re-exported)

---

## Priority 4: Encapsulate Item Property Arrays

### Problem

The `ITEM_PROPERTIES` and `ITEM_PROPERTY_POWERS` parallel arrays require careful parity maintenance. The current implementation in `property_lists_mut` and surrounding functions (lines 1365–1494) is complex:

- Finds field indices by name
- Checks which array appears first
- Splits and joins the fields slice
- Validates parity
- Handles DA2 encoding differences

This logic is spread across multiple helper functions and is difficult to follow.

### Solution

Create an `ItemProperties` wrapper struct in `edit/internal.rs`:

```rust
struct ItemProperties<'a> {
    ids: &'a mut Vec<Value>,
    powers: &'a mut Vec<Value>,
}

impl ItemProperties {
    fn from_item(item: &mut GffStruct, container: InventoryContainer, index: usize) -> Result<Self, EditError>
    fn push(&mut self, property_id: u32, power: f32, game: Option<GameId>) -> Result<(), EditError>
    fn remove(&mut self, index: usize)
    fn len(&self) -> usize
}
```

### Files Affected

- `edit/internal.rs` — new `ItemProperties` struct
- `edit/editor.rs` — refactor all item property operations to use `ItemProperties`

---

## Priority 5: Game-Specific Behavior Strategy

### Problem

Game-specific branching (`if preferred_game.is_some_and(GameId::is_da2)`) is scattered throughout the codebase:

- Property power encoding (DA2 bitcasts floats through u32)
- Combined vs. separate ability lists
- Plot flags support (DA2 only)
- Stat ID mappings

### Solution

Consider a `GameBehavior` trait in `domain/gamedata.rs` or a dedicated `domain/game.rs`:

```rust
trait GameBehavior {
    fn stat_id_for(stat: CoreStat) -> u32;
    fn level_stat_id(&self) -> u32;
    fn experience_stat_id(&self) -> u32;
    fn encode_property_power(&self, value: f32) -> Value;
    fn ability_list_style(&self) -> AbilityListStyle;
    fn supports_plot_flags(&self) -> bool;
}
```

Implement for `GameId::Dao`, `GameId::DaoAwakening`, `GameId::Da2`.

### Files Affected

- New file: `domain/game.rs` (or added to `domain/gamedata.rs`)
- `domain/save.rs`, `edit/editor.rs` — use game behavior trait

---

## Priority 6: Migrate EditError to thiserror

### Problem

`edit/errors.rs` manually implements `Display` and `Error` (lines 97–194). Other error types in the codebase use `thiserror` for cleaner derivation.

### Solution

Migrate to `thiserror`:

```rust
#[derive(Debug, thiserror::Error)]
pub enum EditError {
    #[error("invalid character target: {target:?}")]
    InvalidTarget { target: CharacterTarget },

    #[error("missing field at {path}")]
    MissingField { path: String },
    // ...
}
```

### Files Affected

- `edit/errors.rs` — migrate to `thiserror`
- Verify no breakages via `cargo check`

---

## Priority 7: Template-Based Row Insertion Safety

### Problem

`insert_character_stat_row_value` (lines 1726–1771) clones the first stat row as a template and modifies it. If no stat rows exist, it returns a cryptic `MissingField` error.

### Solution

1. Add validation that at least one stat row exists before attempting insertion.
2. Consider a dedicated error variant `NoStatRowTemplate` for clarity.

---

## Non-Goals (Out of Scope)

- Changing the GFF4 binary format handling
- Modifying the SQLite schema or gamedata pipeline
- Breaking public API changes (library consumers should not be affected)

---

## Order of Implementation

1. **Stat ID consolidation** — lowest risk, highest duplicate removal
2. **Value conversion helpers** — moderate risk
3. **Extract editor helpers** — moderate risk, improves readability
4. **ItemProperties encapsulation** — isolates complex logic
5. **Game behavior strategy** — larger refactor, do when UI needs it
6. **thiserror migration** — low risk, mechanical
7. **Template insertion safety** — low risk, defensive
