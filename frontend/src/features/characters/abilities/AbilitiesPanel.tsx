import { useMemo, useState } from "react";
import { groupedAbilities } from "../../../lib/abilityUtils";
import type { Ability, AbilityListKind, Character } from "../../../types";
import type { CharacterPanelActions, CharacterPanelState } from "../CharacterPanel";
import { KindTabs } from "./KindTabs";
import { RankLadder } from "./RankLadder";
import { TreeList } from "./TreeList";
import type { AbilityTree } from "./TreeList";

type AbilitiesPanelProps = {
  state: CharacterPanelState;
  actions: CharacterPanelActions;
  canEdit: boolean;
  busy: boolean;
};

export function AbilitiesPanel({ state, actions, canEdit, busy }: AbilitiesPanelProps) {
  const [activeKindDraft, setActiveKindDraft] = useState<AbilityListKind>(state.visibleAbilityKinds[0] ?? "skills");
  const [search, setSearch] = useState("");
  const [selectedTreeIdDraft, setSelectedTreeIdDraft] = useState("");
  const activeKind = state.visibleAbilityKinds.includes(activeKindDraft)
    ? activeKindDraft
    : state.visibleAbilityKinds[0] ?? "skills";

  const treesByKind = useMemo(() => {
    return Object.fromEntries(
      state.visibleAbilityKinds.map((kind) => [kind, abilityTrees(kind, state, actions)]),
    ) as Partial<Record<AbilityListKind, AbilityTree[]>>;
  }, [actions, state]);

  const filteredTrees = useMemo(() => filterTrees(treesByKind[activeKind] ?? [], search), [
    activeKind,
    search,
    treesByKind,
  ]);
  const ownedIds = useMemo(() => new Set(state.abilityDrafts[activeKind].map((ability) => ability.id)), [
    activeKind,
    state.abilityDrafts,
  ]);
  const visibleSelectedTree = filteredTrees.some((tree) => tree.id === selectedTreeIdDraft)
    ? selectedTreeIdDraft
    : filteredTrees[0]?.id ?? "";
  const selectedTree = filteredTrees.find((tree) => tree.id === visibleSelectedTree) ?? null;
  const pointChip = pointChipLabel(activeKind, state.character?.point_pools ?? null);

  if (state.visibleAbilityKinds.length === 0) {
    return (
      <section className="abilities-layout">
        <div className="card-2">No ability lists are available for this character.</div>
      </section>
    );
  }

  return (
    <section className="abilities-layout">
      <div className="abilities-toolbar">
        <KindTabs
          kinds={state.visibleAbilityKinds}
          activeKind={activeKind}
          counts={abilityCounts(state.abilityDrafts, actions.visibleTreeAbilities)}
          onSelect={(kind) => {
            setActiveKindDraft(kind);
            setSelectedTreeIdDraft("");
          }}
        />
        <input
          className="search-input ability-search"
          type="search"
          value={search}
          onChange={(event) => setSearch(event.target.value)}
          placeholder="Search abilities..."
          aria-label="Search abilities"
        />
        <span className="chip" aria-label={pointChip.ariaLabel}>
          {pointChip.label}: <strong className="mono">{pointChip.value}</strong>
        </span>
      </div>
      <div className="abilities-grid">
        <TreeList
          kind={activeKind}
          trees={filteredTrees}
          selectedTreeId={visibleSelectedTree}
          ownedIds={ownedIds}
          onSelect={setSelectedTreeIdDraft}
        />
        <RankLadder
          kind={activeKind}
          isDa2={state.isDa2}
          tree={selectedTree}
          ownedIds={ownedIds}
          canEdit={canEdit}
          busy={busy}
          abilityIsLocked={actions.abilityIsLocked}
          onAbilityAdd={actions.handleVisibleAbilityAdd}
          onAbilityRemove={actions.handleAbilityRemove}
        />
      </div>
    </section>
  );
}

function abilityTrees(kind: AbilityListKind, state: CharacterPanelState, actions: CharacterPanelActions): AbilityTree[] {
  const treeAbilities = actions.visibleTreeAbilities(kind);
  const visibleAvailable = actions.visibleAbilities(kind, state.availableAbilities[kind]);
  const byId = new Map<number, Ability>();
  for (const ability of [...treeAbilities, ...visibleAvailable]) {
    byId.set(ability.id, ability);
  }

  const isCompanion = state.characterKey !== "main";
  return groupedAbilities(kind, Array.from(byId.values()), visibleAvailable)
    .map((group) => ({
      id: `${kind}-${group.label}`,
      label: group.label,
      abilities: group.abilities,
    }))
    .filter((tree) => !(isCompanion && tree.label.startsWith("Other ")));
}

function filterTrees(trees: AbilityTree[], search: string) {
  const query = search.trim().toLowerCase();
  if (!query) {
    return trees;
  }
  return trees.filter((tree) => {
    return (
      tree.label.toLowerCase().includes(query) ||
      tree.abilities.some((ability) => {
        return (
          (ability.name ?? `Ability ${ability.id}`).toLowerCase().includes(query) ||
          ability.id.toString().includes(query)
        );
      })
    );
  });
}

function abilityCounts(
  abilityDrafts: Record<AbilityListKind, Ability[]>,
  visibleTreeFn: (list: AbilityListKind) => Ability[],
): Record<AbilityListKind, number> {
  // Count owned abilities that pass through the full visible-tree filter (DB-known check included).
  const ownedSkills = new Set(abilityDrafts.skills.map((a) => a.id));
  const ownedTalents = new Set(abilityDrafts.talents.map((a) => a.id));
  const ownedSpells = new Set(abilityDrafts.spells.map((a) => a.id));
  return {
    skills: visibleTreeFn("skills").filter((a) => ownedSkills.has(a.id)).length,
    talents: visibleTreeFn("talents").filter((a) => ownedTalents.has(a.id)).length,
    spells: visibleTreeFn("spells").filter((a) => ownedSpells.has(a.id)).length,
  };
}

function pointChipLabel(kind: AbilityListKind, pointPools: Character["point_pools"] | null) {
  if (kind === "skills") {
    return {
      label: "Skill pts",
      value: pointPools?.skill_points ?? 0,
      ariaLabel: "Skill points",
    };
  }
  if (kind === "talents") {
    return {
      label: "Talent pts / Spec pts",
      value: `${pointPools?.talent_points ?? 0} / ${pointPools?.specialization_points ?? 0}`,
      ariaLabel: "Talent and specialization points",
    };
  }
  return {
    label: "Spell pts",
    value: 0,
    ariaLabel: "Spell points",
  };
}
