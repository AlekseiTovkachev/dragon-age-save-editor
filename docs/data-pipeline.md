# Game Data Pipeline

The runtime database is `data/gamedata.db`.

It is generated from:

- `data/schema.sql`
- `data/seeds/abilities_dao.csv`
- `data/seeds/abilities_da2.csv`
- `data/seeds/items_dao.csv`
- `data/seeds/items_daoa.csv`
- `data/seeds/items_da2.csv`
- `data/seeds/item_properties.csv`
- `data/seeds/material_codes.csv`
- `data/seeds/properties.csv`

Build:

```powershell
npm run data:build
```

Verify:

```powershell
npm run data:verify
```

The verifier checks game keys, item category coverage, stackability rules, wiki URL shape, DAOA item coverage, and common mojibake markers.
