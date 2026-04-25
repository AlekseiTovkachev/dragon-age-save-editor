# Editing Patterns

## SaveEditor Architecture

`SaveEditor` holds two parallel representations:
1. `raw: GffFile` — raw GFF4 bytes, mutated for file output
2. `save: SaveGame` — domain model, extracted once at load

**Invariant:** Every edit method mutates BOTH `raw` and `save` in sync.

```rust
pub struct SaveEditor {
    raw: GffFile,
    save: SaveGame,
}
```

## Edit Flow

### 1. Load
```rust
let mut editor = SaveEditor::from_path_with_lookup(path, lookup, None)?;
// or
let editor = SaveEditor::from_gff_with_lookup(raw_gff, lookup, preferred_game)?;
```

### 2. Edit
```rust
editor.set_money(99999)?;
editor.patch_character_core_stats(target, CoreStatsPatch { strength: Some(50), ..Default::default() })?;
editor.set_character_level(target, 27)?;
editor.add_item_property(InventoryContainer::Backpack, 0, property_id, 5.0, lookup)?;
```

### 3. Save
```rust
editor.write_to_path("output.das")?;
```

## Character Targeting

```rust
pub enum CharacterTarget {
    MainCharacter,
    Companion(usize),  // index into save.companions
}
```

## Inventory Containers

```rust
pub enum InventoryContainer {
    Backpack,
    Equipment { target: CharacterTarget },
}
```

## Core Edit Operations

### Money
```rust
fn set_money(&mut self, money: u32) -> Result<(), EditError>
```
Updates `root.SAVEGAME_PARTYLIST.SAVEGAME_MONEY`.

### Character Stats
```rust
fn patch_character_core_stats(&mut self, target: CharacterTarget, patch: CoreStatsPatch) -> Result<(), EditError>
fn set_character_stat(&mut self, target: CharacterTarget, stat: CoreStat, value: u32) -> Result<(), EditError>
fn set_character_level(&mut self, target: CharacterTarget, level: u32) -> Result<(), EditError>
fn set_character_experience(&mut self, target: CharacterTarget, experience: u32) -> Result<(), EditError>
fn patch_character_point_pools(&mut self, target: CharacterTarget, patch: PointPoolsPatch) -> Result<(), EditError>
```

Each stat update locates the stat row in `SAVEGAME_STATLIST` by `SAVEGAME_STATPROPERTY_INDEX`, then updates `SAVEGAME_STATPROPERTY_BASE`.

### Abilities
```rust
fn replace_character_abilities(
    &mut self,
    target: CharacterTarget,
    list: AbilityListKind,  // Skills | Talents | Spells
    ability_ids: &[u32],
    lookup: &dyn GameDataLookup,
) -> Result<(), EditError>
```

DA2 uses a combined `SAVEGAME_ABILITYLIST`; DAO uses separate `SAVEGAME_SKILLLIST/TALENTLIST/SPELLLIST`.

Validates:
- Ability kind matches list type
- Core abilities are present if ability requires them

### Items
```rust
fn patch_item_metadata(&mut self, container: InventoryContainer, index: usize, patch: ItemMetadataPatch) -> Result<(), EditError>
fn remove_backpack_item(&mut self, index: usize) -> Result<(), EditError>
fn clone_backpack_item(&mut self, index: usize) -> Result<usize, EditError>
fn set_backpack_item_stack_size(&mut self, index: usize, stack_size: u32) -> Result<(), EditError>
fn replace_backpack_item(&mut self, index: usize, replacement: BackpackItemReplacement) -> Result<(), EditError>
fn add_item_property(&mut self, container: InventoryContainer, index: usize, property_id: u32, power: f32, lookup: Option<&dyn GameDataLookup>) -> Result<(), EditError>
fn remove_item_property(&mut self, container: InventoryContainer, index: usize, property_index: usize) -> Result<(), EditError>
fn set_item_property_power(&mut self, container: InventoryContainer, index: usize, property_index: usize, power: f32) -> Result<(), EditError>
fn set_item_property_id(&mut self, container: InventoryContainer, index: usize, property_index: usize, property_id: u32, lookup: Option<&dyn GameDataLookup>) -> Result<(), EditError>
```

