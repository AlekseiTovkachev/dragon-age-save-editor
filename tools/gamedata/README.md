# Game Data Pipeline

`data/gamedata.db` is generated from `data/schema.sql` and the CSV files in `data/seeds`. Item and ability seeds are split by game so DAO, DAOA, and DA2 catalog changes are easy to review.

Run:

```powershell
npm run data:build
npm run data:verify
python tools/gamedata/verify_gamedata.py --report
```

The verification script checks game keys, item categories, stackability rules, wiki URL shape, DAOA coverage, seed/DB row counts, and common mojibake markers.
