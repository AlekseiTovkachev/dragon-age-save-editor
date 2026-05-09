import { CHARACTER_TAB_TITLES, CHARACTER_TABS, type CharacterTab } from "../../lib/navigation";

type CharacterSubtabsProps = {
  activeTab: CharacterTab;
  onSelect: (tab: CharacterTab) => void;
};

export function CharacterSubtabs({ activeTab, onSelect }: CharacterSubtabsProps) {
  return (
    <nav className="subtabs" aria-label="Character sections">
      {CHARACTER_TABS.map((tab) => (
        <button
          type="button"
          key={tab}
          className={activeTab === tab ? "is-active" : undefined}
          aria-current={activeTab === tab ? "page" : undefined}
          onMouseDown={(event) => event.preventDefault()}
          onClick={() => onSelect(tab)}
        >
          {CHARACTER_TAB_TITLES[tab]}
        </button>
      ))}
    </nav>
  );
}
