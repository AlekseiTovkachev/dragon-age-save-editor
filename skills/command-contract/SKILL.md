---
name: command-contract
description: Use when adding, changing, testing, or debugging SaveCommand, SaveCommandResult, DTOs, CommandError, frontend TypeScript command types, mock backend behavior, or Tauri invoke wiring.
metadata:
  short-description: Rust/TypeScript command contract
---

# Command Contract

## Start Here

- Read `docs/command-contract.md` for the full contract workflow.
- Check Rust source in `src/app/commands.rs`, `src/app/dto.rs`, and `src/app/errors.rs`.
- Check frontend mirrors in `frontend/src/types.ts`, `frontend/src/api.ts`, and `frontend/src/test/mockBackend.ts`.

## Contract Rules

- Rust `SaveCommand` is serialized with the `command` tag.
- Rust `SaveCommandResult` is serialized with the `result` tag.
- TypeScript unions in `frontend/src/types.ts` must mirror the Rust JSON shape.
- Frontend callers should use `expectResult` when they require a specific result variant.
- Tests should assert stable error codes instead of matching full message text.

## Change Checklist

1. Update Rust command/result enums.
2. Update DTO conversions and command execution.
3. Update TypeScript command/result unions.
4. Update the mocked backend for smoke-visible behavior.
5. Update `frontend/src/api.contract.test.ts`.
6. Add Rust command tests for success and important failures.
7. Add frontend hook/planner/smoke coverage for visible workflows.

## Guardrails

- Keep dirty-state, summary, and saved-state behavior explicit after mutations.
- Prefer `apply_batch` for multi-step UI commits.
- Do not add frontend-only command shapes.
