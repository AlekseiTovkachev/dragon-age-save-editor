import { editableItemProperties, itemLabel } from "../lib/itemUtils";
import type { ItemPropertyDraft } from "../lib/itemUtils";
import type { Item, SelectableItemProperty } from "../types";
import {
  ButtonRow,
  EmptyState,
  Field,
  FieldGrid,
  GridTable,
  GridTableRow,
  PanelBody,
  SelectInput,
  TextInput,
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
  itemIndex,
  canEdit,
  busy,
  allowRemove,
  canEditStackSize,
  canCloneBackpackItem,
  canEditMaterial,
  metadataDraft,
  propertyDraft,
  itemPropertiesDraft,
  availableItemProperties,
  onMetadataChange,
  onPropertyDraftChange,
  onPropertyAdd,
  onPropertyRemove,
  onPropertyUpdate,
  onRemove,
  onClone,
  onWikiOpen,
}: ItemEditorProps) {
  const editableAvailableItemProperties = editableItemProperties(availableItemProperties);
  const visibleItemPropertiesDraft = itemPropertiesDraft
    .map((property, propertyIndex) => ({ property, propertyIndex }))
    .filter(({ property }) => editableItemProperties([property]).length > 0);

  return (
    <>
      <div className="panel-heading">
        <h2>{item ? itemLabel(item, itemIndex ?? 0) : "Item Detail"}</h2>
      </div>
      <PanelBody className="item-editor-body">
        {item ? (
          <div className="item-editor-grid">
            <section className="item-overview-column">
              <h3>Overview</h3>
              <FieldGrid>
                <Field label="Resref"><TextInput value={item.resref ?? ""} disabled /></Field>
                <Field label="Category"><TextInput value={item.category.label} disabled /></Field>
                {canEditStackSize ? (
                  <Field label="Stack Size">
                    <TextInput
                      value={metadataDraft.stack_size}
                      onChange={(event) => onMetadataChange({ stack_size: event.target.value })}
                      disabled={!canEdit || busy}
                    />
                  </Field>
                ) : null}
                <Field label="Wiki">
                  {item.wiki_url ? (
                    <a
                      className="field-link"
                      href={item.wiki_url}
                      onClick={(event) => {
                        event.preventDefault();
                        onWikiOpen(item.wiki_url!);
                      }}
                    >
                      Open item page
                    </a>
                  ) : (
                    <TextInput value="No wiki link" disabled />
                  )}
                </Field>
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
                <Field label="Item Level">
                  <TextInput
                    value={metadataDraft.item_level}
                    onChange={(event) => onMetadataChange({ item_level: event.target.value })}
                    disabled={!canEdit || busy}
                  />
                </Field>
              </FieldGrid>
              {allowRemove ? (
                <ButtonRow>
                  {canCloneBackpackItem ? (
                    <button onClick={onClone} disabled={!canEdit || busy}>Clone Item</button>
                  ) : null}
                  <button onClick={onRemove} disabled={!canEdit || busy}>Remove Item</button>
                </ButtonRow>
              ) : null}
            </section>
            <section className="properties-section">
              <div className="panel-heading"><h3>Properties</h3></div>
              <GridTable
                className="property-table"
                bodyClassName="property-list"
                header={(
                  <GridTableRow className="property-row property-header">
                    <span>Property</span>
                    <span>Power</span>
                    <span>Action</span>
                  </GridTableRow>
                )}
                body={visibleItemPropertiesDraft.map(({ property, propertyIndex }) => (
                  <GridTableRow key={`${property.id}-${propertyIndex}`} className="property-row">
                    <SelectInput
                      value={property.id}
                      onChange={(event) => onPropertyUpdate("id", propertyIndex, event.target.value)}
                      disabled={!canEdit || busy}
                    >
                      {editableAvailableItemProperties.map((option) => (
                        <option key={`existing-property-${propertyIndex}-${option.id}`} value={option.id}>
                          {option.name ?? `Property ${option.id}`}
                        </option>
                      ))}
                    </SelectInput>
                    <TextInput
                      value={property.power}
                      onChange={(event) => onPropertyUpdate("power", propertyIndex, event.target.value)}
                      disabled={!canEdit || busy}
                    />
                    <button onClick={() => onPropertyRemove(propertyIndex)} disabled={!canEdit || busy}>
                      Remove
                    </button>
                  </GridTableRow>
                ))}
                footer={(
                  <GridTableRow className="property-row add-property">
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
                    <TextInput
                      placeholder="Power"
                      value={propertyDraft.power}
                      onChange={(event) => onPropertyDraftChange({ power: event.target.value })}
                      disabled={!canEdit || busy}
                    />
                    <button onClick={onPropertyAdd} disabled={!canEdit || busy}>Add</button>
                  </GridTableRow>
                )}
              />
            </section>
          </div>
        ) : <EmptyState>Select an item to edit metadata and properties.</EmptyState>}
      </PanelBody>
    </>
  );
}
