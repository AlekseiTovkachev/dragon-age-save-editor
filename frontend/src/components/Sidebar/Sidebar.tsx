import type { SaveSummary } from "../../types";
import type { Section } from "../../lib/navigation";
import { NavList, type SectionCounts } from "./NavList";
import { SaveActions } from "./SaveActions";
import { SaveIdentityCard } from "./SaveIdentityCard";

type SidebarProps = {
  summary: SaveSummary | null;
  screenshotDataUrl: string | null;
  sections: Section[];
  activeSection: Section;
  sectionCounts: SectionCounts;
  busy: boolean;
  onSectionSelect: (section: Section) => void;
  onOpen: () => void;
  onSaveAs: () => void;
  onCommitDrafts: () => void;
  onResetDrafts: () => void;
};

export function Sidebar({
  summary,
  screenshotDataUrl,
  sections,
  activeSection,
  sectionCounts,
  busy,
  onSectionSelect,
  onOpen,
  onSaveAs,
  onCommitDrafts,
  onResetDrafts,
}: SidebarProps) {
  return (
    <aside className="app-sidebar" aria-label="Save editor navigation">
      <SaveIdentityCard summary={summary} screenshotDataUrl={screenshotDataUrl} />

      <div className="nav-section">Edit</div>
      <NavList sections={sections} activeSection={activeSection} counts={sectionCounts} onSelect={onSectionSelect} />

      <div className="nav-spacer" />

      <div className="nav-section">Save</div>
      <SaveActions
        hasSummary={Boolean(summary)}
        dirty={Boolean(summary?.dirty)}
        busy={busy}
        onOpen={onOpen}
        onSaveAs={onSaveAs}
        onCommitDrafts={onCommitDrafts}
        onResetDrafts={onResetDrafts}
      />
    </aside>
  );
}
