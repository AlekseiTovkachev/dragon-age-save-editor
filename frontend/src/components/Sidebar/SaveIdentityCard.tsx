import { gameLabel } from "../../lib/format";
import type { SaveSummary } from "../../types";

type SaveIdentityCardProps = {
  summary: SaveSummary | null;
  screenshotDataUrl: string | null;
};

function saveName(summary: SaveSummary): string {
  const parts = summary.source_path.split(/[\\/]/);
  return parts[parts.length - 1] || summary.source_path;
}

export function SaveIdentityCard({ summary, screenshotDataUrl }: SaveIdentityCardProps) {
  const title = summary?.main_character_name || "No save loaded";
  const meta = summary ? `${gameLabel(summary.preferred_game)} - ${saveName(summary)}` : "Open a save to begin editing.";

  return (
    <div className="id-card" tabIndex={0}>
      <div className="shot" aria-label={summary ? "Save screenshot preview" : "No save screenshot"}>
        {screenshotDataUrl ? <img src={screenshotDataUrl} alt="Save screenshot" /> : null}
        <span className="shot-label">{summary ? gameLabel(summary.preferred_game) : "Dragon Age Save"}</span>
      </div>
      <h2>{title}</h2>
      <div className="meta" title={summary?.source_path || undefined}>
        {meta}
      </div>
      {summary ? (
        <div className="row-status">
          <span className={summary.dirty ? "chip blood" : "chip gold"}>
            <span className="dot" style={{ background: summary.dirty ? "var(--blood-2)" : "var(--gold)" }} aria-hidden="true" />
            {summary.dirty ? "Unsaved changes" : "Saved copy ready"}
          </span>
          <span className="mono muted">{summary.companion_count + 1} party</span>
        </div>
      ) : null}
      {summary && screenshotDataUrl ? (
        <div className="shot-popover">
          <div className="popover-shot">
            <img src={screenshotDataUrl} alt="Save screenshot full preview" />
          </div>
          <div className="mono muted">Preview of the selected save screenshot.</div>
        </div>
      ) : null}
    </div>
  );
}
