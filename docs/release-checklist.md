# v0.9 Release Checklist

The path from current `main` to a public beta GitHub Release. Decisions captured from the 2026-05-18 grilling session, corrected after surfacing the `verification/ingame/` test suite.

## Release shape

- **Distribution:** Public beta GitHub Release with a Windows Tauri bundle attached.
- **Gating:** Quality before distribution. Track A must pass before Track B starts.
- **Signing:** Unsigned for v0.9. SmartScreen behavior documented in the README. Revisit at v1.0.
- **Platforms:** Windows only for v0.9.
- **License:** MIT.

## How verification actually works in this project

- **`cargo test` / `npm run verify`** — automated unit + write/reload + smoke (mocked backend).
- **`npm run ingame-test:dao` / `npm run ingame-test:da2`** — Playwright against the real desktop UI + the `apply_edit` sidecar, driven against `DAO_SAVE` or `DA2_SAVE` respectively. Each spec ends in an injected pass/fail panel that waits indefinitely for the user to launch the game, eyeball the change, and click. Use `npm run ingame-test -- <spec>` for a single spec; if both `DAO_SAVE` and `DA2_SAVE` are set, the generic runner requires `INGAME_GAME=dao` or `INGAME_GAME=da2`.
- `docs/manual-qa.md` is a small packaging smoke check only. Automated tests and `verification/ingame/` remain the real quality gate.

## Track A — Quality (release-blocker)

### A1. Close roadmap write/reload test gaps

Source: `docs/roadmap.md` near-term list. Scope corrected: DAO/DaoAwakening items do not carry `SAVEGAME_ITEM_LEVEL`, so item-level roundtrip is a DA2-only concept.

- [x] Write/reload test: DAO backpack metadata (item_cost, material). `src/edit/editor/tests.rs::write_reload_backpack_metadata_edit`.
- [x] Write/reload test: DA2 backpack metadata (item_cost, material, item_level). `src/edit/editor/tests.rs::write_reload_da2_backpack_metadata_edit`.
- [x] Write/reload test: DA2 item property power float-bitcast roundtrip on an existing property. `src/edit/editor/tests.rs::write_reload_da2_item_property_power_roundtrip`.
- [x] Write/reload test: DA2 plot flag integer cleared back to zero. `src/edit/editor/tests.rs::write_reload_da2_plot_flag_integer_cleared_to_zero`. (Existing `write_reload_da2_plot_flag_edit` covers the non-zero/boolean set/clear cases.)

### A2. Promote automatable smoke gaps into the test suite

- [x] Smoke: DA2 commit/reset walkthrough — already covered by `verification/smoke/app.smoke.spec.ts` "resets, commits, and saves DA2 plot flag drafts" at line 150. Full reset → commit → save-as roundtrip on DA2 plot flags.
- [x] Save As → reload roundtrip — covered by the real-serialization path (Rust write/reload tests in `src/edit/editor/tests.rs` and the `verification/ingame/` suite via the `apply_edit` sidecar). Adding a mock-backend reload would require snapshotting `save_as` state for re-open in the mock and would only verify React re-hydration, which every `openMockSave`-based smoke spec already exercises. Net low marginal value; not pursued.

### A3. In-game verification on real saves

Use the existing `verification/ingame/` suite. Each spec drives the real UI, writes through the sidecar, then waits for human pass/fail after launching the game.

**DAO + Awakening:**

- [x] Run every `verification/ingame/dao/*.spec.ts` against a DAO vanilla save with the prerequisites documented in `verification/ingame/README.md`. All PASS.
- [x] Awakening-style in-game suite run removed from v0.9 gate. The current DAO-family specs are highly save-specific (named DAO companions/items and exact inventory prerequisites), so running them against Awakening-style saves has low signal unless the suite is rebuilt to work against arbitrary saves.

**DA2 — net-new work, locked into v0.9 scope:**

