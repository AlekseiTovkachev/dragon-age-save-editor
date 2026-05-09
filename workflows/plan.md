# Workflow: Plan

> **When to use:** Before implementing anything non-trivial — new edit commands, GFF4 parsing changes,
> architectural shifts, or features that touch more than one layer (Rust + frontend + game data).
> Simple bug fixes with clear root causes may skip this.

---

## Rules

- Do NOT write any code during this workflow.
- Read the relevant skill files before designing the approach.
- Do not proceed until the Founder (or CTO) explicitly approves the plan.
- Flag irreversible decisions — any change to the dual-sync invariant, the SaveEditor public API,
  command DTOs, the GFF4 binary format handling, or the game data schema.

---

## Process

### Step 1 — Understand the request

- What is being asked?
- What are the acceptance criteria?
- What is explicitly out of scope?
- What is unclear? List ambiguities — ask before proceeding.

### Step 2 — Read context

At minimum:
- `AGENTS.md` — conventions, DoD, ownership
- `CONTEXT.md` — domain terminology
- `skills/codebase-map/SKILL.md` — module layout
- The specific skill files for the area being changed (save-format, editing-patterns, frontend-workflows, etc.)
- The relevant `docs/*.md` (architecture.md, command-contract.md, testing.md as appropriate)

### Step 3 — Design the approach

- Simplest correct change that satisfies the acceptance criteria
- Which files need to change?
- Which layers are affected (Rust lib / frontend / Tauri / game data)?
- What tests are needed (write/reload? Vitest? smoke?)?
- Is anything irreversible? Flag it explicitly.

### Step 4 — Present the plan

```
## Plan: [Feature / Change Name]

**Goal:** [What we're building and why]

**Layers affected:** Rust / Frontend / Tauri / Game data (delete inapplicable)

**Approach:**
1. [Step 1]
2. [Step 2]

**Files to change:**
- `path/to/file` — [what changes]

**New files:**
- `path/to/new` — [purpose]

**Irreversible decisions:** [list, or "none"]

**Tests needed:**
- Rust: [write/reload / extraction unit test]
- Frontend: [Vitest — what]
- Smoke: [yes/no — which workflow]
- Game data: [data:verify / data:build]

**Docs to update:**
- [docs/architecture.md / docs/command-contract.md / etc.]

**Open questions for Founder:**
- [if any]
```

### Step 5 — Wait for approval

Do NOT proceed until approved. If feedback arrives, update the plan first.

---

## What NOT to Do

- Don't write code during planning.
- Don't skip reading the relevant skill files.
- Don't assume game-specific behavior — read `skills/save-format/SKILL.md` or `docs/game-behavior.md`.
- Don't guess at unclear requirements.
