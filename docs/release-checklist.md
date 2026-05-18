# v1.0 Release Checklist

The path from current `main` to a public GitHub Release. Decisions captured from the 2026-05-18 grilling session.

## Release shape

- **Distribution:** Public GitHub Release with a Windows Tauri bundle attached.
- **Gating:** Quality before distribution. Track A must pass before Track B starts.
- **Signing:** Unsigned for v1.0. SmartScreen behavior documented in the README. Revisit at v1.1.
- **Platforms:** Windows only for v1.0.
- **License:** MIT.

## Track A — Quality (release-blocker)

### A1. Close roadmap write/reload test gaps

Source: `docs/roadmap.md` near-term list.

- [ ] Write/reload test: item metadata edits.
- [ ] Write/reload test: item property power encoding (covers DA2 float-bitcast roundtrip — also clears `docs/tauri-manual-qa-checklist.md` line 53).
- [ ] Write/reload test: DA2 plot flags.

Acceptance: every persistent edit has a roundtrip test for at least one DAO fixture and one DA2 fixture.

### A2. Promote automatable QA-checklist items into the test suite

Source: `docs/tauri-manual-qa-checklist.md` "Commit And Reset Model" and "Save As And Reload" sections.

- [ ] Smoke: DA2 commit/reset walkthrough mirroring the DAO flow in `smoke/app.smoke.spec.ts`.
- [ ] Smoke: Save As → reload roundtrip with the mocked backend.
- [ ] Confirm `src/app/tests.rs::save_as_writes_new_file_and_keeps_original_unchanged` (or sibling test) covers DA2, not just DAO.

Acceptance: the only items remaining on the manual checklist are inherently visual or responsive.

### A3. In-game roundtrip on real hardware

The irreplaceable check — no automated test proves the game engine accepts the save.

- [ ] DAO vanilla: edit a visible value, launch the game, verify in-game, play 5–10 minutes.
- [ ] One Awakening-style campaign (Awakening proper preferred): same.
- [ ] DA2: same.

Acceptance: zero unresolved bugs from this pass.

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
- TypeScript-from-Rust DTO generation, broader fixture coverage, native Tauri dialog test harness (all retained in `docs/roadmap.md` "Possible Later Work").
