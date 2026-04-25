# Frontend Refactoring Plan

## Architecture Overview

- **React 18 + TypeScript + Vite** with Tauri backend
- **State management**: local `useState`/`useMemo`/`useEffect` — no external state library
- **API layer**: `api.ts` wraps Tauri `invoke` calls
- **Single massive component**: `App.tsx` (~1554 lines) handles all state, API calls, and rendering

## Key Problems

1. **`App.tsx` is monolithic** — All state, logic, and rendering lives in one file
2. **Repeated patterns** — `run()` wrapper, refresh functions, reset functions for each section
3. **No extracted hooks** — Complex stateful logic not reusable across components
4. **Draft state proliferation** — Each section has its own draft state with similar structure
5. **Internal render functions** — `renderItemList()`, `renderItemEditor()` are functions, not components

## Priority 1: Extract Custom Hooks

### Problem

`run()` pattern (setBusy → try → catch → setError → finally) is repeated 15+ times. Character and inventory editing logic is embedded in App.tsx.

### Solution

Create `useAsyncOperation` hook:
```typescript
function useAsyncOperation() {
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function run(action: () => Promise<void>) {
    setBusy(true);
    setError(null);
    try {
      await action();
    } catch (caught) {
      setError(toErrorMessage(caught));
    } finally {
      setBusy(false);
    }
  }

  return { busy, error, run, clearError: () => setError(null) };
}
```

Extract feature hooks:
- `useCharacterEditor(target)` — loads character, manages draft, apply/reset
- `useInventoryEditor(container)` — item selection, metadata draft, property management
- `usePlotFlagsEditor()` — boolean/integer draft management
- `useCraftingEditor()` — recipe checklist management

### Files Affected

- New: `frontend/src/hooks/useAsyncOperation.ts`
- New: `frontend/src/hooks/useCharacterEditor.ts`
- New: `frontend/src/hooks/useInventoryEditor.ts`

---

## Priority 2: Extract ItemList as Proper Component

### Problem

`renderItemList()` is a function returning JSX, not a proper React component.

### Solution

```typescript
function ItemList({ items, selectedIndex, onSelect }: ItemListProps) {
  return (
    <div className="item-list scroll-region">
      {items.map((entry) => (
        <button
          key={entry.index}
          className={entry.index === selectedIndex ? "list-row active" : "list-row"}
          onClick={() => onSelect(entry.index)}
        >
          <strong>{itemLabel(entry.item, entry.index)}</strong>
          <span>{entry.item.resref ?? "no resref"}</span>
          <span>{entry.item.properties.length} Propert{entry.item.properties.length === 1 ? "y" : "ies"}</span>
        </button>
      ))}
    </div>
  );
}
```

### Files Affected

- New: `frontend/src/components/ItemList.tsx`

---

## Priority 3: Extract App.tsx into Feature Components

### Problem

`App.tsx` handles characters, inventory, recipes, plot flags, crafting — all in one file.

### Solution

Extract by section:
```
frontend/src/components/
├── CharacterPanel.tsx      # characters tab (overview + abilities + equipment)
├── CharacterOverview.tsx   # stats, level, experience, approval
├── CharacterAbilities.tsx  # ability list management
├── InventoryPanel.tsx     # inventory tab (items + money)
├── ItemEditor.tsx          # item metadata + properties (from renderItemEditor)
├── CraftingPanel.tsx      # recipes tab
├── PlotFlagsPanel.tsx     # DA2 plot flags tab
```

### Files Affected

- New: `frontend/src/components/CharacterPanel.tsx`
- New: `frontend/src/components/InventoryPanel.tsx`
- New: `frontend/src/components/CraftingPanel.tsx`
- New: `frontend/src/components/PlotFlagsPanel.tsx`
- Modify: `frontend/src/App.tsx` — becomes orchestrator passing hooks to components

---

## Priority 4: Extract Pure Transformation Utilities

### Problem

Functions like `groupedAbilities`, `groupedPlotBooleans`, `groupedRecipeChecklistIds` are defined inside App.tsx but are pure transformations.

### Solution

Move to `lib/`:
- `groupedAbilities` → `lib/abilityUtils.ts`
- `groupedPlotBooleans`, `groupedPlotIntegers` → `lib/plotFlagUtils.ts`
- `groupedRecipeChecklistIds` → `lib/recipeUtils.ts`
- `toItemPropertyDrafts` → `lib/itemUtils.ts`
- `cloneAbilities` → `lib/abilityUtils.ts`

### Files Affected

- New: `frontend/src/lib/plotFlagUtils.ts`
- New: `frontend/src/lib/recipeUtils.ts`
- Modify: `frontend/src/lib/abilityUtils.ts` (add groupedAbilities, cloneAbilities)
- Modify: `frontend/src/lib/itemUtils.ts` (add toItemPropertyDrafts)

---

## Priority 5: Consolidate Draft State Management

### Problem

Each section has its own draft state pattern:
- `statsDraft`, `levelDraft`, `experienceDraft`, `approvalDraft`, `pointPoolsDraft`
- `abilityDrafts` + `selectedAbilityToAdd`
- `itemMetadataDraft`, `itemPropertiesDraft`, `propertyDraft`
- `plotBooleanDrafts`, `plotIntegerDrafts`
- `craftingRecipeDrafts`

### Solution

After extracting hooks (Priority 1), each hook manages its own draft state. Consider typed draft structures:

```typescript
type CharacterDraft = {
  stats: CoreStats;
  level: string;
  experience: string;
  approval: string;
  pointPools: PointPools;
  abilities: Record<AbilityListKind, Ability[]>;
};
```

Optionally consider `react-hook-form` for complex form state if the project grows.

### Files Affected

- Modify: extracted hooks in `frontend/src/hooks/`

---

## Priority 6: Strict Command/Result Types

### Problem

`SaveCommand` and `SaveCommandResult` are union types with string discriminants. Mismatch between command and result types is not enforced.

### Solution

Currently the types are correct at the Tauri boundary. The issue is that `executeCommand` returns `SaveCommandResult` which is a union. Consider narrowing with type guards:

```typescript
function isCharacterResult(result: SaveCommandResult): result is CharacterResult {
  return result.result === "character";
}
```

Or consider discriminated unions per command:
```typescript
type GetCharacterResult = { result: "character"; target: CharacterTarget; character: Character };
```

### Files Affected

- Modify: `frontend/src/types.ts`

---

## Order of Implementation

1. **Extract `useAsyncOperation` hook** — no UI changes, just state extraction
2. **Extract `ItemList` component** — straightforward, used in multiple places
3. **Extract `useCharacterEditor`, `useInventoryEditor` hooks** — core feature logic
4. **Extract render functions** → proper components (`ItemEditor`, `CharacterOverview`, etc.)
5. **Break up `App.tsx`** — pass hooks as props to extracted components
6. **Move pure functions to `lib/`**
7. **Consolidate draft types** — after hooks are stable

## Non-Goals (Out of Scope)

- Adding Redux, Zustand, or other state management libraries
- Switching to a component library (Material UI, etc.)
- Rewriting CSS — current plain CSS approach is adequate
- Changing the Tauri API contract
- Adding unit tests (integration-level testing sufficient for now)

---

## Success Criteria

After refactoring:
- `App.tsx` should be < 200 lines, primarily orchestrating components
- Each hook/component should be independently testable
- No duplicate `run()` wrappers — all async operations use the shared hook
- Feature components can be rendered in isolation