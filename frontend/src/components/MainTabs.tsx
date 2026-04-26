import { Flag, Package, ScrollText, UsersRound } from "lucide-react";
import { SECTION_TITLES } from "../lib/navigation";
import type { Section } from "../lib/navigation";

type MainTabsProps = {
  sections: Section[];
  activeSection: Section;
  onSelect: (section: Section) => void;
  moneyDraft?: string;
  onMoneyChange?: (value: string) => void;
  canEditMoney?: boolean;
  busy?: boolean;
};

const SECTION_ICONS = {
  characters: UsersRound,
  inventory: Package,
  recipes: ScrollText,
  plot_flags: Flag,
} satisfies Record<Section, typeof UsersRound>;

export function MainTabs({
  sections,
  activeSection,
  onSelect,
  moneyDraft,
  onMoneyChange,
  canEditMoney = false,
  busy = false,
}: MainTabsProps) {
  return (
    <div className="navigation-row">
      <nav className="main-tabbar">
        {sections.map((entry) => (
          <button
            type="button"
            key={entry}
            className={activeSection === entry ? "nav-link active" : "nav-link"}
            onMouseDown={(event) => event.preventDefault()}
            onClick={() => onSelect(entry)}
          >
            {(() => {
              const Icon = SECTION_ICONS[entry];
              return <Icon size={18} strokeWidth={2} aria-hidden="true" />;
            })()}
            {SECTION_TITLES[entry]}
          </button>
        ))}
      </nav>
      {moneyDraft !== undefined && onMoneyChange ? (
        <label className="tab-money-control">
          <span>Money</span>
          <input
            value={moneyDraft}
            onChange={(event) => onMoneyChange(event.target.value)}
            disabled={!canEditMoney || busy}
          />
        </label>
      ) : null}
    </div>
  );
}
