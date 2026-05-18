import { EmptyState, Field, NumericInput } from "../../components/ui";
import { isDirty } from "../../lib/dirty";
import { titleCase } from "../../lib/format";
import type { CharacterTab } from "../../lib/navigation";
import type { Dispatch, SetStateAction } from "react";
import type { Ability, AbilityListKind, Character, CharacterSummary } from "../../types";
import { InlineItemEditor } from "../inventory/InlineItemEditor";
import { InventoryTable } from "../inventory/InventoryTable";
import type { InventoryPanelActions, InventoryPanelState } from "../inventory/InventoryPanel";
import { AbilitiesPanel } from "./abilities/AbilitiesPanel";
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
  isDa2: boolean;
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
              <AbilitiesPanel state={state} actions={actions} canEdit={canEdit} busy={busy} />
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
                min={-100}
                max={100}
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
          {([
            ["attribute_points", "Attribute Points"],
            ["skill_points", "Skill Points"],
            ["talent_points", "Talent Points"],
            ["specialization_points", "Specialization Points"],
          ] as [string, string][]).filter(([key]) => !(state.isDa2 && key === "skill_points")).map(([key, label]) => (
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
