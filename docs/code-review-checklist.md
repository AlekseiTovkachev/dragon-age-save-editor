# Code Review Checklist

## Save Parsing

- [ ] Uses the correct GFF field labels and names.
- [ ] Handles DAO, DAO Awakening, and DA2 differences explicitly.
- [ ] Handles missing fields with clear errors or documented optional behavior.
- [ ] Includes a read-side test for new extracted data.

## Save Mutation

- [ ] Updates the raw GFF value.
- [ ] Updates the synchronized domain model value.
- [ ] Preserves unrelated save fields.
- [ ] Has a write/reload test for persisted edits.

## Game Data

- [ ] Updates DB seed files, not only the generated DB.
- [ ] Updates verifier rules if category, stackability, or game-key behavior changes.
- [ ] Keeps categories mapped by `ItemCategory`.
- [ ] Keeps stackability consistent with category rules.
- [ ] Runs `npm run data:build` and `npm run data:verify`.

## Frontend

- [ ] Handles every command result shape it sends.
- [ ] Refreshes dirty state and summaries after mutating commands.
- [ ] Preserves Apply/Reset behavior.
- [ ] Preserves DAO, DAO Awakening, and DA2 visibility rules.
- [ ] Keeps unknown/unsupported values visible but safe where applicable.

## Verification

- [ ] `cargo test`
- [ ] `cargo check`
- [ ] `npm run check`
- [ ] `npm run data:build`
- [ ] `npm run data:verify`
- [ ] `cargo check` in `src-tauri`
