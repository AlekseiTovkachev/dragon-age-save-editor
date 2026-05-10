import { getCurrentWindow } from "@tauri-apps/api/window";

const isMac = navigator.userAgent.includes("Macintosh");
const isTauri = "__TAURI_INTERNALS__" in window;
const win = isTauri ? getCurrentWindow() : null;

const noop = () => {};

export function TitleBar() {
  const minimize = win ? () => void win.minimize() : noop;
  const toggleMax = win ? () => void win.toggleMaximize() : noop;
  const close = win ? () => void win.close() : noop;

  return (
    <div className="titlebar" data-tauri-drag-region>
      {isMac ? (
        <div className="titlebar-controls titlebar-mac">
          <button className="titlebar-btn mac-close" onClick={close} aria-label="Close" />
          <button className="titlebar-btn mac-minimize" onClick={minimize} aria-label="Minimize" />
          <button className="titlebar-btn mac-zoom" onClick={toggleMax} aria-label="Zoom" />
        </div>
      ) : null}
      <span className="titlebar-title" data-tauri-drag-region>
        Dragon Age Save Editor
      </span>
      {!isMac ? (
        <div className="titlebar-controls titlebar-win">
          <button className="titlebar-btn win-minimize" onClick={minimize} aria-label="Minimize">
            <svg width="10" height="1" viewBox="0 0 10 1" aria-hidden="true"><rect width="10" height="1" fill="currentColor"/></svg>
          </button>
          <button className="titlebar-btn win-maximize" onClick={toggleMax} aria-label="Maximize/Restore">
            <svg width="10" height="10" viewBox="0 0 10 10" fill="none" aria-hidden="true"><rect x="0.5" y="0.5" width="9" height="9" stroke="currentColor"/></svg>
          </button>
          <button className="titlebar-btn win-close" onClick={close} aria-label="Close">
            <svg width="10" height="10" viewBox="0 0 10 10" aria-hidden="true"><path d="M1 1l8 8M9 1l-8 8" stroke="currentColor" strokeWidth="1.2" strokeLinecap="round"/></svg>
          </button>
        </div>
      ) : null}
    </div>
  );
}
