import { targetKey } from "../../lib/itemUtils";
import type { CharacterSummary } from "../../types";

type PartyRailProps = {
  characters: CharacterSummary[];
  activeKey: string;
  activeDirty: boolean;
  onSelect: (key: string) => void;
};

export function PartyRail({ characters, activeKey, activeDirty, onSelect }: PartyRailProps) {
  return (
    <aside className="party-rail" aria-label="Party members">
      <div className="rail-label">Party</div>
      {characters.map((entry) => {
        const key = targetKey(entry.target);
        const active = key === activeKey;
        const roleLabel = entry.target === "main_character" ? "Hero" : "Companion";
        const classes = ["party-card", active ? "is-active" : "", active && activeDirty ? "dirty" : ""]
          .filter(Boolean)
          .join(" ");

        return (
          <button key={key} type="button" className={classes} onClick={() => onSelect(key)}>
            <span className="party-card-main">
              <span className="nm">{entry.name}</span>
              <span className="ro">{active ? "Selected" : roleLabel}</span>
            </span>
            <span className="pip" aria-hidden="true" />
          </button>
        );
      })}
    </aside>
  );
}
