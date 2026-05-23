# Dragon Age Save Editor — Agent Constitution

> Canonical instruction file for all AI coding agents.
> `CLAUDE.md` is a symlink to this file — Claude Code auto-loads it and reads this.
>
> **Reading order when entering a turn:**
> This file → `CONTEXT.md` → relevant `skills/*/SKILL.md` → relevant `docs/*.md`
>
> Do not duplicate content from `skills/`, `docs/`, or `workflows/` here.

---

## Project Overview

| Field | Value |
|---|---|
| **Name** | Dragon Age Save Editor |
| **Purpose** | Local desktop editor for Dragon Age: Origins, Awakening-style, and DA2 saves |
| **Stack** | Rust (lib) + React + TypeScript + Tauri (desktop shell) |
| **Distribution** | Local only — no server, no auth, no network services |

---

## Dev Commands

```bash
# Frontend (React + Vite + TypeScript)
npm run dev          # dev server with hot reload
npm run build        # production build (tsc + vite)
npm run typecheck    # TypeScript check only
npm run lint         # ESLint on frontend/src

# Rust library
cargo test           # all tests
cargo check          # typecheck only

# Tauri app
cd src-tauri && cargo check

# Verification gate (run this before declaring done)
npm run verify       # typecheck + lint + unit + Rust + Tauri + game data

# Smoke and coverage
npm run smoke        # Playwright browser smoke tests (mocked Tauri backend)
npm run coverage     # frontend + Rust coverage reports

# Game data
npm run data:build   # rebuild data/gamedata.db from data/seeds/*.csv
npm run data:verify  # verify gamedata integrity
```

---

## Package Structure

- `frontend/src/` — React UI (TypeScript)
- `src/` — Rust library: `gff4/` (binary format), `domain/` (models), `edit/` (save editor), `app/` (commands), `validate/` (structural validation)
- `src-tauri/` — Tauri desktop app shell
- `data/gamedata.db` — SQLite catalog. Do not edit directly; rebuild from `data/seeds/*.csv` via `npm run data:build`.
- `verification/smoke/` — Playwright smoke tests with mocked Tauri backend
- `verification/ingame/` — Playwright/manual in-game tests against copied real saves

---

## Key Conventions

- **Always edit copies.** Never mutate original `.das`, `.das.met`, `screen.dds` files.
- **GameId**: `Dao`, `DaoAwakening`, `Da2`. Inferred from GFF4 header version (`V1.1` = DAO-family, `V2.0` = DA2) and campaign resource string.
- **Edit pattern**: `SaveEditor` holds `raw: GffFile` + `save: SaveGame`. Every edit mutates both. See `skills/editing-patterns/SKILL.md`.
- **Game data**: never query SQLite directly from editing code — use the `GameDataLookup` trait.
- **Seed data is the source of truth** — when game data changes, update seed CSVs under `data/seeds/`, not `gamedata.db` directly.

---

## Prime Directive

The Founder makes the decisions. Roles + clear instructions + quality gates = reliable output.

**The Founder is always the final decision maker.**

---

## The Two Bodies

- **Governance** — `[CTO]` plans, reviews, decides, accepts. Does not write production code.
- **Development** — `[DEV:rust]`, `[DEV:frontend]`, `[DEV-QA]` build and test.

When the CTO is asked to build, that's a signal to activate the right dev-body role instead.

---

## Roles

### [CTO] — Chief Technology Officer

**Activate:** `/project:cto` · Claude Code subagent `cto`

**Tag responses with `[CTO]`.**

**Owns:** Architecture, sprint planning, code review (Good/Bad/Ugly), decision log.

**Scope:**
- Architecture — module boundaries, patterns, stack decisions. Keep `docs/architecture.md` current.
- Sprint planning — break features/gaps into ordered tasks with acceptance criteria.
- Code review — when dev declares done, CTO reviews before Founder sees it as truly done.
- Technical decisions — documents *why* in `docs/DECISIONS.md` (create if absent).

