---
name: frontend-workflows
description: Use when changing React panels, hooks, command planners, Apply/Reset behavior, dirty state, game-specific visibility, or smoke-visible UI workflows in Dragon Age Save Editor.
metadata:
  short-description: Frontend workflow and draft-state guidance
---

# Frontend Workflows

## Start Here

- Read `docs/architecture.md` for frontend ownership.
- Read `docs/frontend-refactoring.md` for current frontend status and guardrails.
- For command calls, also read `skills/command-contract/SKILL.md`.
- For smoke tests, also read `skills/testing-and-coverage/SKILL.md`.

## Structure

- `frontend/src/App.tsx`: shell orchestration.
- `frontend/src/features/*`: feature hooks, panels, and workflow state.
- `frontend/src/components/*`: reusable UI components.
- `frontend/src/lib/*`: pure helpers.
- `frontend/src/test/mockBackend.ts`: smoke-test backend behavior.

## Workflow Rules

- Preserve Apply/Reset semantics.
- Reset returns drafts to the latest committed state, not necessarily the originally loaded file.
- Successful commits should refresh dirty state, summaries, and relevant feature data.
- DAO-family and DA2 visibility rules must stay explicit.
- Unknown or unsupported save values should remain visible where safe.

## Test Expectations

- Hook and planner changes need Vitest coverage.
- Main user-path changes need smoke coverage.
- Mock backend behavior should mutate state realistically enough to catch broken UI flows.

## Guardrails

- Avoid introducing global state unless local feature hooks become a real maintenance burden.
- Keep UI text and controls consistent with the existing app rather than adding explanatory landing-page content.
- Do not let unsupported DA2/DAO behavior fail silently.
