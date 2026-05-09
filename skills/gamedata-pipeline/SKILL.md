---
name: gamedata-pipeline
description: Use when editing Dragon Age item, ability, material, recipe, property, stackability, or category data, or when changing the SQLite gamedata build and verification pipeline.
metadata:
  short-description: Game data seed and verifier workflow
---

# Gamedata Pipeline

## Start Here

- Read `docs/data-pipeline.md`.
- Edit seed CSVs and schema under `data/`; do not hand-edit `data/gamedata.db`.
- Use the `GameDataLookup` trait from runtime code rather than direct SQLite queries.

## Commands

```bash
npm run data:build
npm run data:verify
```

Use the report command from `docs/data-pipeline.md` when checking row counts.

## Change Areas

- Items: resrefs, display names, categories, stackability, wiki URLs.
- Abilities: IDs, names, trees, kinds, core requirements, game keys.
- Materials: family, target, tier, game-specific availability.
- Properties: property IDs and display names.
- Recipes and plot flag catalogs: static Rust catalogs under `src/app/catalogs`.

## Guardrails

- Update verifier rules when adding a new category, stackability rule, game key, or data integrity rule.
- Keep DAO, DAO Awakening, and DA2 availability distinct.
- Rebuild generated DB artifacts only from source data.
- Add tests or verifier cases for data rules that could silently drift.
