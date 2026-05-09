# Command Contract

The frontend/backend contract is the tagged JSON shape shared by Rust `SaveCommand` / `SaveCommandResult` and TypeScript `SaveCommand` / `SaveCommandResult`.

## Rust Source Of Truth

- Commands: `src/app/commands.rs`
- DTOs: `src/app/dto.rs`
- Errors: `src/app/errors.rs`
- Tauri bridge: `src-tauri/src/main.rs`

Rust enums use serde tags:

- `SaveCommand`: `{ "command": "snake_case_name", ... }`
- `SaveCommandResult`: `{ "result": "snake_case_name", ... }`

## Frontend Mirror

- Types: `frontend/src/types.ts`
- Invoke wrapper: `frontend/src/api.ts`
- Contract tests: `frontend/src/api.contract.test.ts`
- Smoke mock: `frontend/src/test/mockBackend.ts`

Use `expectResult(response, "result_name")` when a caller expects a specific result variant.

## Mutation Pattern

Most UI workflows send one `apply_batch` command containing smaller save commands. After a successful mutation, the frontend should refresh or consume returned state so committed drafts, summaries, dirty state, and save-as availability stay consistent.

## Adding Or Changing Commands

1. Update Rust `SaveCommand` and `SaveCommandResult`.
2. Update DTO conversion code if needed.
3. Update TypeScript command/result unions.
4. Update `frontend/src/test/mockBackend.ts`.
5. Update `frontend/src/api.contract.test.ts`.
6. Add Rust command tests for success and relevant errors.
7. Add frontend hook/planner tests or smoke tests for user-visible workflows.

## Error Shape

Tauri command failures should serialize to:

```ts
type CommandError = {
  code: CommandErrorCode;
  message: string;
};
```

Prefer stable error codes over matching message text in tests.
