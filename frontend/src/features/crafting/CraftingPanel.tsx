import { ChoiceRow, EmptyState, Panel, PanelBody, ScrollRegion, SectionCard } from "../../components/ui";
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

export function CraftingPanel({ state, actions, canEdit, busy }: CraftingPanelProps) {
  return (
    <Panel className="detail-panel" title="Crafting Recipes" scroll>
      <PanelBody>
        <ScrollRegion className="recipe-checklist">
          {state.sortedRecipeIds.length > 0 ? (
            state.groupedRecipeIds.map((group) => (
              <SectionCard key={`recipe-group-${group.category}`} title={group.category} className="recipe-group">
                {group.ids.map((recipeId) => (
                  <ChoiceRow key={`recipe-${recipeId}`} kind="checkbox">
                    <input
                      type="checkbox"
                      checked={state.craftingRecipeDrafts.includes(recipeId)}
                      onChange={(event) => actions.handleToggle(recipeId, event.target.checked)}
                      disabled={!canEdit || busy || !actions.recipeIsKnown(recipeId)}
                    />
                    <span>
                      {actions.recipeLabel(recipeId)}
                      {actions.recipeIsKnown(recipeId) ? "" : " (unknown, preserved)"}
                    </span>
                  </ChoiceRow>
                ))}
              </SectionCard>
            ))
          ) : (
            <EmptyState>No recipe catalog entries are available for this save.</EmptyState>
          )}
        </ScrollRegion>
      </PanelBody>
    </Panel>
  );
}