DA2 now has the minimum `verification/ingame/` coverage chosen for differential risk surface (the things that aren't a re-skin of DAO).

- [x] **Helpers refactor.** `verification/ingame/helpers.ts` now reads `DAO_SAVE` for the DAO folder script and `DA2_SAVE` for the DA2 folder script. The generic runner accepts exactly one save env var, errors if neither is set, and errors if both are set without an explicit `INGAME_GAME`. `SAVE_PATH` is exposed to specs, `prereq.da2Save()` mirrors `prereq.daoFamilySave()`, and `verification/ingame/README.md` is updated.
- [x] **`verification/ingame/da2/stats.spec.ts`** — set level + a core stat + money. Verified in-game (level + money exact; strength shows base+gear bonus, expected).
- [x] **`verification/ingame/da2/properties.spec.ts`** — add a property to an equipped item. Verified in-game: the added property roundtrips and appears on the item. Note: DA2 rescales the raw power value (power 25 → ~+315 displayed), so the spec verifies presence, not the literal number.
- [x] **`verification/ingame/da2/abilities.spec.ts`** — add Lacerate/Murder to Hawke, add Walking Bomb/Death Vortex to Anders, and set Anders approval to -10. Verified in-game.
- [x] **`verification/ingame/da2/plot-flags.spec.ts`** — set one boolean and one integer plot flag, save, verify the game reflects the change. Verified via the Nexus worldstate inspection mod; codex entries may not refresh after first unlock, so codex text is not a reliable verification surface for edited imported worldstate.
- [x] **`verification/ingame/da2/combo.spec.ts`** — multi-feature roundtrip combining stats + a property edit + a plot flag. Verified in-game; plot flag uses "Andraste's ashes revealed" -> No for an easy observable edit.

Acceptance: each new spec runs end-to-end against a real DA2 save with PASS recorded.

### A3a. DA2 in-game issues found and fixed

- [x] **In-game Save As helper could pass too early.** `verification/ingame/saveAs()` was waiting on stale "Saved copy ready" sidebar text, so tests could read the original file before the sidecar copied the working save back. Fixed by waiting for the actual `save_as` HTTP response.
- [x] **DA2 sidecar screenshot output exceeded the default Node stdout buffer.** `get_document_assets` can return a multi-megabyte base64 `screen.dds`; `tools/ingame-server.mjs` now raises `spawnSync` `maxBuffer`.
- [x] **DA2 ability replacement rejected valid saves with existing odd IDs.** Hawke's save had existing unknown/internal ability IDs and a known ability with a missing catalogued core. Replacing the list to add Lacerate/Murder revalidated those old entries and failed. Fixed by preserving existing ability IDs as-is while still validating newly added IDs and their dependencies.

### A5. Latent product bug — active item-editor draft not flushed before global Apply

- [x] When an item-editor draft (e.g. a freshly added property) is created and **Apply Drafts** is clicked with no intervening context switch, the global apply must include it. `useInventoryEditor.planCommands()` now forces `storeCurrentItemDraft()` before reading `itemDrafts.current`, so the live editor draft is flushed before global Apply / Save As pending-draft checks plan commands. Covered by `frontend/src/features/inventory/useInventoryEditor.test.tsx` ("plans stack size, metadata, and property mutations in order").

### A6. Plot flags — no cross-decision contradiction validation

- [x] The DA2 plot-flags panel validates *within* exclusive groups and now covers the cross-decision rules from `docs/dao_da2_decision_tables_tracker.md` as visible `PlotWarningsPanel` entries. Concrete cases found while building `verification/ingame/da2/plot-flags.spec.ts`:
  - **Race vs. origin** — race (`1001`) and the Origin group (`2000`–`2005`) are independent controls; race = Elf with a Human Noble origin is accepted. Each origin carries a required race (tracker section 10).
  - **Race vs. political marriage** — a Warden consort outcome (`2024` / `2026`, Landsmeet group) requires `1001 = 3` (human) and `2005 = 1` (human noble). Changing the Warden to an elf silently contradicts an existing consort flag.
  - These are examples, not the full list — the tracker enumerates Landsmeet, companion, Isabela, Warden's Keep, and Awakening cross-rules too.
  - Fixed for v0.9: `plotFlagValidation.ts` / `src/app/plot_flag_rules.rs` warn on origin/race mismatches, political marriage identity overrides, Landsmeet contradictions, companion prerequisites, Isabela prerequisites, Warden's Keep contradictions, and Awakening contradictions. `Apply Drafts` is disabled while DA2 plot warnings are present.
- [x] **Silent reversion via implications — no user feedback, and order-dependent.** Implications run in the `PatchPlotFlags` backend handler (`apply_implications`), but the frontend no longer re-runs implications on every boolean/exclusive-group click. A prince-consort outcome (`2024`/`2026`) still forces the Warden to a human noble at commit time, but the draft UI now keeps direct user edits visible and warns that the identity would be forced instead of snapping values back during later clicks. Two problems:
  - **Silent.** A user can pick race = Elf, and it snaps back to Human with no message.
  - **Order-dependent.** Because the frontend re-runs implications per click, editing identity *before* clearing the consort outcome gets reverted on the next exclusive-group interaction; editing it *after* works. The same set of changes succeeds or fails purely on click order. Surfaced debugging `verification/ingame/da2/plot-flags.spec.ts` — the spec now clears the Landsmeet consort outcome first as a workaround.
  - Fixed for v0.9: implication overrides are surfaced through `PlotWarningsPanel`, frontend click handling is order-independent, and backend implications remain the final save-time normalization path.
- [x] **Bug (fixed) — multi-word plot sections had no accessible name.** `PlotSectionCard` built the heading `id`/`aria-labelledby` from the raw section title. `aria-labelledby` is a space-separated IDREF list, so multi-word titles ("Nature of the Beast", "Broken Circle", "Arl of Redcliffe") resolved to nonexistent IDs — those `<section>`s got no accessible name and were not exposed as `region` landmarks (screen readers would not announce them). Single-word sections worked by accident. Fixed by slugifying the title into a single-token id. Surfaced because `verification/ingame/da2/plot-flags.spec.ts` could not locate the "Nature of the Beast" region.

### A4. Cleanup and known bugs found while planning

- [x] Item Level input is now hidden for DAO/DaoAwakening — only rendered when `preferred_game === "da2"`. Driven by a new `canEditItemLevel` flag on the inventory panel state, paralleling `canEditMaterial`.
- [x] Item Cost display removed from `ItemEditor.tsx` for v0.9. Rust DTO and command surface kept intact (no contract break); the UI no longer surfaces it.
- [x] **Bug — DA2 item-properties clear-then-add fails the writer.** Fixed in `src/edit/internal.rs`: the empty-list defaults for DA2 `ITEM_PROPERTIES` and `ITEM_PROPERTY_POWERS` were `Float32`/`UInt32`, but the actual DA2 GFF4 schema declares both as `Int32` (with powers being `f32.to_bits()` reinterpreted into `i32`). Now defaults to `Int32` for both. Regression: `write_reload_da2_item_property_clear_and_add`. The previously misleading unit assertion `da2_added_item_property_uses_float_property_id_storage` was renamed and corrected to `da2_added_item_property_uses_int32_storage_with_float_bitcast_power`.

## Track B — Distribution (after Track A passes)

### B1. User-facing `README.md` at repo root

- [x] One page, ~3-minute read. Sections:

- What it is.
- Supported games and variants.
- "Always work on a copy" safety warning.
- Walkthrough: open → edit → apply → save as → copy back.
- Known limitations per game variant.
- Troubleshooting (including the Windows SmartScreen warning).
- Where to file issues.
- License pointer.

### B2. `LICENSE` file

- [x] MIT, with the author's name and year.

### B3. Tauri bundle polish

- [x] Desktop bundle icon set present under `src-tauri/icons/`.
- [x] `tauri.conf.json`: `identifier`, `version`, `productName`, `publisher` set for release.
- [x] Version bump to `0.9.0` in `Cargo.toml`, `package.json`, `src-tauri/Cargo.toml`, `tauri.conf.json`.

### B4. Build the release artifact

- [x] `npm run tauri build` produces a `.msi` (and/or `.exe`) for `0.9.0`. Produced `src-tauri/target/release/bundle/msi/Dragon Age Save Editor_0.9.0_x64_en-US.msi` and `src-tauri/target/release/bundle/nsis/Dragon Age Save Editor_0.9.0_x64-setup.exe`.
- [x] Run `docs/manual-qa.md` packaging smoke on a clean Windows user account. Both the `.msi` and `.exe` installers launched successfully, edited a DA2 save, and the edits were verified in game.

### B5. GitHub Release

- [ ] Tag `v0.9.0`.
- [ ] Upload installer artifact.
- [ ] Release notes: one-paragraph summary + "Known limitations" section.

## Deferred to v1.0+

Explicitly out of scope for v0.9, captured here so the punch list stays honest:

- Code signing (OV or EV certificate).
- Auto-update mechanism.
- GitHub Actions release workflow.
- macOS and Linux bundles.
- `CHANGELOG.md`.
- Telemetry / crash reporting.
- Rework DAO-family in-game specs so they can run meaningfully against arbitrary DAO/Awakening-style saves instead of relying on named companions/items and exact inventory prerequisites.
- TypeScript-from-Rust DTO generation, broader fixture coverage, native Tauri dialog test harness (all retained in `docs/roadmap.md` "Possible Later Work").
