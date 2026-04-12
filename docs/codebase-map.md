# Codebase Map

## Areas

`src/app`

Frontend/Tauri-facing app service layer. Owns command DTOs, `SaveDocument` state, command dispatch, and domain-to-DTO conversion.

`src/domain`

Pure-ish save domain models, game data lookup, extracted save representation, and stats/items/abilities.

`src/edit`

Save mutation layer. Owns raw GFF mutation and synchronized domain updates.

`src/gff4`

Low-level GFF4 read/write/schema/value support.

`src/validate`

Save validation for expected structural shapes.

`src-tauri`

Tauri command bridge and app shell.

`frontend/src`

React UI and Tauri command client.

`tools/gamedata` and `data`

Reproducible game catalog database pipeline.

## Review Order

1. `gff4`: low-level serialization, mostly stable.
2. `domain`: extracted model and lookup rules.
3. `edit`: mutation correctness and raw/domain synchronization.
4. `app`: command boundary and DTOs.
5. `frontend`: state ownership and rendering.
6. `data`: source-of-truth and verification.
