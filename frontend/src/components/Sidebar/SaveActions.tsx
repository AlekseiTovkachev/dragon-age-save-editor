import { FolderOpen, RotateCcw, Save, ShieldCheck } from "lucide-react";

type SaveActionsProps = {
  hasSummary: boolean;
  dirty: boolean;
  busy: boolean;
  hasPlotWarnings: boolean;
  onOpen: () => void;
  onSaveAs: () => void;
  onCommitDrafts: () => void;
  onResetDrafts: () => void;
};

export function SaveActions({
  hasSummary,
  dirty,
  busy,
  hasPlotWarnings,
  onOpen,
  onSaveAs,
  onCommitDrafts,
  onResetDrafts,
}: SaveActionsProps) {
  return (
    <div className="nav-list" style={{ paddingBottom: "0.85rem" }}>
      <button type="button" className="nav-item" onClick={onOpen} disabled={busy}>
        <span className="icn">
          <FolderOpen size={18} strokeWidth={1.8} aria-hidden="true" />
        </span>
        Open Save...
      </button>
      {hasSummary ? (
        <>
          <button type="button" className="nav-item" onClick={onCommitDrafts} disabled={busy || hasPlotWarnings}>
            <span className="icn">
              <ShieldCheck size={18} strokeWidth={1.8} aria-hidden="true" />
            </span>
            Apply Drafts
          </button>
          <button type="button" className="nav-item" onClick={onResetDrafts} disabled={busy}>
            <span className="icn">
              <RotateCcw size={18} strokeWidth={1.8} aria-hidden="true" />
            </span>
            Reset Drafts
          </button>
        </>
      ) : null}
      <button type="button" className="nav-item danger" onClick={onSaveAs} disabled={busy || !hasSummary || !dirty}>
        <span className="icn">
          <Save size={18} strokeWidth={1.8} aria-hidden="true" />
        </span>
        Save As...
      </button>
    </div>
  );
}
