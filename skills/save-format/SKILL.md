---
name: save-format
description: Use when parsing, validating, mutating, or reasoning about Dragon Age DAO/Awakening/DA2 GFF4 save structure, domain extraction, game inference, items, abilities, plot flags, or gamedata lookup.
metadata:
  short-description: Dragon Age GFF4 save format guide
---

# Save Format

## GFF4 Overview

Dragon Age saves (`.das`) are GFF4 binary files parsed into:

```rust
pub struct GffFile {
    pub header: ResolvedHeader,
    pub root: GffStruct,
}
```

`ResolvedHeader` carries the file type/version information used for game inference. DAO-family saves are `V1.1`; DA2 saves are `V2.0`.

## Key Root Fields

| Label | ID | Purpose |
| --- | ---: | --- |
| `SAVEGAME_PARTYLIST` | 1 | Money, companions, backpack, recipes, approval |
| `SAVEGAME_PLAYERCHAR` | 2 | Main character wrapper |
| `SAVEGAME_CAMPAIGN` | 3 | DAO campaign resource for Awakening inference |
| `WVLT` | 16024 | DA2 world vault for plot flags |

Field IDs live in `src/gff4/fields.rs`; prefer constants and `field_id_by_name()` over magic numbers.

## Extraction Shape

```
root.SAVEGAME_PARTYLIST
├── SAVEGAME_MONEY
├── SAVEGAME_PARTYPOOLMEMBERS -> companions
├── SAVEGAME_BACKPACK -> backpack
├── SAVEGAME_CRAFTING_RECIPE_LIST -> crafting_recipes
└── SAVEGAME_PARTY_APPROVAL_LIST -> companion approval

root.SAVEGAME_PLAYERCHAR.SAVEGAME_PLAYERCHAR_CHAR -> main_character
root.WVLT -> DA2 plot_flags
```

`SaveGame::from_gff_with_lookup()` extracts domain state and enriches it with optional gamedata lookup.

## Game Inference

- `V1.1` means DAO-family.
- DAO campaign resources such as Awakening, Witch Hunt, and Golems of Amgarrak map to `GameId::DaoAwakening`.
- `V2.0` maps to `GameId::Da2`.
- Game-specific behavior lives in `src/domain/game.rs`.

## Character Notes

- Core stats use IDs 1 through 6: strength, dexterity, willpower, magic, cunning, constitution.
- Level, experience, and point-pool stat IDs vary by game; use `src/domain/stats.rs`.
- DAO stores skills, talents, and spells separately.
- DA2 stores abilities differently but exposes the same `Character` domain shape.

## Item Notes

`Item` contains raw save fields plus gamedata-enriched metadata such as category, stackability, material profile, material info, wiki URL, and property names.

Properties use parallel arrays:

- `ITEM_PROPERTIES`: property IDs.
- `ITEM_PROPERTY_POWERS`: property power values.

Always keep these arrays the same length. Use existing item-property helpers in `src/edit/internal.rs`.

## Plot Flags

DA2 plot flags live in `root.WVLT`.

- Boolean flags: `WVB1`.
- Integer flags: `WVI1`.
- Domain shape: `PlotFlags { booleans: BTreeMap<u16, bool>, integers: BTreeMap<u16, i32> }`.

DAO-family saves do not support plot flag editing and should return `UnsupportedPlotFlags`.

## Gamedata Lookup

`data/gamedata.db` enriches raw IDs and resrefs. Access it through `GameDataLookup` rather than querying SQLite directly from editing code.

Typical lookup surfaces:

- items and stackability;
- material families, targets, and tiers;
- abilities, trees, kinds, and core requirements;
- item property names.

Rebuild the database from seed CSVs with `npm run data:build`; verify with `npm run data:verify`.
