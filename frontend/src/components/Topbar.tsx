import { FolderOpen, RotateCcw, Save, ShieldCheck } from "lucide-react";
import { gameLabel } from "../lib/format";
import type { SaveSummary } from "../types";

type TopbarProps = {
  summary: SaveSummary | null;
  screenshotDataUrl: string | null;
  busy: boolean;
  onOpen: () => void;
  onSaveAs: () => void;
  onCommitDrafts: () => void;
  onResetDrafts: () => void;
};

function saveName(summary: SaveSummary): string {
  const parts = summary.source_path.split(/[\\/]/);
  return parts[parts.length - 1] || summary.source_path;
}

export function Topbar({
  summary,
  screenshotDataUrl,
  busy,
  onOpen,
  onSaveAs,
  onCommitDrafts,
  onResetDrafts,
}: TopbarProps) {
  return (
    <header className="topbar">
      <div className="topbar-title">
        <h1>Dragon Age Save Editor</h1>
        <p>{summary ? `${gameLabel(summary.preferred_game)} - ${saveName(summary)}` : "Open a save to begin editing."}</p>
        {summary ? <small>{summary.source_path}</small> : null}
      </div>
      <div className="topbar-document">
        {summary ? (
          <>
            <span>{summary.main_character_name}</span>
            <small>{summary.dirty ? "Unsaved changes" : "Saved copy ready"}</small>
          </>
        ) : (
          <>
            <span>No save loaded</span>
            <small>Original files stay untouched</small>
          </>
        )}
      </div>
      <div className="toolbar">
        {summary && screenshotDataUrl ? (
          <div className="topbar-preview" tabIndex={0}>
            <img className="topbar-preview-image" src={screenshotDataUrl} alt="Save screenshot" />
            <div className="topbar-preview-popover">
              <img src={screenshotDataUrl} alt="Save screenshot full preview" />
            </div>
          </div>
        ) : null}
        {summary && !screenshotDataUrl ? <span className="topbar-preview-empty">No screenshot</span> : null}
        <button className="button-secondary" onClick={onOpen} disabled={busy}>
          <FolderOpen size={17} aria-hidden="true" />
          Open Save
        </button>
        {summary ? (
          <>
            <button className="button-secondary" onClick={onCommitDrafts} disabled={busy}>
              <ShieldCheck size={17} aria-hidden="true" />
              Commit Changes
            </button>
            <button className="button-secondary" onClick={onResetDrafts} disabled={busy}>
              <RotateCcw size={17} aria-hidden="true" />
              Reset Drafts
            </button>
          </>
        ) : null}
        <button className="button-primary" onClick={onSaveAs} disabled={busy || !summary || !summary.dirty}>
          <Save size={17} aria-hidden="true" />
          Save As
        </button>
      </div>
    </header>
  );
}
