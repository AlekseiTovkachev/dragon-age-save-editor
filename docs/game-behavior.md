# Game Behavior Notes

## Game Detection

The backend distinguishes these save families:

- `dao`: vanilla Dragon Age: Origins.
- `dao_awakening`: Awakening-format DAO content, including Awakening, Witch Hunt, and Golems of Amgarrak.
- `da2`: Dragon Age II.

DAO and Awakening share the DAO save format, but Awakening-style saves allow additional catalog content.

## Awakening Gating

For vanilla DAO saves:

- material tiers 8 and 9 are hidden;
- abilities with IDs `>= 400000` are hidden;
- DA2 plot flags are hidden.

For Awakening-style DAO saves:

- Awakening material tiers are available;
- Awakening abilities are available;
- DA2 plot flags remain hidden.

For DA2 saves:

- skills are hidden because DA2 has no skill list;
- DA2 plot flags are available;
- DA2 item property power values are treated as floats where the save stores float bit patterns.

## Item Stack And Clone Rules

Stack editing is allowed only when the item catalog marks the item as stackable.

Clone is allowed only for non-stackable backpack items. Cloning preserves the item data and assigns a fresh `OBJECT_ID`.
