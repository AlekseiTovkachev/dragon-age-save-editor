import { useMemo, useState } from "react";
import { editableItemProperties } from "../lib/itemUtils";
import type { ItemPropertyDraft } from "../lib/itemUtils";
import type { Item, SelectableItemProperty } from "../types";
import {
  EmptyState,
  Field,
  NumericInput,
  SelectInput,
} from "./ui";

type ItemMetadataDraft = {
  material: string;
  item_level: string;
  stack_size: string;
};

type PropertyDraft = {
  property_id: string;
  power: string;
};

type ItemEditorProps = {
  item: Item | null;
  itemIndex: number | null;
  canEdit: boolean;
  busy: boolean;
  allowRemove: boolean;
  canEditStackSize: boolean;
  canCloneBackpackItem: boolean;
  canEditMaterial: boolean;
  canEditItemLevel: boolean;
  metadataDraft: ItemMetadataDraft;
  propertyDraft: PropertyDraft;
  itemPropertiesDraft: ItemPropertyDraft[];
  availableItemProperties: SelectableItemProperty[];
  onMetadataChange: (patch: Partial<ItemMetadataDraft>) => void;
  onPropertyDraftChange: (patch: Partial<PropertyDraft>) => void;
  onPropertyAdd: () => void;
  onPropertyRemove: (propertyIndex: number) => void;
  onPropertyUpdate: (kind: "id" | "power", propertyIndex: number, raw: string) => void;
  onRemove: () => void;
  onClone: () => void;
  onWikiOpen: (url: string) => void;
};

export function ItemEditor({
  item,
  itemIndex: _itemIndex,
  canEdit,
  busy,
  allowRemove,
  canEditStackSize,
  canCloneBackpackItem,
  canEditMaterial,
  canEditItemLevel,
  metadataDraft,
  propertyDraft,
  itemPropertiesDraft,
  availableItemProperties,
  onMetadataChange,
  onPropertyDraftChange,
  onPropertyAdd,
  onPropertyRemove,
  onPropertyUpdate: _onPropertyUpdate,
  onRemove,
  onClone,
  onWikiOpen,
}: ItemEditorProps) {
  const [showAddForm, setShowAddForm] = useState(false);

  const editableAvailableItemProperties = useMemo(
    () => editableItemProperties(availableItemProperties),
    [availableItemProperties],
  );
  const visibleItemPropertiesDraft = useMemo(
    () =>
      itemPropertiesDraft
        .map((property, propertyIndex) => ({ property, propertyIndex }))
        .filter(({ property }) => editableItemProperties([property]).length > 0),
    [itemPropertiesDraft],
  );

  if (!item) {
    return (
      <div className="item-editor-inline">
        <EmptyState>Select an item to edit metadata and properties.</EmptyState>
      </div>
    );
  }

  return (
    <div className="item-editor-inline">
      {/* Header */}
      <div className="item-editor-header">
        <div className="item-editor-header-left">
          <div className="item-sub">
            {item.resref ? <span>{item.resref}</span> : null}
            {item.resref && item.wiki_url ? <span>•</span> : null}
            {item.wiki_url ? (
              <a
                href={item.wiki_url}
                onClick={(event) => {
                  event.preventDefault();
                  onWikiOpen(item.wiki_url!);
                }}
              >
                open wiki page →
              </a>
            ) : null}
          </div>
        </div>
        {allowRemove ? (
          <div className="item-editor-header-actions">
            {canCloneBackpackItem ? (
              <button onClick={onClone} disabled={!canEdit || busy}>Clone</button>
            ) : null}
            <button className="danger" onClick={onRemove} disabled={!canEdit || busy}>Remove</button>
          </div>
        ) : null}
      </div>

      {/* Fields row */}
      <div className="item-fields-row">
        {canEditMaterial ? (
          <Field label="Material">
            <SelectInput
              value={metadataDraft.material}
              onChange={(event) => onMetadataChange({ material: event.target.value })}
              disabled={!canEdit || busy}
            >
              {item.material_options.map((option) => (
                <option key={`material-${option.code}`} value={option.code}>
                  {`Tier ${option.tier} - ${option.name}`}
                </option>
              ))}
            </SelectInput>
          </Field>
        ) : null}
        {canEditItemLevel ? (
          <Field label="Item Level">
            <NumericInput
              value={metadataDraft.item_level}
              min={0}
              onChange={(event) => onMetadataChange({ item_level: event.target.value })}
              disabled={!canEdit || busy}
            />
          </Field>
        ) : null}
        {canEditStackSize ? (
          <Field label="Stack Size">
            <NumericInput
              value={metadataDraft.stack_size}
              min={1}
              max={99}
              onChange={(event) => onMetadataChange({ stack_size: event.target.value })}
              disabled={!canEdit || busy}
            />
          </Field>
        ) : null}
      </div>

      {/* Properties section */}
      <div className="prop-chips-section">
        <span className="prop-chips-label">
          Properties ({visibleItemPropertiesDraft.length})
        </span>
        <div className="prop-chips">
          {visibleItemPropertiesDraft.map(({ property, propertyIndex }) => {
            const match = editableAvailableItemProperties.find((p) => p.id === property.id);
            if (!match) return null;
            const displayName = match.name ?? `Property ${property.id}`;
            return (
              <div key={`${property.id}-${propertyIndex}`} className="prop-chip">
                <span className="prop-chip-name">{displayName}</span>
                {property.power ? (
                  <span className="prop-chip-power">+{property.power}</span>
                ) : null}
                <button
                  className="prop-chip-remove"
                  onClick={() => onPropertyRemove(propertyIndex)}
                  disabled={!canEdit || busy}
                  aria-label={`Remove ${displayName}`}
                >
                  ×
                </button>
              </div>
            );
          })}

          {showAddForm ? (
            <div className="prop-add-form">
              <SelectInput
                value={propertyDraft.property_id}
                onChange={(event) => onPropertyDraftChange({ property_id: event.target.value })}
                disabled={!canEdit || busy}
              >
                {editableAvailableItemProperties.map((property) => (
                  <option key={`property-${property.id}`} value={property.id}>
                    {property.name ?? `Property ${property.id}`}
                  </option>
                ))}
              </SelectInput>
              <NumericInput
                placeholder="Power"
                value={propertyDraft.power}
                min={0}
                allowDecimal
                onChange={(event) => onPropertyDraftChange({ power: event.target.value })}
                disabled={!canEdit || busy}
              />
              <button
                onClick={() => {
                  onPropertyAdd();
                  setShowAddForm(false);
                }}
                disabled={!canEdit || busy}
              >
                Add
              </button>
              <button onClick={() => setShowAddForm(false)}>Cancel</button>
            </div>
          ) : (
            <button
              className="add-prop-btn"
              onClick={() => setShowAddForm(true)}
              disabled={!canEdit || busy}
            >
              + add property
            </button>
          )}
        </div>
      </div>
    </div>
  );
}
