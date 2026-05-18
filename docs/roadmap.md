# Roadmap And Known Gaps

This is the current working roadmap. Historical completed refactors live in `docs/refactoring.md` and `docs/frontend-refactoring.md`.

For the path to v1.0, see `docs/release-checklist.md`.

## Near-Term

- Expand write/reload tests around item metadata, item property power encoding, and DA2 plot flags. (Tracked in `docs/release-checklist.md` A1.)
- Keep smoke tests aligned with main user workflows when UI flow changes.
- Refresh manual QA checklist statuses after each substantial desktop UI pass.

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
