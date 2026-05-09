# Workflow: QA

> **When to use:** After implementation is complete and CTO review has passed.
> Also used to design test cases during planning.
>
> See `docs/testing.md` and `skills/testing-and-coverage/SKILL.md` for the authoritative
> test placement rules and coverage strategy. This workflow defines the *process*.

---

## Rules

- Run `npm run verify` first — it's the mandatory gate.
- Do not mutate original save fixtures — always use copies or temp paths.
- Smoke tests stay deterministic — no timing dependencies.
- A bug found in QA is cheaper than one found during manual play.

---

## Test Placement (quick reference)

| What | Where | Tool |
|---|---|---|
| Rust parser/extractor behavior | `src/gff4/`, `src/domain/`, `src/app/` | `cargo test` |
| Rust mutation + persistence | Inline test with write/reload using temp path | `cargo test` |
| Frontend hooks, planners, helpers | Beside the source file | Vitest |
| Command/DTO contract | `frontend/src/api.contract.test.ts` | Vitest |
| Main user workflows | `smoke/` with `VITE_E2E_MOCK=1` | Playwright |
| Game data rules | Verifier + `npm run data:verify` | Python verifier |

See `docs/testing.md` for when each level applies.

---

## Process

### Step 1 — Run the verification gate

```bash
npm run verify
```

All passes before manual QA. Fix failures before proceeding.

### Step 2 — Run smoke tests

```bash
npm run smoke
```

For any UI workflow or command behavior change.

### Step 3 — Manual verification checklist

Work through the feature's acceptance criteria plus:

**Rust behavior:**
- [ ] New extraction behavior has a read-side test
- [ ] New persistent edit has a write/reload test
- [ ] DAO-family and DA2 differences handled explicitly
- [ ] Unknown/missing fields handled gracefully (clear error or documented optional)

**Frontend behavior:**
- [ ] All command result shapes handled
- [ ] Apply/Reset semantics preserved
- [ ] DAO-family and DA2 visibility rules preserved
- [ ] Unknown values visible but safe (not silently dropped)
- [ ] Mock backend updated if smoke-visible behavior changed

**Game data (if changed):**
- [ ] Seed CSVs updated (not DB directly)
- [ ] `npm run data:build` run successfully
- [ ] `npm run data:verify` passes

**Docs:**
- [ ] `docs/architecture.md` updated if module changed
- [ ] `docs/command-contract.md` updated if DTO changed
- [ ] `docs/testing.md` updated if verification commands changed
- [ ] Skill files updated if agent-facing conventions changed

### Step 4 — File bugs

```
**Bug:** [short description]
**Steps to reproduce:** [exact steps]
**Expected:** [what should happen]
**Actual:** [what actually happens]
**Severity:** Critical / High / Medium / Low
**Repro command:** [e.g. npm run smoke / cargo test]
```

### Step 5 — QA report

```
## [DEV-QA] QA Report: [Feature / Sprint]

**Status:** ✅ Pass / ⚠️ Pass with issues / 🔴 Fail

**Gates:**
- npm run verify: ✅ / 🔴
- npm run smoke: ✅ / ⚠️ / 🔴 / N/A
- Manual checklist: ✅ complete / ⚠️ partial

**Bugs found:**
- [Bug description — severity]

**Next:** [ship / fix and re-verify]
```

---

## What NOT to Do

- Don't skip `npm run verify`.
- Don't mutate original save fixtures in tests.
- Don't add flaky timing-dependent smoke tests.
- Don't mark QA complete with open 🔴 bugs.
