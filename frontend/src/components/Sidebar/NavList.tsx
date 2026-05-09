import { Flag, Package, ScrollText, UsersRound } from "lucide-react";
import { SECTION_TITLES } from "../../lib/navigation";
import type { Section } from "../../lib/navigation";

export type SectionCounts = Record<Section, number>;

type NavListProps = {
  sections: Section[];
  activeSection: Section;
  counts: SectionCounts;
  onSelect: (section: Section) => void;
};

const SECTION_ICONS = {
  characters: UsersRound,
  inventory: Package,
  recipes: ScrollText,
  plot_flags: Flag,
} satisfies Record<Section, typeof UsersRound>;

export function NavList({ sections, activeSection, counts, onSelect }: NavListProps) {
  return (
    <nav className="nav-list" aria-label="Editor sections">
      {sections.map((section) => {
        const Icon = SECTION_ICONS[section];
        const active = activeSection === section;
        return (
          <button
            type="button"
            key={section}
            className={active ? "nav-item is-active" : "nav-item"}
            aria-current={active ? "page" : undefined}
            onClick={() => onSelect(section)}
          >
            <span className="icn">
              <Icon size={18} strokeWidth={1.8} aria-hidden="true" />
            </span>
            {SECTION_TITLES[section]}
            <span className="count">{counts[section]}</span>
          </button>
        );
      })}
    </nav>
  );
}
