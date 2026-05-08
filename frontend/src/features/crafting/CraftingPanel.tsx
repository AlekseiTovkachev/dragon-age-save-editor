import { useMemo, useState } from "react";
import { ChoiceRow, EmptyState, Panel, PanelBody } from "../../components/ui";
import type { RecipeChecklistGroup } from "../../lib/recipeUtils";

type CraftingPanelProps = {
  state: CraftingPanelState;
  actions: CraftingPanelActions;
  canEdit: boolean;
  busy: boolean;
};

export type CraftingPanelState = {
  sortedRecipeIds: number[];
  groupedRecipeIds: RecipeChecklistGroup[];
  craftingRecipeDrafts: number[];
};

export type CraftingPanelActions = {
  handleToggle: (recipeId: number, checked: boolean) => void;
  recipeIsKnown: (recipeId: number) => boolean;
  recipeLabel: (recipeId: number) => string;
};

const ALL_CATEGORIES = "All";

export function CraftingPanel({ state, actions, canEdit, busy }: CraftingPanelProps) {
  const [search, setSearch] = useState("");
  const [category, setCategory] = useState(ALL_CATEGORIES);
  const disabled = !canEdit || busy;

  const categories = useMemo(() => recipeCategories(state.groupedRecipeIds), [state.groupedRecipeIds]);
  const filteredGroups = useMemo(
    () => filterRecipeGroups(state.groupedRecipeIds, actions.recipeLabel, category, search),
    [actions.recipeLabel, category, search, state.groupedRecipeIds],
  );

  return (
    <Panel
      className="detail-panel recipes-panel"
      title={
        <div>
          <div className="crumb">Edit &middot; Crafting</div>
          <h2>Recipes</h2>
        </div>
      }
      headingAction={
        <span className="mono muted recipe-known-count">
          {state.craftingRecipeDrafts.length} of {state.sortedRecipeIds.length} known
        </span>
      }
      scroll
    >
      <PanelBody>
        <div className="recipe-stack">
          <div className="recipe-toolbar">
            <input
              className="search-input recipe-search"
              type="search"
              value={search}
              onChange={(event) => setSearch(event.target.value)}
              placeholder="Search recipes..."
              aria-label="Search recipes"
            />
            <div className="recipe-chip-row" aria-label="Recipe categories">
              {categories.map((entry) => (
                <button
                  key={`recipe-category-${entry}`}
                  type="button"
                  className={["cat-chip", entry === category ? "is-active" : ""].filter(Boolean).join(" ")}
                  onClick={() => setCategory(entry)}
                  aria-pressed={entry === category}
                >
                  {entry}
                </button>
              ))}
            </div>
          </div>

          {state.sortedRecipeIds.length > 0 ? (
            filteredGroups.length > 0 ? (
              <div className="recipe-grid">
                {filteredGroups.map((group) => (
                  <RecipeCategoryCard
                    key={`recipe-group-${group.category}`}
                    group={group}
                    actions={actions}
                    selectedIds={state.craftingRecipeDrafts}
                    disabled={disabled}
                  />
                ))}
              </div>
            ) : (
              <div className="recipe-empty">No recipes match the current filters.</div>
            )
          ) : (
            <EmptyState>No recipe catalog entries are available for this save.</EmptyState>
          )}
        </div>
      </PanelBody>
    </Panel>
  );
}

type RecipeCategoryCardProps = {
  group: RecipeChecklistGroup;
  actions: CraftingPanelActions;
  selectedIds: number[];
  disabled: boolean;
};

function RecipeCategoryCard({ group, actions, selectedIds, disabled }: RecipeCategoryCardProps) {
  const knownInGroup = group.ids.filter((recipeId) => selectedIds.includes(recipeId)).length;

  return (
    <section className="recipe-card" aria-labelledby={`recipe-group-${group.category}`}>
      <div className="recipe-card-head">
        <h3 id={`recipe-group-${group.category}`}>{group.category}</h3>
        <span className="mono muted">
          {knownInGroup} / {group.ids.length}
        </span>
      </div>
      <div className="recipe-rows">
        {group.ids.map((recipeId) => {
          const knownRecipe = actions.recipeIsKnown(recipeId);
          return (
            <ChoiceRow
              key={`recipe-${recipeId}`}
              kind="checkbox"
              className={["recipe-row", selectedIds.includes(recipeId) ? "is-known" : ""].filter(Boolean).join(" ")}
            >
              <input
                type="checkbox"
                checked={selectedIds.includes(recipeId)}
                onChange={(event) => actions.handleToggle(recipeId, event.target.checked)}
                disabled={disabled || !knownRecipe}
              />
              <span className="recipe-label">
                {actions.recipeLabel(recipeId)}
                {knownRecipe ? "" : " (unknown, preserved)"}
              </span>
            </ChoiceRow>
          );
        })}
      </div>
    </section>
  );
}

function recipeCategories(groups: RecipeChecklistGroup[]) {
  return [ALL_CATEGORIES, ...groups.map((group) => group.category)];
}

function filterRecipeGroups(
  groups: RecipeChecklistGroup[],
  recipeLabel: (recipeId: number) => string,
  category: string,
  search: string,
) {
  const query = search.trim().toLowerCase();
  return groups
    .filter((group) => category === ALL_CATEGORIES || group.category === category)
    .map((group) => ({
      ...group,
      ids: group.ids.filter((recipeId) => {
        if (!query) {
          return true;
        }
        return [
          recipeId.toString(),
          group.category,
          recipeLabel(recipeId),
        ].some((value) => value.toLowerCase().includes(query));
      }),
    }))
    .filter((group) => group.ids.length > 0);
}