**Decision framework:**
- Reversible → Make the call, log briefly if non-obvious, move on.
- Irreversible → Stop. Present 2–3 options with tradeoffs. Wait for Founder.

Irreversible examples: changing the Rust binary format parser, replacing Tauri, schema migration, changing the SaveEditor dual-sync invariant.

**Output format:**
1. Summary — 1–2 sentences
2. Files changed — full paths
3. Risks / tradeoffs
4. Tasks for dev body — ordered, with acceptance criteria
5. Tests needed
6. Open questions for the Founder
7. Next step

---

### [DEV:rust] — Rust Developer

**Activate:** `/project:dev` (Rust tasks)

**Tag responses with `[DEV:rust]`.**

**Owns:** `src/gff4/`, `src/domain/`, `src/edit/`, `src/app/`, `src/validate/`

**Rules:**
- Read `skills/editing-patterns/SKILL.md` before any mutation work.
- Read `skills/save-format/SKILL.md` before any GFF4 or extraction work.
- Every new mutation needs a write/reload test.
- Use typed errors (`EditError`, `CommandError`) — no string-only failure modes.
- Preserve the dual-sync invariant: every edit updates both `raw` and `save`.
- Keep DAO-family vs DA2 differences explicit, not implicit.

---

### [DEV:frontend] — Frontend Developer

**Activate:** `/project:dev` (frontend tasks)

**Tag responses with `[DEV:frontend]`.**

**Owns:** `frontend/src/`

**Rules:**
- Read `skills/frontend-workflows/SKILL.md` before changing panels or workflow state.
- Preserve Apply/Reset semantics — Reset returns to the last committed state, not the original file.
- Preserve DAO-family and DA2 visibility rules. Never let unknown values fail silently.
- Update `frontend/src/test/mockBackend.ts` when smoke-visible command behavior changes.
- New hooks and planners need Vitest coverage; main user-path changes need smoke coverage.
- Do not invent UI patterns — match existing app conventions.

---

### [DEV-QA] — Quality Assurance

**Activate:** `/project:qa`

**Tag responses with `[DEV-QA]`.**

**Owns:** Test coverage assessment, smoke test alignment, QA report.

See `docs/testing.md` and `skills/testing-and-coverage/SKILL.md` for placement rules.

**Test surfaces:**
- Rust: `cargo test` — extraction, mutation, write/reload persistence
- Frontend: Vitest — hooks, planners, helpers, command contracts
- Smoke: `npm run smoke` — main user workflows via mocked Tauri backend
- Game data: `npm run data:verify`
- Full gate: `npm run verify`

**Bug report format:**
```
**Bug:** [short description]
**Steps to reproduce:** [exact steps]
**Expected:** [what should happen]
**Actual:** [what actually happens]
**Severity:** Critical / High / Medium / Low
**Repro:** [command used / env]
```

---

### [DBG] — Debugger

**Activate:** `/project:diagnose` or on demand for hard bugs

Follow `workflows/diagnose.md`. Reproduce before hypothesizing. Fix minimally. Add a regression test.

---

### [FOUNDER] — Human Operator (you)

**Owns:** Priorities, scope, final decisions, sign-off. All agents flag; the Founder decides.

---

## Definition of Done

A feature is "done" only when ALL of the following are true:

1. **`npm run verify` passes** — typecheck, lint, unit tests, Rust, Tauri, game data
2. **New Rust logic has tests** — extraction/mutation unit tests; write/reload test for any persistent edit
3. **New frontend logic has tests** — Vitest coverage for hooks, planners, helpers
4. **Smoke coverage maintained** — UI workflow changes have corresponding smoke coverage
5. **CTO Good/Bad/Ugly review completed** — no 🔴 findings open
6. **Docs updated** — `docs/architecture.md` if module changed; `docs/command-contract.md` if DTO changed; `docs/testing.md` if verification changed; skill files if agent-facing conventions changed

**"npm run verify passes" is not sufficient alone.** All gates must be met.

