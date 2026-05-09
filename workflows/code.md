# Workflow: Code

> **When to use:** Implementing an approved feature or fix.
> A plan must exist and be approved before starting (unless it's a trivial one-liner with clear acceptance criteria).
> For bugs, run `workflows/diagnose.md` first.

---

## Rules

- Read the relevant skill file before touching any code.
- Follow existing patterns in the codebase — don't introduce new idioms without CTO approval.
- Solve the current problem — don't add scope or anticipate hypothetical future requirements.
- Preserve the dual-sync invariant: every `SaveEditor` edit updates both `raw` and `save`.
- Do not mutate original save fixtures — always work on copies.
- Do not add a dependency without flagging to the CTO first.

---

## Process

### Step 1 — Re-read context

Before touching any file:
- The approved plan — scope, files to change, acceptance criteria
- The relevant skill files for the area
- The specific files you're about to edit

### Step 2 — Rust work (if applicable)

1. Locate the correct field by constant from `src/gff4/fields.rs` — no magic numbers.
2. Validate type, game support (`GameId`), and numeric range before mutating.
3. Use the existing helper style from `src/edit/internal.rs`.
4. Update both `raw` (GFF tree) and `save` (domain model).
5. Return typed errors from `EditError` / `CommandError`.
6. Add a write/reload test for any edit that persists to disk.

### Step 3 — Frontend work (if applicable)

1. Read `skills/frontend-workflows/SKILL.md` and `skills/command-contract/SKILL.md`.
2. Preserve Apply/Reset semantics.
3. Update `frontend/src/test/mockBackend.ts` if smoke-visible command behavior changes.
4. Handle every command result shape — use `expectResult` when a specific variant is expected.
5. Add Vitest coverage for new hooks, planners, or pure helpers.
6. Add smoke coverage if a main user path changes.

### Step 4 — Game data work (if applicable)

1. Edit seed CSVs under `data/seeds/` — never edit `gamedata.db` directly.
2. Run `npm run data:build` to regenerate the DB.
3. Run `npm run data:verify` to check integrity.
4. Update verifier rules if categories or stackability rules change.

### Step 5 — Self-review before handing off

- [ ] `npm run verify` passes
- [ ] New Rust logic has tests (extraction unit + write/reload for persistence)
- [ ] New frontend logic has Vitest coverage
- [ ] Main user path changes have smoke coverage
- [ ] No original save fixtures were mutated
- [ ] No hardcoded field IDs — all constants from `src/gff4/fields.rs`
- [ ] Docs updated where needed (architecture.md, command-contract.md, testing.md, skill files)

### Step 6 — Report

```
## [DEV:rust / DEV:frontend] [Feature / Task]

**Implemented:** [1–2 sentence summary]

**Files changed:**
- `path/to/file` — [what changed]

**Tests added:**
- `path/to/test` — [what it covers]

**Verification:** [how to confirm it works]

**Blockers:** [anything needing CTO or Founder input — or "none"]
```

---

## What NOT to Do

- Don't start without an approved plan for non-trivial changes.
- Don't skip the write/reload test for any persistent Rust edit.
- Don't mutate original save files.
- Don't hardcode GFF4 field IDs — use constants.
- Don't let unsupported game behavior fail silently.
- Don't mark done until `npm run verify` passes.
