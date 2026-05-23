# Roadmap And Known Gaps

This is the current working roadmap. Historical completed refactors live in `docs/refactoring.md` and `docs/frontend-refactoring.md`.

For the current public beta release path, see `docs/release-checklist.md`.

## Near-Term

- Finish the `v0.9` public beta release: clean install smoke, tag, installer upload, release notes.
- Rework DAO-family in-game specs so they can run meaningfully against arbitrary DAO/Awakening-style saves.
- Keep smoke tests aligned with main user workflows when UI flow changes.

## v1.0 Candidates

- Decide whether `v1.0` needs broader real-save coverage beyond the current `v0.9` gate.
- Revisit code signing after the first public beta feedback.
- Consider a changelog once releases become recurring instead of one-off.

## Structure Cleanup After v0.9

- Split large Rust test modules, especially `src/app/tests.rs` and `src/edit/editor/tests.rs`, by feature area.
- Break up the large plot flag panel into smaller view components without changing behavior.
- Consider archiving completed refactor notes under `docs/archive/`.
- Revisit whether root-level Node/Rust/Tauri config files should be documented in one short repository map.

## Documentation

- Keep `docs/architecture.md` current when modules move.
- Keep `docs/testing.md` current when verification commands or coverage tooling changes.
- Keep repo skills under `skills/*/SKILL.md` concise and first-class.
- Keep skill metadata in `skills/*/agents/openai.yaml` aligned with the corresponding `SKILL.md`.

## Possible Later Work

- Generate TypeScript command/result types from Rust DTOs or add a stricter schema check.
- Add broader fixture coverage for edge-case saves.
- Add native Tauri dialog coverage if the test harness starts running against the desktop shell.
- Revisit frontend state management only if local feature hooks become difficult to maintain.
