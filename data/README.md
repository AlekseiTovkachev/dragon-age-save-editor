# Game Data

`gamedata.db` contains catalog data used to enrich save contents with names, categories, material metadata, item properties, and ability metadata.

Source files:

- `schema.sql`: SQLite schema.
- `seeds/*.csv`: normalized table contents. Item rows are split into `items_dao.csv`, `items_daoa.csv`, and `items_da2.csv`; ability rows are split into `abilities_dao.csv` and `abilities_da2.csv`.
- `gamedata.db`: generated database used by the app at runtime.

Rebuild and verify:

```powershell
npm run data:build
npm run data:verify
```

The item and ability rows are based on Dragon Age wiki data plus manually verified save-editor behavior for DAO, Awakening-style DAO saves, and DA2.
