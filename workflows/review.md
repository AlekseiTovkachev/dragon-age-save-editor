# Workflow: Review

> **When to use:** When the dev body declares work done and it needs the CTO quality gate.
> Also used for mid-sprint code review or architecture review.
>
> *AI reviews AI — never ship a first draft.*

---

## Rules

- The CTO reviews and flags. Doesn't silently rewrite.
- A 🔴 Ugly finding blocks the feature from shipping. Fix or Founder defers it explicitly.
- Use `docs/code-review-checklist.md` for domain-specific checks — it is the authoritative checklist for this project. This workflow defines the *process*; that checklist defines the *criteria*.
- One fix-iteration round maximum; after that, Founder decides ship-or-defer.

---

## Rating Scale

| Rating | Meaning | Action |
|---|---|---|
| ✅ Good | Correct, rule-adherent, tests present | Acknowledge |
| ⚠️ Bad | Weak pattern, fixable this sprint | P1 — flag for fix |
| 🔴 Ugly | Blocks ship — invariant violated, missing required test, unsafe behavior | P0 — fix or escalate |

---

## Process

### Step 1 — Read context

- The approved plan / sprint task and acceptance criteria
- `AGENTS.md` — Definition of Done
- `CONTEXT.md` — domain terminology to stay accurate
- The changed files

### Step 2 — Work through `docs/code-review-checklist.md`

This checklist is the domain-specific quality gate. Work through every applicable section:
- **Save Parsing** — correct field labels, DAO/DA2 differences, missing-field handling, read-side test
- **Save Mutation** — raw + domain updated, unrelated fields preserved, write/reload test
- **Game Data** — seed files updated (not DB directly), verifier rules current, `data:build` + `data:verify` run
- **Frontend** — all result shapes handled, `expectResult` used, Apply/Reset preserved, DAO/DA2 visibility rules kept, mock backend updated
- **Verification** — `npm run verify` + `npm run smoke` (for workflow/UI changes) + `npm run coverage` (if assessing health) + `npm run data:build` (if seed/schema changed)
- **Documentation** — architecture.md, command-contract.md, testing.md, skill files updated where applicable

### Step 3 — Check general quality

Beyond the domain checklist:
- Does the change follow existing patterns in the codebase?
- Are there any magic numbers where constants should be used?
- Are errors typed (`EditError`, `CommandError`) — no string-only failure paths?
- Are unknown or unsupported values handled visibly rather than silently?
- Was any decision made that should have been escalated (irreversible calls made quietly)?

### Step 4 — Write the review

```
## [CTO] Review: [Feature / Task Name]

**Overall:** ✅ Good / ⚠️ Needs work / 🔴 Blocked

**Checklist:** [docs/code-review-checklist.md sections worked through]

**Findings:**
✅ [what is sound — be specific]
⚠️ [what needs fixing — P1]
🔴 [what blocks shipping — P0]

**Required changes before ship:**
- [ ] [Specific change with acceptance criteria]

**Accepted tradeoffs:**
- [What was consciously left imperfect and why]

**Next:** [fix iteration / ship / escalate to Founder]
```

---

## What NOT to Do

- Don't skip `docs/code-review-checklist.md` — it has the domain-specific criteria this project needs.
- Don't silently fix problems — flag and let the dev body fix.
- Don't block on style — only on correctness and non-negotiables.
- Don't let 🔴 Ugly findings ship without explicit Founder sign-off.
- Don't accept "verify passes" as sufficient if the checklist sections are incomplete.
