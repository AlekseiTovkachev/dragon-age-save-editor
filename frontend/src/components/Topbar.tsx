import type { SaveSummary } from "../types";

type TopbarProps = {
  summary: SaveSummary | null;
  screenshotDataUrl: string | null;
  busy: boolean;
  onOpen: () => void;
  onSaveAs: () => void;
};

export function Topbar({ summary, screenshotDataUrl, busy, onOpen, onSaveAs }: TopbarProps) {
  return (
    <header className="topbar">
      <div>
        <h1>Dragon Age Save Editor</h1>
        <p>Open, edit, and save as a new file.</p>
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
        <button onClick={onOpen} disabled={busy}>Open Save</button>
        <button onClick={onSaveAs} disabled={busy || !summary}>Save As</button>
      </div>
    </header>
  );
}
