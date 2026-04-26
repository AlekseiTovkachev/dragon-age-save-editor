import { ChevronDown, ChevronRight } from "lucide-react";
import { ItemEditor } from "../../components/ItemEditor";
import { ItemList } from "../../components/ItemList";
import {
  EmptyState,
  Field,
  FieldGrid,
  ListRow,
  Panel,
  PanelBody,
  ScrollRegion,
  SectionCard,
  SelectInput,
  TextInput,
} from "../../components/ui";
import { abilityLabel, groupedAbilities } from "../../lib/abilityUtils";
import { titleCase } from "../../lib/format";
import { targetKey } from "../../lib/itemUtils";
import { CHARACTER_TAB_TITLES, CHARACTER_TABS } from "../../lib/navigation";
import type { CharacterTab } from "../../lib/navigation";
import { useState } from "react";
import type { Dispatch, SetStateAction } from "react";
import type { Ability, AbilityListKind, Character, CharacterSummary } from "../../types";
import type { InventoryPanelActions, InventoryPanelState } from "../inventory/InventoryPanel";

type CharacterPanelProps = {
  state: CharacterPanelState;
  actions: CharacterPanelActions;
  inventoryState: InventoryPanelState;
  inventoryActions: InventoryPanelActions;
  characterTab: CharacterTab;
  setCharacterTab: (tab: CharacterTab) => void;
  canEdit: boolean;
  busy: boolean;
};

export type CharacterPanelState = {
  characters: CharacterSummary[];
  characterKey: string;
  character: Character | null;
  levelDraft: string;
  experienceDraft: string;
  approvalDraft: string;
  statsDraft: Record<string, string>;
  pointPoolsDraft: Record<string, string>;
  visibleAbilityKinds: AbilityListKind[];
  selectedAbilityToAdd: Record<AbilityListKind, string>;
  availableAbilities: Record<AbilityListKind, Ability[]>;
  abilityDrafts: Record<AbilityListKind, Ability[]>;
};

export type CharacterPanelActions = {
  setCharacterKey: (key: string) => void;
  setLevelDraft: (value: string) => void;
  setExperienceDraft: (value: string) => void;
  setApprovalDraft: (value: string) => void;
  setStatsDraft: Dispatch<SetStateAction<Record<string, string>>>;
  setPointPoolsDraft: Dispatch<SetStateAction<Record<string, string>>>;
  coreAbilityOptions: (list: AbilityListKind) => Ability[];
  visibleTreeAbilities: (list: AbilityListKind) => Ability[];
  setSelectedAbilityToAdd: Dispatch<SetStateAction<Record<AbilityListKind, string>>>;
  handleAbilityAdd: (list: AbilityListKind) => void;
  visibleAbilities: (list: AbilityListKind, abilities: Ability[]) => Ability[];
  abilityIsLocked: (list: AbilityListKind, abilityId: number) => boolean;
  handleAbilityRemove: (list: AbilityListKind, abilityId: number) => void;
  handleVisibleAbilityAdd: (list: AbilityListKind, abilityId: number) => void;
};

