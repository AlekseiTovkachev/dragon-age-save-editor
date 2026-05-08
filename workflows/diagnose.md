# Workflow: Diagnose

> **When to use:** A bug has been reported or discovered.
> Reproduce before hypothesizing. Fix minimally. Add a regression test.

---

## Rules

- Reproduce first. If you cannot reproduce it, you cannot be sure you've fixed it.
- Understand before fixing — know *why* it fails before writing a solution.
- Fix minimally. The smallest correct fix is better than a refactor.
- Add a regression test. The fix is not done without a test that would have caught the bug.
- Do not mutate original save fixtures.

---

## Process

### Step 1 — Reproduce

1. Read the bug report. Note exact steps, environment, expected vs actual.
2. Replicate. Can you see the bug?
   - Yes: proceed.
   - No: investigate what differs. Ask for more detail before guessing.
3. Document the exact reproduction steps that work.

### Step 2 — Minimise

- Which specific input triggers it?
- Does it happen in isolation, or only in combination?
- DAO-family only, DA2 only, or both?
- Consistent or intermittent?

### Step 3 — Hypothesise

Before reading code, form a theory:
- Which layer is likely responsible (GFF4 parsing / domain extraction / SaveEditor mutation / command layer / frontend)?
- What could cause the observed difference between expected and actual?

### Step 4 — Instrument

Read the relevant skill file before diving into code:
- `skills/save-format/SKILL.md` — GFF4 parsing or extraction issues
- `skills/editing-patterns/SKILL.md` — mutation or dual-sync issues
- `skills/command-contract/SKILL.md` — DTO or result shape issues
- `skills/frontend-workflows/SKILL.md` — Apply/Reset or dirty-state issues

Trace the execution path. Confirm or refute the hypothesis. Form a new one if refuted.

### Step 5 — Fix minimally

Smallest change that corrects the behavior. No refactoring, no scope creep.

### Step 6 — Add regression test

- **Rust bug:** add a `cargo test` unit test or write/reload test that fails before the fix and passes after.
- **Frontend bug:** add a Vitest test for the failing case.
- **Smoke-visible bug:** add or update a smoke test if the failure is in a main user workflow.

### Step 7 — Verify

- Run the regression test — it must pass.
- Run `npm run verify` — no new failures.
- Manually reproduce the original steps — the bug must not appear.

### Step 8 — Report

```
## [DBG] Bug Fix: [Bug Description]

**Root cause:** [What was actually wrong]
**Layer:** [Rust GFF4 / domain / edit / command / frontend]
**Game scope:** [DAO-family / DA2 / both]

**Reproduction:** [Exact steps]

**Fix:** [What changed and why this fixes it]

**Files changed:**
- `path/to/file` — [what changed]

**Regression test:** `path/to/test` — [what it verifies]

**Verified:**
- [ ] Bug no longer reproduces
- [ ] npm run verify passes
- [ ] No new regressions
```

---

## What NOT to Do

- Don't fix without reproducing.
- Don't guess the root cause without tracing it.
- Don't refactor surrounding code during a bug fix.
- Don't skip the regression test.
- Don't mutate original save fixtures.
- Don't mark done before `npm run verify` is green.
