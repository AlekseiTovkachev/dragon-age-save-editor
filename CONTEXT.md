# Dragon Age Save Editor — Domain Glossary

> Agents read this file early in every session.
> Consistent terminology is how a team of agents stays coherent across sessions.

---

## Save File Format

| Term | Definition |
|---|---|
| **GFF4** | Generic File Format 4 — the binary container format Dragon Age uses for save files |
| **GffFile** | Rust struct: top-level parsed representation (`header: ResolvedHeader` + `root: GffStruct`) |
| **GffStruct** | A node in the GFF4 tree — maps field IDs to `Value` entries |
| **ResolvedHeader** | Parsed GFF4 header carrying file type, version, and game-inference data |
| **`.das`** | Dragon Age save file extension |
| **V1.1** | GFF4 header version for DAO-family saves (DAO vanilla + Awakening-style) |
| **V2.0** | GFF4 header version for DA2 saves |
| **field ID** | Numeric identifier for a GFF4 field. Constants live in `src/gff4/fields.rs`. Use the constants — never magic numbers. |
| **field name / label** | Human-readable label for a GFF4 field (e.g. `SAVEGAME_MONEY`) |

---

## Game Identifiers

| Term | Definition |
|---|---|
| **GameId** | Enum: `Dao`, `DaoAwakening`, `Da2`. Inferred from GFF4 version + campaign resource. |
| **DAO-family** | Dragon Age: Origins vanilla saves — `V1.1`, `GameId::Dao` |
| **DaoAwakening** | Awakening-style saves (Awakening, Witch Hunt, Golems of Amgarrak) — `V1.1` + specific campaign resource strings |
| **DA2** | Dragon Age II saves — `V2.0`, `GameId::Da2` |

---

## Domain Model

| Term | Definition |
|---|---|
| **SaveGame** | The extracted domain model of a loaded save — characters, inventory, crafting, plot flags |
| **SaveEditor** | Rust struct that owns both `raw: GffFile` (the binary tree) and `save: SaveGame` (the domain model). Every edit must update both. |
| **dual-sync invariant** | The rule that every `SaveEditor` mutation must update both `raw` and `save`. Violating this causes stale UI or corrupt writes. |
| **SaveCommand** | Tagged enum of all valid edit operations. Sent from the frontend, executed by `SaveDocument`. |
| **SaveCommandResult** | Tagged result type returned to the frontend after a command. Frontend uses `expectResult` to narrow the variant. |
| **SaveDocument** | Application-layer wrapper around `SaveEditor` — manages document lifecycle and command dispatch. |

---

## Game Data

| Term | Definition |
|---|---|
| **gamedata.db** | Generated SQLite catalog enriching raw save IDs with item names, categories, materials, abilities, etc. Located at `data/gamedata.db`. |
| **seed CSV** | Source-of-truth files under `data/seeds/`. Game data changes go here — never directly in the DB. |
| **GameDataLookup** | Rust trait for accessing game data. Use this in editing code — never query SQLite directly. |
| **data:build** | `npm run data:build` — rebuilds `gamedata.db` from seed CSVs |
| **data:verify** | `npm run data:verify` — verifies gamedata integrity rules |

---

## Item Data

| Term | Definition |
|---|---|
| **ITEM_PROPERTIES** | GFF4 field: parallel array of property IDs on an item |
| **ITEM_PROPERTY_POWERS** | GFF4 field: parallel array of power values for each property. Must stay the same length as `ITEM_PROPERTIES`. |
| **parallel-array invariant** | `ITEM_PROPERTIES` and `ITEM_PROPERTY_POWERS` must always have the same length. |
| **stackability** | Whether items of a category can be stacked. Governed by category rules in game data. |
| **resref** | Resource reference — the game's string ID for an item. Used for item replacement. |

---

## Testing

| Term | Definition |
|---|---|
| **write/reload test** | A Rust test that applies an edit, writes the save to a temp path, re-reads it, and asserts the change persisted. Required for any edit that should survive to disk. |
| **smoke test** | A Playwright browser test exercising main user workflows through the real React UI with a mocked Tauri backend. Lives in `smoke/`. |
| **mocked backend** | `frontend/src/test/mockBackend.ts` — simulates Tauri command responses for smoke and unit tests. Update when smoke-visible command behavior changes. |
| **verification gate** | `npm run verify` — the full project check: typecheck + lint + unit + Rust + Tauri + game data. Required before declaring anything done. |
| **contract test** | `frontend/src/api.contract.test.ts` — checks TypeScript/Rust command DTO compatibility assumptions. Update when command shapes change. |

---

## Safety

| Term | Definition |
|---|---|
| **safety copy** | A copy of the original save files. Always edit copies — never originals. |
| **original save** | The untouched `.das`, `.das.met`, and `screen.dds` files from the game. Keep until the edited save has been loaded and verified in-game. |