### Plot Flags (DA2 only)
```rust
fn patch_plot_flags(&mut self, booleans: &[PlotBooleanPatch], integers: &[PlotIntegerPatch]) -> Result<(), EditError>
```
Mutates `root.WVLT.WVB1` or `root.WVLT.WVI1`.

### Recipes
```rust
fn replace_crafting_recipes(&mut self, recipe_ids: &[u32]) -> Result<(), EditError>
```

## Validation Rules

### Stack Size
- Max: 99
- Error: `EditError::InvalidStackSize { stack_size }`

### Item Clone
- Only DAO family and DA2 support clone
- Error: `EditError::UnsupportedGameForClone`
- Non-stackable items only
- Error: `EditError::ItemIsStackable`

### Item Replacement
- Resref must match existing item
- Error: `EditError::BackpackResrefMismatch`

### Property Arrays
- `ITEM_PROPERTIES` and `ITEM_PROPERTY_POWERS` must have equal length
- Error: `EditError::InvalidPropertyArrayParity`

### Ability Kinds
- Each ability has a `kind` (Skill/Spell/Talent)
- List type must match ability kind
- Error: `EditError::InvalidAbilityKind`

### Core Abilities
- Some abilities require core abilities to be equipped
- Enforced during `replace_character_abilities`
- Error: `EditError::MissingCoreAbility`

### Plot Flags
- Only DA2 supports plot flags
- Error: `EditError::UnsupportedPlotFlags`

## Error Types

```rust
pub enum EditError {
    MissingField { path: String },
    InvalidTarget { target: CharacterTarget },
    InvalidItemIndex { container: InventoryContainer, index: usize },
    InvalidStackSize { stack_size: u32 },
    ItemIsStackable { index: usize },
    ItemIsNotStackable { index: usize },
    BackpackResrefMismatch { index: usize, expected: String, actual: String },
    InvalidPropertyIndex { container: InventoryContainer, item_index: usize, property_index: usize },
    InvalidPropertyArrayParity { container: InventoryContainer, item_index: usize, ids_len: usize, powers_len: usize },
    UnsupportedNumericValue { path: String, actual: String },
    UnsupportedPlotFlags { game: Option<GameId> },
    UnsupportedGameForClone { game: Option<GameId> },
    NumericRange { path: String, detail: String },
    InvalidAbilityKind { ability_id: u32, expected: AbilityListKind, actual: AbilityKind },
    UnknownAbility { ability_id: u32 },
    MissingCoreAbility { target: CharacterTarget, list: AbilityListKind, required_id: u32 },
    LookupFailed { path: String, detail: String },
    IoError { path: String, detail: String },
}
```

## Safety Guidelines

1. **Always edit copies** — Never modify original saves
2. **Preserve file associations** — `.das` + `.das.met` + `screen.dds` must stay together
3. **Check in game** — Verify edited saves load correctly before deleting originals
4. **Backup all three files** — The `.met` file contains critical metadata

## Raw GFF4 Mutation Pattern

When modifying `raw`, methods follow this pattern:

```rust
// 1. Get mutable reference to raw structure
let party = raw_party_mut(&mut self.raw)?;

// 2. Get mutable reference to field
let value = party.get_mut(SAVEGAME_MONEY).ok_or_else(|| EditError::MissingField { ... })?;

// 3. Update value using helper
set_numeric_value(value, money, "root.SAVEGAME_PARTYLIST.SAVEGAME_MONEY")?;

// 4. Update domain model
self.save.money = money;
```

Numeric value helpers handle type coercion:
```rust
fn set_numeric_value(value: &mut Value, new_value: u32, path: &str) -> Result<(), EditError>
fn set_signed_numeric_value(value: &mut Value, new_value: i32, path: &str) -> Result<(), EditError>
fn set_property_power_value(value: &mut Value, new_value: f32, game: Option<GameId>, path: &str) -> Result<(), EditError>
```
