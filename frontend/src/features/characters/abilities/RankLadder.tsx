import { abilityLabel } from "../../../lib/abilityUtils";
import type { Ability, AbilityListKind } from "../../../types";
import type { AbilityTree } from "./TreeList";

type RankLadderProps = {
  kind: AbilityListKind;
  tree: AbilityTree | null;
  ownedIds: Set<number>;
  canEdit: boolean;
  busy: boolean;
  abilityIsLocked: (list: AbilityListKind, abilityId: number) => boolean;
  onAbilityAdd: (list: AbilityListKind, abilityId: number) => void;
  onAbilityRemove: (list: AbilityListKind, abilityId: number) => void;
};

export function RankLadder({
  kind,
  tree,
  ownedIds,
  canEdit,
  busy,
  abilityIsLocked,
  onAbilityAdd,
  onAbilityRemove,
}: RankLadderProps) {
  if (!tree) {
    return (
      <div className="ranks-panel">
        <div className="tree-list-empty">No ability tree selected.</div>
      </div>
    );
  }

  return (
    <div className="ranks-panel" aria-label={`${tree.label} ranks`}>
      <div className="card-head">
        <h3 className="card-title">{tree.label} ranks</h3>
        <span className="helptext">Locked ranks are required by another ability.</span>
      </div>
      {tree.abilities.map((ability, index) => {
        const owned = ownedIds.has(ability.id);
        const locked = owned && abilityIsLocked(kind, ability.id);
        return (
          <RankRow
            key={`${kind}-${ability.id}`}
            ability={ability}
            rank={index + 1}
            owned={owned}
            locked={locked}
            canEdit={canEdit}
            busy={busy}
            onAdd={() => onAbilityAdd(kind, ability.id)}
            onRemove={() => onAbilityRemove(kind, ability.id)}
          />
        );
      })}
    </div>
  );
}

type RankRowProps = {
  ability: Ability;
  rank: number;
  owned: boolean;
  locked: boolean;
  canEdit: boolean;
  busy: boolean;
  onAdd: () => void;
  onRemove: () => void;
};

function RankRow({ ability, rank, owned, locked, canEdit, busy, onAdd, onRemove }: RankRowProps) {
  return (
    <div className={["rank-row", owned ? "owned" : "", locked ? "locked" : ""].filter(Boolean).join(" ")}>
      <div className="rank-num">{rank}</div>
      <div className="rank-main">
        <div className="rank-name">{ability.name ?? `Ability ${ability.id}`}</div>
        <div className="rank-desc">{abilityLabel(ability)}</div>
      </div>
      {owned ? (
        <button
          type="button"
          onClick={onRemove}
          disabled={!canEdit || busy || locked}
          aria-label={`${locked ? "Required" : "Remove"} ${ability.name ?? `Ability ${ability.id}`}`}
        >
          {locked ? "Required" : "Remove"}
        </button>
      ) : (
        <button
          type="button"
          onClick={onAdd}
          disabled={!canEdit || busy}
          aria-label={`Add ${ability.name ?? `Ability ${ability.id}`}`}
        >
          Add
        </button>
      )}
    </div>
  );
}
