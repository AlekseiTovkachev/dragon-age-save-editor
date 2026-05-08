// helpers shared by tab scripts
window.WF = {
  add(id, html) {
    const wrap = document.createElement("div");
    wrap.className = "tab-content";
    wrap.dataset.id = id;
    wrap.innerHTML = html;
    document.getElementById("body").appendChild(wrap);
    return wrap;
  },
  intro(title, body) {
    return `<div class="section-intro"><h2>${title}</h2><p>${body}</p></div>`;
  },
  variant({ tag, title, note = "", caption = "", cols = 12, body }) {
    return `
      <div class="variant" style="grid-column: span ${cols} / span ${cols};">
        <div class="variant-head">
          <span class="variant-tag">${tag}</span>
          <span class="variant-title">${title}</span>
          ${note ? `<span class="variant-note">${note}</span>` : ""}
        </div>
        <div class="variant-body">${body}</div>
        ${caption ? `<div class="variant-caption">${caption}</div>` : ""}
      </div>`;
  },
};