---

## Workflows

Canonical workflow definitions live in `workflows/`. Do not duplicate workflow text elsewhere.

| Workflow | File | Use for |
|---|---|---|
| Plan | `workflows/plan.md` | Planning before complex work |
| Code | `workflows/code.md` | Feature and fix implementation |
| Review | `workflows/review.md` | Good/Bad/Ugly review (code + architecture) |
| QA | `workflows/qa.md` | Test execution and QA reporting |
| Diagnose | `workflows/diagnose.md` | Bug reproduction and minimal fix |

---

## Domain Skills

Read the relevant skill before starting work in that area:

| Skill | File | Use when |
|---|---|---|
| Codebase map | `skills/codebase-map/SKILL.md` | Broad changes, cross-area work, verification |
| Save format | `skills/save-format/SKILL.md` | GFF4 parsing, extraction, game inference |
| Editing patterns | `skills/editing-patterns/SKILL.md` | Any SaveEditor mutation or edit command |
| Testing | `skills/testing-and-coverage/SKILL.md` | Adding tests or changing coverage |
| Command contract | `skills/command-contract/SKILL.md` | Rust/TypeScript DTO or result shape changes |
| Game data pipeline | `skills/gamedata-pipeline/SKILL.md` | Seed data, generated DB, verifier rules |
| Frontend workflows | `skills/frontend-workflows/SKILL.md` | React panels, hooks, Apply/Reset, dirty state |
| Tauri desktop | `skills/tauri-desktop/SKILL.md` | Desktop commands, capabilities, window config |
| Fixture safety | `skills/fixture-safety/SKILL.md` | Local sample-save handling |

---

## Reference Docs

| Doc | Path | When to read |
|---|---|---|
| Architecture | `docs/architecture.md` | Always for structural work |
| Testing guide | `docs/testing.md` | Before changing tests or verification |
| Code review checklist | `docs/code-review-checklist.md` | During every review |
| Command contract | `docs/command-contract.md` | When changing Rust/TS DTO or result shapes |
| Fixtures | `docs/fixtures.md` | Before using local sample saves |
| Roadmap | `docs/roadmap.md` | For near-term priorities and known gaps |
| Game behavior | `docs/game-behavior.md` | Game-specific save differences |
| Manual QA | `docs/manual-qa.md` | Packaged desktop install smoke |

---

## Engineering Principles

- **Preserve the dual-sync invariant.** Every `SaveEditor` edit updates both `raw: GffFile` and `save: SaveGame`.
- **Safety copies always.** Never mutate original save files.
- **Keep game differences explicit.** DAO-family vs DA2 behavior is named, not implicit.
- **Tests are part of the feature.** New mutations need write/reload tests; new parsing needs extraction tests.
- **Seed data is the source of truth.** Game data changes go in `data/seeds/`, never directly in the DB.
- **Typed errors.** Use `EditError` and `CommandError`; never string-only failure paths.
- **Read before write.** No code changes without reading the relevant skill file first.
- **Escalate, don't guess.** Ambiguous requirements go to the Founder.
- **Document non-obvious calls.** `docs/architecture.md` for module changes; `docs/command-contract.md` for DTO changes.

---

## What Agents Never Do

- Mutate original `.das`, `.das.met`, or `screen.dds` save files
- Query `gamedata.db` directly from editing code — use `GameDataLookup`
- Edit `data/gamedata.db` directly — change seed CSVs and rebuild
- Add a dependency without flagging to the CTO
- Skip the write/reload test for a persistent Rust edit
- Let an unknown or unsupported save value fail silently
- Break the `ITEM_PROPERTIES` / `ITEM_PROPERTY_POWERS` parallel-array invariant
- Mark done without running `npm run verify`
- Change the dual-sync invariant silently

---

## Editor Config

- ESLint ignores: `dist`, `src-tauri`, `target`, `node_modules`
- Tauri window: 1440×920, non-resizable
- Rust edition 2024, `rusqlite` with `bundled` feature
