import { targetKey } from "../../lib/itemUtils";
import type { CharacterSummary } from "../../types";

type PartyRailProps = {
  characters: CharacterSummary[];
  activeKey: string;
  activeLevel: number | null | undefined;
  activeDirty: boolean;
  onSelect: (key: string) => void;
};

export function PartyRail({ characters, activeKey, activeLevel, activeDirty, onSelect }: PartyRailProps) {
  return (
    <aside className="party-rail" aria-label="Party members">
      <div className="rail-label">Party</div>
      {characters.map((entry) => {
        const key = targetKey(entry.target);
        const active = key === activeKey;
        const levelLabel = active && activeLevel !== null && activeLevel !== undefined ? `L${activeLevel}` : "L-";
        const classes = ["party-card", active ? "is-active" : "", active && activeDirty ? "dirty" : ""]
          .filter(Boolean)
          .join(" ");

        return (
          <button key={key} type="button" className={classes} onClick={() => onSelect(key)}>
            <span className="badge-lvl" aria-label={levelLabel}>
              {levelLabel}
            </span>
            <span className="party-card-main">
              <span className="nm">{entry.name}</span>
              <span className="ro">{active ? "Selected" : "Companion"}</span>
            </span>
            <span className="pip" aria-hidden="true" />
          </button>
        );
      })}
    </aside>
  );
}
