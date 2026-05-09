# Fixtures And Sample Saves

Use copied saves only. Never mutate a user's original `.das` file in place.

## Local Fixture Directory

`sample_saves/` is ignored by git and may contain local DAO, DAO Awakening-style, and DA2 saves for development and testing.

Recommended structure:

```
sample_saves/
├── dao/
├── daoa/
└── da2/
```

Keep each save's sibling files together when available:

- `.das`
- `.das.met`
- `screen.dds`

## Test Usage

- Rust tests may read fixtures from `sample_saves/` when present, but should write edited output to temporary paths.
- Smoke tests should use the mocked Tauri backend, not local fixture files.
- Manual Tauri QA should use duplicated saves and verify saved copies in game before deleting originals.

## Fixture Coverage Goals

Keep at least one working local sample for:

- DAO vanilla;
- DAO Awakening-style campaign, including expansion or DLC saves when possible;
- DA2 with world-vault plot flags;
- stackable and non-stackable backpack items;
- items with multiple properties;
- crafting recipes.

Document fixture-specific quirks in test names or comments instead of relying on path names alone.
