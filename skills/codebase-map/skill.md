# Codebase Map

## Module Overview

```
dragon_age_save_editor/
├── gff4/           # Low-level GFF4 binary format
├── domain/         # Domain models (save, character, item, abilities, stats)
├── edit/           # Save editor (mutates raw GFF4 + domain in sync)
├── app/            # Commands, DTOs, document model
├── validate/       # Validation rules
└── main.rs         # CLI entry point
```

## Key Modules

### `gff4/` — GFF4 Binary Format
- `reader.rs` — Parse `.das` files into `GffFile`
- `writer.rs` — Serialize `GffFile` back to bytes
- `header.rs` — File type/version header
- `fields.rs` — Field ID constants (e.g., `SAVEGAME_MONEY`, `SAVEGAME_BACKPACK`)
- `schema.rs` — GFF4 struct/list schemas
- `value.rs` — Value enum (UInt8, Int32, Struct, List, etc.)

**Key type:** `GffFile` with `root: GffStruct`

### `domain/` — Domain Models
- `save.rs` — `SaveGame` struct, extraction from GFF4, `PlotFlags`
- `character.rs` — `Character` with stats, abilities, equipment
- `item.rs` — `Item`, `ItemCategory`, `ItemProperty`, `MaterialProfile`
- `ability.rs` — `AbilityRef`, `AbilityKind` (Skill/Spell/Talent)
- `stats.rs` — `CoreStats` (STR/DEX/WIL/MAG/CUN/CON), `PointPools`
- `gamedata.rs` — SQLite lookup for item/ability names, material info (`GameDataLookup` trait)

**Key type:** `SaveGame` owns domain state; extracted from `GffFile` via `SaveGame::from_gff_with_lookup()`

### `edit/` — Save Editor
- `editor.rs` — `SaveEditor` struct: holds `raw: GffFile` + `save: SaveGame`
- All edit methods mutate both `raw` and `save` in sync
- `types.rs` — Edit-specific types (`CharacterTarget`, `InventoryContainer`, patch structs)
- `errors.rs` — `EditError` enum

**Pattern:** Every mutation method updates raw GFF4 bytes AND domain state together.

### `app/` — Application Layer
- `commands.rs` — `SaveCommand` enum + `SaveCommandResult`
- `document.rs` — `SaveDocument` wraps `SaveEditor`
- `dto.rs` — Data transfer objects for commands
- `catalogs/` — Static data: `crafting_recipes.rs`, `plot_flags.rs`

## Key Conventions

### `GameId` Enum
```rust
pub enum GameId {
    Dao,
    DaoAwakening,
    Da2,
}
```
- Game inferred from GFF4 header version (`V1.1` = DAO, `V2.0` = DA2)
- DAO campaign resource string distinguishes vanilla vs Awakening
- Passed as `preferred_game: Option<GameId>` to enable game-specific logic

### Error Handling Pattern
- `ExtractError` — failures parsing GFF4 into domain
- `EditError` — failures during editing operations
- `LookupError` — SQLite gamedata lookup failures
- All use `thiserror` for `std::error::Error` + `Display`

### Value Conversion
- `value_to_u32()`, `value_to_i32()`, `value_to_f32()` — safe numeric coercion
- Handles mixed integer sizes, filters invalid values (negative, NaN)
- Used throughout extraction and editing

### Field IDs
All GFF4 field IDs defined in `gff4/fields.rs`:
```rust
pub const SAVEGAME_PARTYLIST: u32 = 1;
pub const SAVEGAME_MONEY: u32 = 5;
pub const SAVEGAME_BACKPACK: u32 = 11;
// etc.
```
Field names also resolvable via `field_id_by_name()`.

## Entry Points

### CLI (`main.rs`)
```
GffFile::from_path() → SaveGame::from_gff_with_lookup() → print summary
```

### Library Use
```rust
use dragon_age_save_editor::edit::SaveEditor;
use dragon_age_save_editor::domain::gamedata::SqliteGameData;

let db = SqliteGameData::open("data/gamedata.db")?;
let mut editor = SaveEditor::from_path_with_lookup("save.das", Some(&db), None)?;
editor.set_money(99999)?;
editor.write_to_path("save_edited.das")?;
```

### Application Layer (`app/commands.rs`)
Commands received as `SaveCommand` enum, executed against `SaveDocument`, return `SaveCommandResult`.
