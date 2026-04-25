# Save Format

## GFF4 Structure Overview

Save files (`.das`) are binary GFF4 format files. The structure is:

```
GffFile
├── header: GffHeader (file_type: "DAV", file_version: "V1.1" or "V2.0")
└── root: GffStruct
```

## Key Root Fields

| Label | ID | Type | Description |
|-------|----|------|-------------|
| SAVEGAME_PARTYLIST | 1 | Struct | Contains money, companions, backpack |
| SAVEGAME_PLAYERCHAR | 2 | Struct | Main character wrapper |
| SAVEGAME_CAMPAIGN | 3 | Struct | Campaign info (for game inference) |
| WVLT (16024) | 16024 | Struct | World vault (DA2 plot flags) |

## Extraction Flow

```
GffFile
  └─ root.SAVEGAME_PARTYLIST
       ├─ SAVEGAME_MONEY → save.money
       ├─ SAVEGAME_PARTYPOOLMEMBERS → save.companions[]
       ├─ SAVEGAME_BACKPACK → save.backpack[]
       ├─ SAVEGAME_CRAFTING_RECIPE_LIST → save.crafting_recipes[]
       └─ SAVEGAME_PARTY_APPROVAL_LIST → companion approvals

  └─ root.SAVEGAME_PLAYERCHAR.SAVEGAME_PLAYERCHAR_CHAR → save.main_character
```

## SaveGame Domain Model

```rust
pub struct SaveGame {
    pub preferred_game: Option<GameId>,
    pub money: u32,
    pub main_character: Character,
    pub companions: Vec<Character>,
    pub backpack: Vec<Item>,
    pub crafting_recipes: Vec<u32>,
    pub plot_flags: PlotFlags,
}
```

## Character Structure

```rust
pub struct Character {
    pub name: String,
    pub template_resref: Option<String>,
    pub approval: Option<i32>,
    pub level: Option<u32>,
    pub experience: Option<u32>,
    pub core_stats: CoreStats,
    pub point_pools: PointPools,
    pub equipment: Vec<Item>,
    pub skills: Vec<AbilityRef>,    // DA2: combined list; DAO: separate lists
    pub talents: Vec<AbilityRef>,
    pub spells: Vec<AbilityRef>,
}
```

### Core Stats
Six attributes identified by stat ID:
- 1 = Strength, 2 = Dexterity, 3 = Willpower, 4 = Magic, 5 = Cunning, 6 = Constitution

Extracted from `SAVEGAME_CREATURE_STATS.SAVEGAME_STATLIST`:
```rust
for stat in stat_list {
    stat_id = stat.SAVEGAME_STATPROPERTY_INDEX
    base = stat.SAVEGAME_STATPROPERTY_BASE
    // if core_stat_from_id(stat_id) → core_stats.set(stat, base)
}
```

### Stat IDs by Game

| Stat | DAO | DA2 |
|------|-----|-----|
| Level | 15 | 36 |
| Experience | 19 | 35 |
| Attribute Points | 34 | 38 |
| Skill Points | 35 | — |
| Talent Points | 36 | 39 |
| Specialization Points | 38 | — |

## Item Structure

```rust
pub struct Item {
    pub resref: Option<String>,
    pub name: Option<String>,
    pub wiki_url: Option<String>,
    pub category: ItemCategory,
    pub stackable: bool,
    pub object_id: Option<i32>,
    pub equipment_slot: Option<u32>,
    pub item_cost: Option<u32>,
    pub item_stacksize: Option<u32>,
    pub item_level: Option<u8>,
    pub material: Option<u32>,
    pub material_profile: Option<MaterialProfile>,
    pub material_info: Option<MaterialInfo>,
    pub properties: Vec<ItemProperty>,
}

pub struct ItemProperty {
    pub id: u32,
    pub name: Option<String>,
    pub power: f32,
}
```

Items have two parallel arrays:
- `ITEM_PROPERTIES` — property IDs
- `ITEM_PROPERTY_POWERS` — f32 power values

DA2 encodes property powers differently (bitcast from u32).

## Game Inference

```rust
fn infer_game(file: &GffFile) -> Option<GameId> {
    match &file.header.file_version {
        b"V1.1" => Some(infer_dao_campaign(file)),  // check campaign resource
        b"V2.0" => Some(GameId::Da2),
        _ => None,
    }
}

fn infer_dao_campaign(file: &GffFile) -> GameId {
    // Check root.SAVEGAME_CAMPAIGN.SAVEGAME_CAMPAIGN_RESOURCE
    // "DAO_PRC_EP_1", "DAO_PRC_STR", "DAO_PRC_GIB" → DaoAwakening
    // otherwise → Dao
}
```

## Plot Flags (DA2 Only)

Stored in `root.WVLT` (World Vault):
```rust
pub struct PlotFlags {
    pub booleans: BTreeMap<u16, bool>,  // WVB1 entries
    pub integers: BTreeMap<u16, i32>,    // WVI1 entries
}
```

Each entry: `{ WORLD_VAULT_ID_LABEL: u16, WORLD_VAULT_VALUE_LABEL: u8/u32 }`

DA2 only. Return `EditError::UnsupportedPlotFlags` for DAO.

## Gamedata Lookup

SQLite database (`data/gamedata.db`) enriches raw data:
- Item names, categories, stackability
- Material metadata (family, target)
- Ability names, trees, types
- Property names

```rust
pub trait GameDataLookup {
    fn item_metadata(&self, resref: &str, game: Option<GameId>) -> Result<Option<ItemMetadata>, LookupError>;
    fn item_material_profile(&self, resref: &str, game: Option<GameId>) -> Result<Option<MaterialProfile>, LookupError>;
    fn material_info(&self, code: u32, game: Option<GameId>) -> Result<Option<MaterialInfo>, LookupError>;
    fn ability(&self, id: u32, game: Option<GameId>) -> Result<Option<AbilityRef>, LookupError>;
    fn item_property_name(&self, id: u32, game: Option<GameId>) -> Result<Option<String>, LookupError>;
}
```
