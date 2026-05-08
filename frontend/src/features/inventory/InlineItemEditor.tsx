import { ItemEditor } from "../../components/ItemEditor";
import type { InventoryPanelActions, InventoryPanelState } from "./InventoryPanel";

type InlineItemEditorProps = {
  state: InventoryPanelState;
  actions: InventoryPanelActions;
  canEdit: boolean;
  busy: boolean;
};

export function InlineItemEditor({ state, actions, canEdit, busy }: InlineItemEditorProps) {
  return (
    <div className="inline-item-editor">
      <ItemEditor
        item={state.selectedItem}
        itemIndex={state.itemIndex}
        canEdit={canEdit}
        busy={busy}
        allowRemove
        canEditStackSize={state.canEditStackSize}
        canCloneBackpackItem={state.canCloneBackpackItem}
        canEditMaterial={state.canEditMaterial}
        metadataDraft={state.itemMetadataDraft}
        propertyDraft={state.propertyDraft}
        itemPropertiesDraft={state.itemPropertiesDraft}
        availableItemProperties={state.availableItemProperties}
        onMetadataChange={(patch) => actions.setItemMetadataDraft((current) => ({ ...current, ...patch }))}
        onPropertyDraftChange={(patch) => actions.setPropertyDraft((current) => ({ ...current, ...patch }))}
        onPropertyAdd={actions.handlePropertyAddDraft}
        onPropertyRemove={actions.handlePropertyRemoveDraft}
        onPropertyUpdate={actions.handlePropertyUpdateDraft}
        onRemove={() => void actions.handleBackpackRemove()}
        onClone={() => void actions.handleBackpackClone()}
        onWikiOpen={(url) => void actions.handleWikiOpen(url)}
      />
    </div>
  );
}
