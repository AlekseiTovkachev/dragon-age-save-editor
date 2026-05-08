import { ChevronDown, ChevronRight } from "lucide-react";
import {
  EmptyState,
  Field,
  NumericInput,
  ScrollRegion,
  SelectInput,
} from "../../components/ui";
import { abilityLabel, groupedAbilities } from "../../lib/abilityUtils";
import { isDirty } from "../../lib/dirty";
import { titleCase } from "../../lib/format";
import type { CharacterTab } from "../../lib/navigation";
import { useState } from "react";
import type { Dispatch, SetStateAction } from "react";
import type { Ability, AbilityListKind, Character, CharacterSummary } from "../../types";
import { InlineItemEditor } from "../inventory/InlineItemEditor";
import { InventoryTable } from "../inventory/InventoryTable";
import type { InventoryPanelActions, InventoryPanelState } from "../inventory/InventoryPanel";
import { CharacterHeader } from "./CharacterHeader";
import { CharacterSubtabs } from "./CharacterSubtabs";
import { PartyRail } from "./PartyRail";

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
  const overviewDirty = hasOverviewDirtyFields(state);
  const abilityDirty = hasAbilityDirtyFields(state);
  const characterDirty = overviewDirty || abilityDirty;

  return (
    <section className="char-layout">
      <PartyRail
        characters={state.characters}
        activeKey={state.characterKey}
        activeLevel={state.character?.level}
        activeDirty={characterDirty}
        onSelect={actions.setCharacterKey}
      />
      <div className="char-detail">
        {state.character ? (
          <>
            <div className="char-topline">
              <CharacterHeader character={state.character} dirty={characterDirty} />
              <CharacterSubtabs activeTab={characterTab} onSelect={setCharacterTab} />
            </div>

            {characterTab === "overview" ? (
              <CharacterOverview state={state} actions={actions} canEdit={canEdit} busy={busy} />
            ) : null}

            {characterTab === "abilities" ? (
              <CharacterAbilities state={state} actions={actions} canEdit={canEdit} busy={busy} />
            ) : null}

            {characterTab === "equipment" ? (
              <CharacterEquipment
                inventoryState={inventoryState}
                inventoryActions={inventoryActions}
                canEdit={canEdit}
                busy={busy}
              />
            ) : null}
          </>
        ) : (
          <div className="card-2">
            <EmptyState>Choose a character to edit.</EmptyState>
          </div>
        )}
      </div>
    </section>
  );
}

function CharacterEquipment({
  inventoryState,
  inventoryActions,
  canEdit,
  busy,
}: Pick<CharacterPanelProps, "inventoryState" | "inventoryActions" | "canEdit" | "busy">) {
  return (
    <section className="character-equipment-layout">
      <p className="muted editor-help">
        Items currently carried by this character. The save format doesn't track equipment slots, so items are listed
        flat - click any row to edit material, properties, and metadata.
      </p>
      <InventoryTable
        items={inventoryState.items}
        selectedIndex={inventoryState.itemIndex}
        onSelect={inventoryActions.setItemIndex}
        renderInlineEditor={() => (
          <InlineItemEditor
            state={inventoryState}
            actions={inventoryActions}
            canEdit={canEdit}
            busy={busy}
            allowBackpackActions={false}
          />
        )}
      />
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
    <div className="character-overview">
      <section className="card-2">
        <div className="card-head">
          <h3 className="card-title">Progress</h3>
        </div>
        <div className="grid-progress">
          <Field label="Level" className="num">
            <NumericInput
              className={isDirty(state.levelDraft, state.character?.level ?? null) ? "inp dirty" : "inp"}
              value={state.levelDraft}
              min={0}
              onChange={(event) => actions.setLevelDraft(event.target.value)}
              disabled={!canEdit || busy}
            />
          </Field>
          <Field label="Experience" className="num">
            <NumericInput
              className={isDirty(state.experienceDraft, state.character?.experience ?? null) ? "inp dirty" : "inp"}
              value={state.experienceDraft}
              min={0}
              onChange={(event) => actions.setExperienceDraft(event.target.value)}
              disabled={!canEdit || busy}
              placeholder={state.character?.experience === null ? "Add to save" : undefined}
            />
          </Field>
          {state.characterKey !== "main" ? (
            <Field label="Approval" className="num">
              <NumericInput
                className={isDirty(state.approvalDraft, state.character?.approval ?? null) ? "inp dirty" : "inp"}
                value={state.approvalDraft}
                min={0}
                onChange={(event) => actions.setApprovalDraft(event.target.value)}
                disabled={!canEdit || busy || state.character?.approval === null}
                placeholder={state.character?.approval === null ? "Unavailable for this character" : undefined}
              />
            </Field>
          ) : null}
        </div>
      </section>
      <section className="card-2">
        <div className="card-head">
          <h3 className="card-title">Attributes</h3>
        </div>
        <div className="grid-attrs">
          {Object.entries(state.statsDraft).map(([key, value]) => (
            <Field key={key} label={titleCase(key)} className="num">
              <NumericInput
                className={
                  isDirty(value, state.character?.core_stats[key as keyof typeof state.character.core_stats] ?? null)
                    ? "inp dirty"
                    : "inp"
                }
                value={value}
                min={0}
                onChange={(event) => actions.setStatsDraft((current) => ({ ...current, [key]: event.target.value }))}
                disabled={!canEdit || busy}
              />
            </Field>
          ))}
        </div>
      </section>
      <section className="card-2">
        <div className="card-head">
          <h3 className="card-title">Point Pools</h3>
        </div>
        <div className="grid-pools">
          {[ 
            ["attribute_points", "Attribute Points"],
            ["skill_points", "Skill Points"],
            ["talent_points", "Talent Points"],
            ["specialization_points", "Specialization Points"],
          ].map(([key, label]) => (
            <Field key={key} label={label} className="num">
              <NumericInput
                className={
                  isDirty(
                    state.pointPoolsDraft[key] ?? "",
                    state.character?.point_pools[key as keyof typeof state.character.point_pools] ?? null,
                  )
                    ? "inp dirty"
                    : "inp"
                }
                value={state.pointPoolsDraft[key] ?? ""}
                min={0}
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
        </div>
      </section>
    </div>
  );
}

function hasOverviewDirtyFields(state: CharacterPanelState) {
  if (!state.character) {
    return false;
  }
  return (
    isDirty(state.levelDraft, state.character.level) ||
    isDirty(state.experienceDraft, state.character.experience) ||
    isDirty(state.approvalDraft, state.character.approval) ||
    Object.entries(state.statsDraft).some(([key, value]) =>
      isDirty(value, state.character?.core_stats[key as keyof Character["core_stats"]] ?? null),
    ) ||
    Object.entries(state.pointPoolsDraft).some(([key, value]) =>
      isDirty(value, state.character?.point_pools[key as keyof Character["point_pools"]] ?? null),
    )
  );
}

function hasAbilityDirtyFields(state: CharacterPanelState) {
  if (!state.character) {
    return false;
  }
  return state.visibleAbilityKinds.some((kind) => {
    const committedIds = state.character?.[kind].map((ability) => ability.id).sort((a, b) => a - b) ?? [];
    const draftIds = state.abilityDrafts[kind].map((ability) => ability.id).sort((a, b) => a - b);
    return committedIds.length !== draftIds.length || committedIds.some((id, index) => id !== draftIds[index]);
  });
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
