---
name: fixture-safety
description: Use when reading, copying, editing, testing, or documenting local Dragon Age save fixtures, sample_saves, .das/.das.met/screen.dds bundles, or manual in-game validation.
metadata:
  short-description: Save fixture safety rules
---

# Fixture Safety

## Start Here

- Read `docs/fixtures.md`.
- Use copied saves only.
- Keep `.das`, `.das.met`, and `screen.dds` together when they exist.

## Rules

- Never mutate original user saves in place.
- Rust tests may read local fixtures but should write edited saves to temporary paths.
- Smoke tests should use the mocked Tauri backend instead of local save files.
- Manual QA should verify edited copies in game before originals are deleted or moved.

## Fixture Goals

Useful local fixtures cover:

- DAO vanilla;
- DAO Awakening-style campaigns;
- DA2 with plot flags;
- stackable and non-stackable items;
- items with multiple properties;
- crafting recipes.

## Guardrails

- Do not commit personal save files.
- Avoid path-dependent assertions when a semantic fixture check is clearer.
- Document unusual fixture assumptions in test names or comments.