export function CharacterPanel({
  state,
  actions,
  inventoryState,
  inventoryActions,
  characterTab,
  setCharacterTab,
  canEdit,
  busy,
}: CharacterPanelProps) {
  return (
    <section className="split-section">
      <Panel className="list-panel" title="Party">
        {state.characters.map((entry) => (
          <ListRow
            key={targetKey(entry.target)}
            active={targetKey(entry.target) === state.characterKey}
            onClick={() => actions.setCharacterKey(targetKey(entry.target))}
          >
            {entry.name}
          </ListRow>
        ))}
      </Panel>
      <Panel className="detail-panel" scroll>
        <div className="panel-heading character-heading">
          <h2>{state.character?.name ?? "Character"}</h2>
          <div className="character-tabbar">
            {CHARACTER_TABS.map((tab) => (
              <button
                type="button"
                key={tab}
                className={characterTab === tab ? "nav-link active" : "nav-link"}
                onMouseDown={(event) => event.preventDefault()}
                onClick={() => setCharacterTab(tab)}
              >
                {CHARACTER_TAB_TITLES[tab]}
              </button>
            ))}
          </div>
        </div>
        <PanelBody>
          {state.character ? (
            <>
              {characterTab === "overview" ? (
                <CharacterOverview state={state} actions={actions} canEdit={canEdit} busy={busy} />
              ) : null}

              {characterTab === "abilities" ? (
                <CharacterAbilities state={state} actions={actions} canEdit={canEdit} busy={busy} />
              ) : null}

              {characterTab === "equipment" ? (
                <div className="character-equipment-layout">
                  <ItemList items={inventoryState.items} selectedIndex={inventoryState.itemIndex} onSelect={inventoryActions.setItemIndex} />
                  <div className="equipment-detail scroll-panel">
                    <ItemEditor
                      item={inventoryState.selectedItem}
                      itemIndex={inventoryState.itemIndex}
                      canEdit={canEdit}
                      busy={busy}
                      allowRemove={false}
                      canEditStackSize={inventoryState.canEditStackSize}
                      canCloneBackpackItem={inventoryState.canCloneBackpackItem}
                      canEditMaterial={inventoryState.canEditMaterial}
                      metadataDraft={inventoryState.itemMetadataDraft}
                      propertyDraft={inventoryState.propertyDraft}
                      itemPropertiesDraft={inventoryState.itemPropertiesDraft}
                      availableItemProperties={inventoryState.availableItemProperties}
                      onMetadataChange={(patch) => inventoryActions.setItemMetadataDraft((current) => ({ ...current, ...patch }))}
                      onPropertyDraftChange={(patch) => inventoryActions.setPropertyDraft((current) => ({ ...current, ...patch }))}
                      onPropertyAdd={inventoryActions.handlePropertyAddDraft}
                      onPropertyRemove={inventoryActions.handlePropertyRemoveDraft}
                      onPropertyUpdate={inventoryActions.handlePropertyUpdateDraft}
                      onRemove={() => void inventoryActions.handleBackpackRemove()}
                      onClone={() => void inventoryActions.handleBackpackClone()}
                      onWikiOpen={(url) => void inventoryActions.handleWikiOpen(url)}
                    />
                  </div>
                </div>
              ) : null}
            </>
          ) : <EmptyState>Choose a character to edit.</EmptyState>}
        </PanelBody>
      </Panel>
    </section>
  );
}

function CharacterOverview({
  state,
  actions,
  canEdit,
  busy,
}: Pick<CharacterPanelProps, "state" | "actions" | "canEdit" | "busy">) {
  return (
    <>
      <SectionCard title="Progress" className="character-field-section">
        <FieldGrid>
          <Field label="Level">
            <TextInput
              value={state.levelDraft}
              onChange={(event) => actions.setLevelDraft(event.target.value)}
              disabled={!canEdit || busy}
            />
          </Field>
          <Field label="Experience">
            <TextInput
              value={state.experienceDraft}
              onChange={(event) => actions.setExperienceDraft(event.target.value)}
              disabled={!canEdit || busy}
              placeholder={state.character?.experience === null ? "Add to save" : undefined}
            />
          </Field>
          {state.characterKey !== "main" ? (
            <Field label="Approval">
              <TextInput
                value={state.approvalDraft}
                onChange={(event) => actions.setApprovalDraft(event.target.value)}
                disabled={!canEdit || busy || state.character?.approval === null}
                placeholder={state.character?.approval === null ? "Unavailable for this character" : undefined}
              />
            </Field>
          ) : null}
        </FieldGrid>
      </SectionCard>
      <SectionCard title="Attributes" className="character-field-section">
        <FieldGrid>
          {Object.entries(state.statsDraft).map(([key, value]) => (
            <Field key={key} label={titleCase(key)}>
              <TextInput
                value={value}
                onChange={(event) => actions.setStatsDraft((current) => ({ ...current, [key]: event.target.value }))}
                disabled={!canEdit || busy}
              />
            </Field>
          ))}
        </FieldGrid>
      </SectionCard>
      <SectionCard title="Point Pools" className="character-field-section">
        <FieldGrid>
          {[ 
            ["attribute_points", "Attribute Points"],
            ["skill_points", "Skill Points"],
            ["talent_points", "Talent Points"],
            ["specialization_points", "Specialization Points"],
          ].map(([key, label]) => (
            <Field key={key} label={label}>
              <TextInput
                value={state.pointPoolsDraft[key] ?? ""}
                onChange={(event) =>
                  actions.setPointPoolsDraft((current) => ({ ...current, [key]: event.target.value }))
                }
                disabled={!canEdit || busy}
                placeholder={
                  state.character?.point_pools[key as keyof typeof state.character.point_pools] === null
                    ? "Add to save"
                    : undefined
                }
              />
            </Field>
          ))}
        </FieldGrid>
      </SectionCard>
    </>
  );
}

