# Testing And Coverage

## Main Commands

```bash
npm run verify
```

Runs the project verification gate:

- frontend typecheck, lint, unit tests, and build via `npm run check`;
- Rust tests via `cargo test`;
- Rust typecheck via `cargo check`;
- Tauri typecheck via `cargo check --manifest-path src-tauri/Cargo.toml`;
- game data verification via `npm run data:verify`.

Other useful commands:

```bash
npm run smoke
npm run coverage
npm run coverage:frontend
npm run coverage:rust
```

## Frontend Tests

- Vitest unit/component tests live beside the code they cover.
- `frontend/src/api.contract.test.ts` checks command/result DTO compatibility assumptions.
- `frontend/src/test/mockBackend.ts` powers Playwright smoke tests when `VITE_E2E_MOCK=1`.

Use frontend tests for hooks, command planners, pure helpers, component states, and API-result narrowing.

## Smoke Tests

Playwright smoke tests live in `verification/smoke/` and exercise primary user workflows through the real React UI with the mocked Tauri backend.

Current smoke coverage should include:

- opening a document;
- editing and committing character/inventory/crafting/plot flag changes;
- reset behavior;
- save-as behavior;
- validation and command failure surfaces.

Smoke tests should stay fast and deterministic. Use Rust tests for binary persistence and real save files.

## Rust Tests

Rust tests should cover:

- GFF4 parsing and write/read roundtrips;
- extraction behavior for DAO-family and DA2 saves;
- `SaveEditor` mutations, including write/reload persistence;
- app command behavior and error mapping;
- validation rules.

When editing saves in tests, write temporary copies rather than mutating fixtures in place.

## Coverage

```bash
npm run coverage
```

Frontend coverage writes to `coverage/frontend`. Rust coverage writes to `coverage/rust`.

`tools/rust-coverage.mjs` uses `cargo llvm-cov` and intentionally ignores `src/gff4/fields.rs`, which is mostly generated-style field constants. Treat coverage as a signal for untested behavior, not a release gate by itself.

## When To Add Tests

- New parser/extractor behavior: add read-side Rust tests.
- New editor mutation: add raw/domain assertion and write/reload coverage.
- New frontend workflow: add hook/planner tests and one smoke test if it is a main user path.
- New command/result shape: update frontend contract tests.
- New gamedata category or rule: update seed data tests and `npm run data:verify`.
