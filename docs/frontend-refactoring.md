# Frontend Architecture Status

The original frontend refactor is complete enough that this file is now a status note, not a plan. For the current system map, see `docs/architecture.md`.

## Current Shape

- `frontend/src/App.tsx` is a small orchestrator.
- Feature state is split into hooks under `frontend/src/features/*`.
- Reusable UI lives under `frontend/src/components/*`.
- Pure helpers live under `frontend/src/lib/*`.
- Vitest covers hooks, components, planners, utility functions, and API contract behavior.
- Playwright smoke tests exercise main workflows against the mocked Tauri backend.

## Remaining Opportunities

- Keep command/result narrowing close to API call sites with `expectResult`.
- Add focused component tests when UI behavior branches by game, dirty state, or validation errors.
- Avoid introducing a global state library unless local hooks become a real maintenance burden.
- Keep CSS split by purpose and feature; do not move to a component library without a separate design decision.

## Guardrails

- Preserve Apply/Reset semantics.
- Preserve DAO, DAO Awakening, and DA2 visibility rules.
- Keep mocked backend behavior realistic enough for smoke tests to catch broken user flows.
- Run `npm run check` and `npm run smoke` after frontend workflow changes.
