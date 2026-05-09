import type { AbilityListKind } from "../../../types";

const KIND_LABELS: Record<AbilityListKind, string> = {
  skills: "Skills",
  talents: "Talents",
  spells: "Spells",
};

type KindTabsProps = {
  kinds: AbilityListKind[];
  activeKind: AbilityListKind;
  counts: Record<AbilityListKind, number>;
  onSelect: (kind: AbilityListKind) => void;
};

export function KindTabs({ kinds, activeKind, counts, onSelect }: KindTabsProps) {
  return (
    <nav className="subtabs ability-kind-tabs" aria-label="Ability lists">
      {kinds.map((kind) => (
        <button
          key={kind}
          className={kind === activeKind ? "is-active" : ""}
          type="button"
          onClick={() => onSelect(kind)}
        >
          {KIND_LABELS[kind]} <span className="mono muted">{counts[kind]}</span>
        </button>
      ))}
    </nav>
  );
}

