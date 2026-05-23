---
name: testing-and-coverage
description: Use when adding tests, changing verification commands, extending smoke coverage, assessing coverage, or deciding where a Dragon Age Save Editor behavior should be tested.
metadata:
  short-description: Dragon Age Save Editor test strategy
---

# Testing And Coverage

## Start Here

- Read `docs/testing.md` for the current verification and coverage strategy.
- Read `docs/fixtures.md` before using local sample saves.
- For save mutation tests, also read `skills/editing-patterns/SKILL.md`.
- For command/DTO tests, also read `skills/command-contract/SKILL.md`.

## Test Placement

- Rust parser/extractor behavior: `cargo test` under `src/gff4`, `src/domain`, or `src/app`.
- Rust editor mutation behavior: test raw/domain state and write/reload persistence.
- Frontend pure helpers and hooks: Vitest tests beside the source file.
- Frontend command planners: Vitest planner tests.
- Main user workflows: Playwright smoke tests under `verification/smoke/` with mocked Tauri backend.
- Game data rules: verifier updates plus `npm run data:verify`.

## Coverage

- Run `npm run coverage` when assessing broad test health.
- Frontend coverage writes to `coverage/frontend`.
- Rust coverage writes to `coverage/rust`.
- Treat coverage as a signal, not the only quality gate.

## Verification

- Main gate: `npm run verify`.
- Smoke workflows: `npm run smoke`.
- Frontend-only gate: `npm run check`.
- Rust-only gate: `cargo test` and `cargo check`.

## Guardrails

- Do not mutate original save fixtures in place.
- Prefer focused tests around behavior and contracts over brittle UI details.
- Add write/reload tests for any persisted save edit.
- Update the mocked backend when smoke-visible command behavior changes.
