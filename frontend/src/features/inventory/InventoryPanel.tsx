import { ItemEditor } from "../../components/ItemEditor";
import { ItemList } from "../../components/ItemList";
import { NumericInput, Panel } from "../../components/ui";
import type { Dispatch, SetStateAction } from "react";
import type { ItemPropertyDraft } from "../../lib/itemUtils";
import type { IndexedItem, Item, SelectableItemProperty } from "../../types";
import type { ItemMetadataDraft, PropertyDraft } from "./useInventoryEditor";

type InventoryPanelProps = {
  state: InventoryPanelState;
  actions: InventoryPanelActions;
  canEdit: boolean;
  busy: boolean;
};

export type InventoryPanelState = {
  moneyDraft: string;
  items: IndexedItem[];
  itemIndex: number | null;
  selectedItem: Item | null;
  canEditStackSize: boolean;
  canCloneBackpackItem: boolean;
  canEditMaterial: boolean;
  itemMetadataDraft: ItemMetadataDraft;
  propertyDraft: PropertyDraft;
  itemPropertiesDraft: ItemPropertyDraft[];
  availableItemProperties: SelectableItemProperty[];
};

export type InventoryPanelActions = {
  setMoneyDraft: (value: string) => void;
  setItemIndex: (index: number) => void;
  setItemMetadataDraft: Dispatch<SetStateAction<ItemMetadataDraft>>;
  setPropertyDraft: Dispatch<SetStateAction<PropertyDraft>>;
  handlePropertyAddDraft: () => void;
  handlePropertyRemoveDraft: (propertyIndex: number) => void;
  handlePropertyUpdateDraft: (kind: "id" | "power", propertyIndex: number, raw: string) => void;
  handleBackpackRemove: () => Promise<void>;
  handleBackpackClone: () => Promise<void>;
  handleWikiOpen: (url: string) => Promise<void>;
};

export function InventoryPanel({ state, actions, canEdit, busy }: InventoryPanelProps) {
  return (
    <section className="split-section inventory-layout">
      <Panel
        className="list-panel"
        title="Inventory"
        headingAction={
          <label className="inventory-money-control">
            <span>Money</span>
            <NumericInput
              value={state.moneyDraft}
              min={0}
              onChange={(event) => actions.setMoneyDraft(event.target.value)}
              disabled={!canEdit || busy}
            />
          </label>
        }
      >
        <h3>Backpack</h3>
        <ItemList items={state.items} selectedIndex={state.itemIndex} onSelect={actions.setItemIndex} />
      </Panel>
      <Panel className="detail-panel" scroll>
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
      </Panel>
    </section>
  );
}
