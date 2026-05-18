# In-Game Test Save Prerequisites

These Playwright tests drive the real UI and write through the `apply_edit` sidecar. They are manual verification tests, not CI smoke tests.

Run them against a copied or restorable save. Set exactly one of `DAO_SAVE` or `DA2_SAVE` — the suite mutates one file at a time:

```powershell
# DAO-family spec
$env:DAO_SAVE = "C:\path\to\dao-savegame.das"
npm run ingame-test -- combo.spec.ts

# DA2 spec
$env:DA2_SAVE = "C:\path\to\da2-savegame.das"
npm run ingame-test -- da2-stats.spec.ts
```

The suite is forced to one worker because all tests mutate the same save file and use one sidecar document.

## Test Prerequisites

| Test | Save prerequisites |
|---|---|
| `stats.spec.ts` | DAO-family save; loadable main character |
| `abilities.spec.ts` | DAO-family save; main character has Master Coercion; main character does not already have Dual-Weapon Mastery |
| `inventory.spec.ts` / Rose's Thorn | DAO-family save; main character has The Rose's Thorn equipped; it has a Tier 1 material option |
| `inventory.spec.ts` / Health Poultice | DAO-family save; backpack contains stackable Health Poultice |
| `properties.spec.ts` | DAO-family save; main character has The Rose's Thorn equipped |
| `backpack-ops.spec.ts` | DAO-family save; backpack contains a non-stackable Ring of Ages; backpack also contains at least one non-Ring item |
| `companion.spec.ts` | DAO-family save; Morrigan is present; Morrigan has approval; Morrigan does not already have Blood Mage or Blood Magic |
| `combo.spec.ts` | DAO-family save; Alistair is present; Alistair has approval; Alistair has at least one equipped armor piece with a Tier 6 Silverite option |
| `da2-stats.spec.ts` | DA2 save; loadable main character |

The same prerequisites are executable in each spec via `ensurePrerequisites(...)`, so a mismatched save fails before UI mutation starts.

## Robustness Notes

- Prefer preflight checks over comments for save assumptions.
- Prefer named row locators and saved-file readback before manual verification where practical.
- Keep tests serial unless each test gets an isolated save copy and sidecar document.
- If a test changes the command contract or mocked smoke behavior, cover that separately in unit/smoke tests.