function CharacterAbilities({
  state,
  actions,
  canEdit,
  busy,
}: Pick<CharacterPanelProps, "state" | "actions" | "canEdit" | "busy">) {
  const [expandedGroups, setExpandedGroups] = useState<Record<string, boolean>>({});

  return (
    <>
      <div className="panel-heading">
        <h2>Ability Lists</h2>
      </div>
      <div className="ability-grid">
        {state.visibleAbilityKinds.map((kind) => {
          const abilityOptions = actions.coreAbilityOptions(kind);
          const treeAbilities = actions.visibleTreeAbilities(kind);
          const groups = groupedAbilities(
            kind,
            treeAbilities,
            actions.visibleAbilities(kind, state.availableAbilities[kind]),
          );
          const selectedAbilityValue = abilityOptions.some(
            (ability) => ability.id.toString() === state.selectedAbilityToAdd[kind],
          )
            ? state.selectedAbilityToAdd[kind]
            : abilityOptions[0]?.id.toString() ?? "";
          return (
            <div key={kind} className="ability-panel">
              <div className="panel-heading">
                <h3>{titleCase(kind)}</h3>
              </div>
              <div className="ability-add-row">
                <SelectInput
                  value={selectedAbilityValue}
                  onChange={(event) =>
                    actions.setSelectedAbilityToAdd((current) => ({ ...current, [kind]: event.target.value }))
                  }
                  disabled={!canEdit || busy}
                >
                  {abilityOptions.map((ability) => (
                    <option key={`${kind}-available-${ability.id}`} value={ability.id}>
                      {abilityLabel(ability)}
                    </option>
                  ))}
                </SelectInput>
                <button onClick={() => actions.handleAbilityAdd(kind)} disabled={!canEdit || busy || abilityOptions.length === 0}>
                  Add
                </button>
              </div>
              <p className="muted editor-help">
                Add core tree entries from the dropdown. Unlocked trees show addable and removable abilities below.
              </p>
              <ScrollRegion className="ability-preview">
                {groups.map((group) => {
                  const groupKey = `${kind}-${group.label}`;
                  const isExpanded = expandedGroups[groupKey] ?? false;
                  return (
                    <AbilityGroup
                      key={groupKey}
                      group={group}
                      kind={kind}
                      expanded={isExpanded}
                      selectedAbilities={state.abilityDrafts[kind]}
                      canEdit={canEdit}
                      busy={busy}
                      onToggle={() =>
                        setExpandedGroups((current) => ({ ...current, [groupKey]: !isExpanded }))
                      }
                      abilityIsLocked={actions.abilityIsLocked}
                      onAbilityAdd={actions.handleVisibleAbilityAdd}
                      onAbilityRemove={actions.handleAbilityRemove}
                    />
                  );
                })}
              </ScrollRegion>
            </div>
          );
        })}
      </div>
    </>
  );
}

type AbilityGroupProps = {
  group: {
    label: string;
    abilities: Ability[];
  };
  kind: AbilityListKind;
  expanded: boolean;
  selectedAbilities: Ability[];
  canEdit: boolean;
  busy: boolean;
  onToggle: () => void;
  abilityIsLocked: (list: AbilityListKind, abilityId: number) => boolean;
  onAbilityAdd: (list: AbilityListKind, abilityId: number) => void;
  onAbilityRemove: (list: AbilityListKind, abilityId: number) => void;
};

function AbilityGroup({
  group,
  kind,
  expanded,
  selectedAbilities,
  canEdit,
  busy,
  onToggle,
  abilityIsLocked,
  onAbilityAdd,
  onAbilityRemove,
}: AbilityGroupProps) {
  return (
    <div className="ability-group">
      <button
        className="ability-group-toggle"
        type="button"
        aria-expanded={expanded}
        onClick={onToggle}
      >
        <span className="ability-group-title">
          {expanded ? (
            <ChevronDown size={16} strokeWidth={2.2} aria-hidden="true" />
          ) : (
            <ChevronRight size={16} strokeWidth={2.2} aria-hidden="true" />
          )}
          {group.label}
        </span>
        <span className="badge">{group.abilities.length}</span>
      </button>
      {expanded ? (
        <div className="ability-group-body">
          {group.abilities.map((ability) => {
            const selected = selectedAbilities.some((entry) => entry.id === ability.id);
            const locked = selected && abilityIsLocked(kind, ability.id);
            return (
              <div key={`${kind}-${ability.id}`} className="ability-entry">
                <div className="ability-entry-header">
                  <strong>{ability.name ?? `Ability ${ability.id}`}</strong>
                  {selected ? (
                    <button
                      onClick={() => onAbilityRemove(kind, ability.id)}
                      disabled={!canEdit || busy || locked}
                    >
                      Remove
                    </button>
                  ) : (
                    <button
                      onClick={() => onAbilityAdd(kind, ability.id)}
                      disabled={!canEdit || busy}
                    >
                      Add
                    </button>
                  )}
                </div>
                <span>{abilityLabel(ability)}</span>
                {locked ? (
                  <span className="muted">Required by another selected ability.</span>
                ) : null}
              </div>
            );
          })}
        </div>
      ) : null}
    </div>
  );
}
