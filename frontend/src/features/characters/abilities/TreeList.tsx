import type { Ability, AbilityListKind } from "../../../types";

export type AbilityTree = {
  id: string;
  label: string;
  abilities: Ability[];
};

type TreeListProps = {
  kind: AbilityListKind;
  trees: AbilityTree[];
  selectedTreeId: string;
  ownedIds: Set<number>;
  onSelect: (treeId: string) => void;
};

export function TreeList({ kind, trees, selectedTreeId, ownedIds, onSelect }: TreeListProps) {
  if (trees.length === 0) {
    return <div className="tree-list-empty">No matching trees.</div>;
  }

  return (
    <div className="tree-list" role="listbox" aria-label={`${kind} trees`}>
      {trees.map((tree) => {
        const owned = tree.abilities.filter((ability) => ownedIds.has(ability.id)).length;
        const total = tree.abilities.length;
        return (
          <button
            key={tree.id}
            className={["tree-row", tree.id === selectedTreeId ? "is-active" : ""].filter(Boolean).join(" ")}
            type="button"
            role="option"
            aria-selected={tree.id === selectedTreeId}
            onClick={() => onSelect(tree.id)}
          >
            <span>{tree.label}</span>
            <span className={["rank-pip", owned === total && total > 0 ? "full" : ""].filter(Boolean).join(" ")}>
              {owned}/{total}
            </span>
          </button>
        );
      })}
    </div>
  );
}

