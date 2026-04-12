import { SECTION_TITLES } from "../lib/navigation";
import type { Section } from "../lib/navigation";

type MainTabsProps = {
  sections: Section[];
  activeSection: Section;
  onSelect: (section: Section) => void;
};

export function MainTabs({ sections, activeSection, onSelect }: MainTabsProps) {
  return (
    <nav className="main-tabbar">
      {sections.map((entry) => (
        <button
          key={entry}
          className={activeSection === entry ? "nav-link active" : "nav-link"}
          onClick={() => onSelect(entry)}
        >
          {SECTION_TITLES[entry]}
        </button>
      ))}
    </nav>
  );
}
