# v1.0 Release Checklist

The path from current `main` to a public GitHub Release. Decisions captured from the 2026-05-18 grilling session, corrected after surfacing the `ingame/` test suite.

## Release shape

- **Distribution:** Public GitHub Release with a Windows Tauri bundle attached.
- **Gating:** Quality before distribution. Track A must pass before Track B starts.
- **Signing:** Unsigned for v1.0. SmartScreen behavior documented in the README. Revisit at v1.1.
- **Platforms:** Windows only for v1.0.
- **License:** MIT.

## How verification actually works in this project

- **`cargo test` / `npm run verify`** — automated unit + write/reload + smoke (mocked backend).
- **`npm run ingame-test -- <spec>`** — Playwright against the real desktop UI + the `apply_edit` sidecar, driven against a real save at `$env:DAO_SAVE`. Each spec ends in an injected pass/fail panel that waits indefinitely for the user to launch the game, eyeball the change, and click. Existing specs cover stats, abilities, inventory, properties, backpack-ops, companion, combo — all DAO-family.
- The manual QA checklists in `docs/manual-testing.md` and `docs/tauri-manual-qa-checklist.md` are **stale** and not part of the release gate.

## Track A — Quality (release-blocker)

### A1. Close roadmap write/reload test gaps

Source: `docs/roadmap.md` near-term list. Scope corrected: DAO/DaoAwakening items do not carry `SAVEGAME_ITEM_LEVEL`, so item-level roundtrip is a DA2-only concept.

- [x] Write/reload test: DAO backpack metadata (item_cost, material). `src/edit/editor/tests.rs::write_reload_backpack_metadata_edit`.
- [x] Write/reload test: DA2 backpack metadata (item_cost, material, item_level). `src/edit/editor/tests.rs::write_reload_da2_backpack_metadata_edit`.
- [x] Write/reload test: DA2 item property power float-bitcast roundtrip on an existing property. `src/edit/editor/tests.rs::write_reload_da2_item_property_power_roundtrip`.
- [x] Write/reload test: DA2 plot flag integer cleared back to zero. `src/edit/editor/tests.rs::write_reload_da2_plot_flag_integer_cleared_to_zero`. (Existing `write_reload_da2_plot_flag_edit` covers the non-zero/boolean set/clear cases.)

### A2. Promote automatable smoke gaps into the test suite

- [ ] Smoke: DA2 commit/reset walkthrough mirroring the DAO flow in `smoke/app.smoke.spec.ts`.
- [ ] Smoke: Save As → reload roundtrip with the mocked backend.

### A3. In-game verification on real saves

Use the existing `ingame/` suite. Each spec drives the real UI, writes through the sidecar, then waits for human pass/fail after launching the game.

**DAO + Awakening:**

- [ ] Run every `ingame/*.spec.ts` against a DAO vanilla save with the prerequisites documented in `ingame/README.md`. All PASS.
- [ ] Run the suite against an Awakening-style save where prerequisites permit. (Some specs may be DAO-vanilla-only because of named companions/items — those are skipped, not failed.)

**DA2 — net-new work, locked into v1.0 scope:**

DA2 currently has zero `ingame/` coverage. Build the minimum DA2 set, chosen for differential risk surface (the things that aren't a re-skin of DAO).

- [x] **Helpers refactor.** `ingame/helpers.ts` now reads either `DAO_SAVE` or `DA2_SAVE`, errors if neither or both are set, and exposes `SAVE_PATH` to specs. `prereq.da2Save()` mirrors `prereq.daoFamilySave()`. `ingame/README.md` updated.
- [x] **`ingame/da2-stats.spec.ts`** — set level + a core stat + money on the main character. *Skeleton landed; needs a real DA2 save and a human in-game PASS to fully close.*
- [ ] **`ingame/da2-properties.spec.ts`** — edit a property power on an equipped item. This is the float-bitcast verification — the highest-risk DA2-specific encoding path. Verify the displayed value in-game matches.
- [ ] **`ingame/da2-plot-flags.spec.ts`** — set one boolean and one integer plot flag, save, verify the game reflects the change. The only in-game verification of the DA2-exclusive plot-flag editor.
- [ ] **`ingame/da2-combo.spec.ts`** — multi-feature roundtrip combining stats + a property edit + a plot flag. Confirms the integrated save still loads.

Acceptance: each new spec runs end-to-end against a real DA2 save with PASS recorded.

### A4. Cleanup and known bugs found while planning

- [x] Item Level input is now hidden for DAO/DaoAwakening — only rendered when `preferred_game === "da2"`. Driven by a new `canEditItemLevel` flag on the inventory panel state, paralleling `canEditMaterial`.
- [x] Item Cost display removed from `ItemEditor.tsx` for v1.0. Rust DTO and command surface kept intact (no contract break); the UI no longer surfaces it.
- [x] **Bug — DA2 item-properties clear-then-add fails the writer.** Fixed in `src/edit/internal.rs`: the empty-list defaults for DA2 `ITEM_PROPERTIES` and `ITEM_PROPERTY_POWERS` were `Float32`/`UInt32`, but the actual DA2 GFF4 schema declares both as `Int32` (with powers being `f32.to_bits()` reinterpreted into `i32`). Now defaults to `Int32` for both. Regression: `write_reload_da2_item_property_clear_and_add`. The previously misleading unit assertion `da2_added_item_property_uses_float_property_id_storage` was renamed and corrected to `da2_added_item_property_uses_int32_storage_with_float_bitcast_power`.

## Track B — Distribution (after Track A passes)

### B1. User-facing `README.md` at repo root

One page, ~3-minute read. Sections:

- What it is.
- Supported games and variants.
- "Always work on a copy" safety warning.
- Walkthrough: open → edit → apply → save as → copy back.
- Known limitations per game variant.
- Troubleshooting (including the Windows SmartScreen warning).
- Where to file issues.
- License pointer.

### B2. `LICENSE` file

MIT, with the author's name and year.

### B3. Tauri bundle polish

- [ ] App icon present at every required size under `src-tauri/icons/`.
- [ ] `tauri.conf.json`: `identifier`, `version`, `productName`, `publisher` set for release.
- [ ] Version bump to `1.0.0` in `Cargo.toml`, `package.json`, `src-tauri/Cargo.toml`, `tauri.conf.json`.

### B4. Build the release artifact

- [ ] `npm run tauri build` produces a `.msi` (and/or `.exe`).
- [ ] Install on a clean Windows user account and execute the full open → edit → apply → save-as workflow once.

### B5. GitHub Release

- [ ] Tag `v1.0.0`.
- [ ] Upload installer artifact.
- [ ] Release notes: one-paragraph summary + "Known limitations" section.

## Deferred to v1.1+

Explicitly out of scope for v1.0, captured here so the punch list stays honest:

- Code signing (OV or EV certificate).
- Auto-update mechanism.
- GitHub Actions release workflow.
- macOS and Linux bundles.
- `CHANGELOG.md`.
- Telemetry / crash reporting.
- Refresh or retire `docs/manual-testing.md` and `docs/tauri-manual-qa-checklist.md` — currently stale; superseded by `ingame/` + automated suite. Either prune them or rewrite them to match what's actually verified today.
- TypeScript-from-Rust DTO generation, broader fixture coverage, native Tauri dialog test harness (all retained in `docs/roadmap.md` "Possible Later Work").
