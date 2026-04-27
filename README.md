# Dragon Age Save Editor

A local save editor for Dragon Age: Origins, Awakening-style DAO saves, and Dragon Age II.

Supported save families:

- DAO vanilla saves.
- DAO Awakening-style saves, including Awakening, Witch Hunt, and Golems of Amgarrak.
- DA2 saves.

The app uses `data/gamedata.db` to enrich raw save data with item names, item categories, material metadata, ability metadata, item property names, and stackability rules.

## Development

Run the frontend build:

```bash
npm run build
```

Run frontend checks:

```bash
npm run typecheck
npm run lint
npm run check
```

Run browser smoke tests with the mocked Tauri backend:

```bash
npm run smoke
```

Run the full project verification gate:

```bash
npm run verify
```

Generate coverage reports:

```bash
npm run coverage
```

Frontend reports are written under `coverage/frontend`; Rust reports are written under `coverage/rust`.

Run Rust checks:

```bash
cargo test
cargo check
cd src-tauri
cargo check
```

Rebuild and verify game data:

```bash
npm run data:build
npm run data:verify
```

## Safety

Always edit copies of game saves. Keep the original `.das`, `.das.met`, and `screen.dds` files until the edited save has been loaded and checked in game.
