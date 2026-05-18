type ApplyOnSavePromptProps = {
  open: boolean;
  busy: boolean;
  onConfirm: () => void;
  onCancel: () => void;
};

export function ApplyOnSavePrompt({ open, busy, onConfirm, onCancel }: ApplyOnSavePromptProps) {
  if (!open) {
    return null;
  }

  return (
    <div className="warning-banner apply-save-prompt" role="alertdialog" aria-label="Apply drafts before saving">
      <span>You have unsaved drafts. Apply them before saving?</span>
      <div className="prompt-actions">
        <button type="button" className="dismiss-button" onClick={onConfirm} disabled={busy}>
          Apply drafts and save
        </button>
        <button type="button" className="dismiss-button" onClick={onCancel} disabled={busy}>
          Cancel
        </button>
      </div>
    </div>
  );
}
