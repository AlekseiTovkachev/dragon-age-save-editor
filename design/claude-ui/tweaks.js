// Vanilla tweaks panel — speaks the host edit-mode protocol.

const TWEAK_DEFAULTS = /*EDITMODE-BEGIN*/{
  "showPartyClass": false,
  "showPartyLevelBadge": true,
  "showDirtyPip": true,
  "compactPartyRail": false,
  "showItemResref": true,
  "accent": "#c9a64a"
}/*EDITMODE-END*/;

const ACCENT_OPTIONS = [
  { value: "#c9a64a", label: "Gold (DAO)" },
  { value: "#b3302a", label: "Blood" },
  { value: "#5b8aa8", label: "Tevinter" },
  { value: "#7fa37a", label: "Veilfire" },
];

window.TWEAKS = { ...TWEAK_DEFAULTS };
const subscribers = new Set();
window.onTweaksChange = (fn) => { subscribers.add(fn); return () => subscribers.delete(fn); };

function applyAccent(hex) {
  document.documentElement.style.setProperty("--gold-2", hex);
  document.documentElement.style.setProperty("--accent", hex);
}

function setTweak(key, value) {
  window.TWEAKS[key] = value;
  if (key === "accent") applyAccent(value);
  subscribers.forEach((fn) => fn(window.TWEAKS));
  window.parent.postMessage({ type: "__edit_mode_set_keys", edits: { [key]: value } }, "*");
}

// ---------- Panel UI ----------
function makePanel() {
  const wrap = document.createElement("div");
  wrap.className = "tweaks-panel";
  wrap.style.display = "none";
  wrap.innerHTML = `
    <div class="tw-head">
      <span class="tw-title">Tweaks</span>
      <button class="tw-close" type="button" aria-label="close">&times;</button>
    </div>
    <div class="tw-body">
      <div class="tw-section">Party rail</div>
      ${toggleRow("showPartyClass", "Show class line")}
      ${toggleRow("showPartyLevelBadge", "Show level badge")}
      ${toggleRow("showDirtyPip", "Show unsaved-changes pip")}
      ${toggleRow("compactPartyRail", "Compact rail")}

      <div class="tw-section">Item editor</div>
      ${toggleRow("showItemResref", "Show internal resref")}

      <div class="tw-section">Theme</div>
      <div class="tw-row">
        <span class="tw-lab">Accent</span>
        <div class="tw-swatches" id="tw-accent"></div>
      </div>
    </div>
  `;
  document.body.appendChild(wrap);

  // Swatches
  const swWrap = wrap.querySelector("#tw-accent");
  ACCENT_OPTIONS.forEach((opt) => {
    const b = document.createElement("button");
    b.type = "button";
    b.className = "tw-swatch";
    b.title = opt.label;
    b.style.background = opt.value;
    b.dataset.value = opt.value;
    if (window.TWEAKS.accent === opt.value) b.classList.add("is-active");
    b.addEventListener("click", () => {
      setTweak("accent", opt.value);
      [...swWrap.children].forEach((c) => c.classList.toggle("is-active", c.dataset.value === opt.value));
    });
    swWrap.appendChild(b);
  });

  // Toggles
  wrap.querySelectorAll("input[type=checkbox][data-key]").forEach((inp) => {
    inp.checked = !!window.TWEAKS[inp.dataset.key];
    inp.addEventListener("change", () => setTweak(inp.dataset.key, inp.checked));
  });

  wrap.querySelector(".tw-close").addEventListener("click", () => {
    wrap.style.display = "none";
    window.parent.postMessage({ type: "__edit_mode_dismissed" }, "*");
  });

  return wrap;
}

function toggleRow(key, label) {
  return `
    <label class="tw-row tw-toggle">
      <span class="tw-lab">${label}</span>
      <input type="checkbox" data-key="${key}" />
    </label>
  `;
}

// ---------- Host protocol ----------
const panel = makePanel();
window.addEventListener("message", (e) => {
  if (!e.data || typeof e.data !== "object") return;
  if (e.data.type === "__activate_edit_mode") panel.style.display = "";
  if (e.data.type === "__deactivate_edit_mode") panel.style.display = "none";
});
window.parent.postMessage({ type: "__edit_mode_available" }, "*");

// Apply initial accent
applyAccent(window.TWEAKS.accent);
