// Hi-fi prototype interactivity

const STATE = {
  charKey: PARTY[0].key,
  charTab: "overview",
  abilityKind: "skills",
  abilityTreeId: ABILITY_TREES.skills[0].id,
  abilitySearch: "",
  invSearch: "",
  invCat: "All",
  invSort: "name",
  invDir: 1,
  invExpanded: null,
  equipSearch: "",
  equipSort: "name",
  equipDir: 1,
  equipExpanded: null,
  pfEra: "All",
  pfSearch: "",
  // editable copies
  party: structuredClone(PARTY),
  trees: structuredClone(ABILITY_TREES),
  items: structuredClone(ITEMS),
  decisions: structuredClone(PLOT_DECISIONS),
  partyGold: 42314,
};

// Mock per-character equipped items (the save format has no slot info,
// so each character just owns a flat list).
STATE.equipped = {};
PARTY.forEach((p, i) => {
  const start = (i * 3) % ITEMS.length;
  STATE.equipped[p.key] = Array.from({ length: 5 }, (_, k) =>
    structuredClone(ITEMS[(start + k) % ITEMS.length])
  );
});

const $ = (s, r=document) => r.querySelector(s);
const $$ = (s, r=document) => [...r.querySelectorAll(s)];

// ---------- Sidebar nav ----------
$$(".nav-item[data-section]").forEach(b => {
  b.addEventListener("click", () => switchSection(b.dataset.section));
});

function switchSection(id) {
  $$(".nav-item[data-section]").forEach(b => b.classList.toggle("is-active", b.dataset.section === id));
  $$("section.section").forEach(s => s.style.display = s.dataset.section === id ? "" : "none");
  window.scrollTo({ top: 0 });
}

// ---------- Party rail ----------
function renderPartyRail() {
  const root = $("#party-rail");
  const t = window.TWEAKS || {};
  root.classList.toggle("compact", !!t.compactPartyRail);
  root.innerHTML = `<div class="rail-label">Party (${STATE.party.length})</div>` + STATE.party.map(p => `
    <button class="party-card ${p.key===STATE.charKey?"is-active":""} ${p.dirty?"dirty":""}" data-key="${p.key}">
      ${t.showPartyLevelBadge !== false ? `<span class="badge-lvl">${p.lvl}</span>` : ""}
      <span style="display:flex; flex-direction:column; min-width:0;">
        <span class="nm" style="overflow:hidden; text-overflow:ellipsis; white-space:nowrap;">${p.name}</span>
        ${t.showPartyClass ? `<span class="ro">${p.role}</span>` : ""}
      </span>
      ${t.showDirtyPip !== false ? `<span class="pip" title="${p.dirty?"unsaved changes":""}"></span>` : ""}
    </button>
  `).join("");
  $$(".party-card", root).forEach(b => b.addEventListener("click", () => {
    STATE.charKey = b.dataset.key;
    renderCharacter();
    renderPartyRail();
  }));
}

// ---------- Character detail ----------
$$("#char-subtabs button").forEach(b => b.addEventListener("click", () => {
  STATE.charTab = b.dataset.tab;
  $$("#char-subtabs button").forEach(x => x.classList.toggle("is-active", x === b));
  $$(".char-tab").forEach(t => t.style.display = t.dataset.tab === STATE.charTab ? "" : "none");
}));

function currentChar() { return STATE.party.find(p => p.key === STATE.charKey); }

function renderCharacter() {
  const c = currentChar();
  $("#char-name").textContent = c.name;
  $("#char-class").textContent = c.role;
  $("#char-lvl-chip").textContent = `Level ${c.lvl}`;
  $("#char-xp-chip").textContent = `${c.xp.toLocaleString()} XP`;
  const pts = Object.entries(c.pools).filter(([,v]) => v > 0).map(([k,v]) => `+${v} ${k.toLowerCase()}`).join(" \u00b7 ") || "no points unspent";
  $("#char-pts-chip").textContent = pts;

  // Progress
  $("#progress-grid").innerHTML = `
    ${numField("Level", c.lvl, false, v => c.lvl = +v||0)}
    ${numField("Experience", c.xp, false, v => c.xp = +v||0)}
    ${c.key==="main" ? `<div></div>` : numField("Approval", c.approval ?? 0, false, v => c.approval = +v||0)}
  `;
  bindNumFields("#progress-grid");

  // Attributes
  $("#attrs-grid").innerHTML = Object.entries(c.stats).map(([k,v]) =>
    numField(k, v, false, nv => c.stats[k] = +nv||0)
  ).join("");
  bindNumFields("#attrs-grid");

  // Pools
  $("#pools-grid").innerHTML = Object.entries(c.pools).map(([k,v]) =>
    numField(`${k} pts`, v, false, nv => c.pools[k] = +nv||0)
  ).join("");
  bindNumFields("#pools-grid");

  // Equipment list rendered separately
  renderEquipment();

  renderAbilities();
}

function numField(label, value, dirty, onChange) {
  return `
    <label class="num">
      <span class="lab">${label}</span>
      <input class="inp ${dirty?"dirty":""}" type="number" value="${value}" data-h="1" />
    </label>
  `;
}

function bindNumFields(scope) {
  $$(`${scope} .inp`).forEach((inp, i) => {
    const original = inp.value;
    inp.addEventListener("input", () => {
      inp.classList.toggle("dirty", inp.value !== original);
      const c = currentChar();
      if (!c.dirty && inp.value !== original) {
        c.dirty = true;
        renderPartyRail();
      }
    });
  });
}

// ---------- Abilities ----------
$$("#kind-tabs button").forEach(b => b.addEventListener("click", () => {
  STATE.abilityKind = b.dataset.kind;
  STATE.abilityTreeId = STATE.trees[STATE.abilityKind][0].id;
  $$("#kind-tabs button").forEach(x => x.classList.toggle("is-active", x === b));
  const labels = { skills:"Skill pts", talents:"Talent pts", spells:"Spell pts" };
  $("#kind-points").innerHTML = `${labels[STATE.abilityKind]}: <strong class="mono" style="color:var(--gold-2); margin-left:0.2rem;">${STATE.abilityKind==="skills"?1:0}</strong>`;
  renderAbilities();
}));

$("#ability-search").addEventListener("input", e => {
  STATE.abilitySearch = e.target.value.toLowerCase();
  renderAbilities();
});

function renderAbilities() {
  const trees = STATE.trees[STATE.abilityKind];
  const filtered = STATE.abilitySearch
    ? trees.filter(t => t.name.toLowerCase().includes(STATE.abilitySearch) ||
                         t.ranks.some(r => r.name.toLowerCase().includes(STATE.abilitySearch)))
    : trees;

  const list = $("#tree-list");
  list.innerHTML = filtered.map(t => {
    const owned = t.ranks.filter(r => r.owned).length;
    const total = t.ranks.length;
    const isActive = t.id === STATE.abilityTreeId;
    return `
      <button class="tree-row ${isActive?"is-active":""}" data-tree="${t.id}">
        <span>${t.name}</span>
        <span class="rank-pip ${owned===total?"full":""}">${owned}/${total}</span>
      </button>
    `;
  }).join("");
  $$(".tree-row", list).forEach(b => b.addEventListener("click", () => {
    STATE.abilityTreeId = b.dataset.tree;
    renderAbilities();
  }));

  const tree = trees.find(t => t.id === STATE.abilityTreeId) || trees[0];
  $("#ranks-panel").innerHTML = `
    <div class="card-head" style="margin-bottom:0.4rem;">
      <span class="card-title">${tree.name} &middot; ranks</span>
      <span class="helptext">Add or remove ranks. Locked ranks are required by another ability.</span>
    </div>
    ${tree.ranks.map((r, i) => `
      <div class="rank-row ${r.owned?"owned":""} ${r.locked?"locked":""}">
        <div class="rank-num">${i+1}</div>
        <div>
          <div class="rank-name">${r.name}</div>
          <div class="rank-desc">${r.desc}</div>
        </div>
        ${r.owned
          ? `<button class="btn-rm" data-tree="${tree.id}" data-rank="${i}" ${r.locked?"disabled":""}>${r.locked?"Required":"Remove"}</button>`
          : `<button class="btn-add" data-tree="${tree.id}" data-rank="${i}">+ Add</button>`}
      </div>
    `).join("")}
  `;
  $$("#ranks-panel button").forEach(b => b.addEventListener("click", () => {
    const t = STATE.trees[STATE.abilityKind].find(x => x.id === b.dataset.tree);
    const r = t.ranks[+b.dataset.rank];
    r.owned = !r.owned;
    const c = currentChar();
    c.dirty = true;
    renderPartyRail();
    renderAbilities();
  }));
}

// ---------- Inventory ----------
function catFilterMatch(it, cat) {
  if (cat === "All") return true;
  if (cat === "Weapons") return ["Longsword","Greatsword","Bow","Staff","Dagger","Axe","Mace"].includes(it.cat);
  if (cat === "Armor") return ["Light armor","Medium armor","Heavy armor","Shield","Helm","Boots","Gloves"].includes(it.cat);
  return it.cat === cat;
}

function renderInvFilters() {
  $("#cat-filters").innerHTML = ITEM_CATEGORIES.map(c => `
    <button class="cat-chip ${c===STATE.invCat?"is-active":""}" data-cat="${c}">${c}</button>
  `).join("");
  $$("#cat-filters .cat-chip").forEach(b => b.addEventListener("click", () => {
    STATE.invCat = b.dataset.cat;
    renderInventory();
  }));
}

$("#inv-search").addEventListener("input", e => { STATE.invSearch = e.target.value.toLowerCase(); renderInventory(); });

$$(".inv-thead .th[data-sort]").forEach(th => th.addEventListener("click", () => {
  if (STATE.invSort === th.dataset.sort) STATE.invDir *= -1;
  else { STATE.invSort = th.dataset.sort; STATE.invDir = 1; }
  $$(".inv-thead .th").forEach(t => t.classList.remove("sorted"));
  th.classList.add("sorted");
  renderInventory();
}));

function renderInventory() {
  let rows = STATE.items.filter(it =>
    catFilterMatch(it, STATE.invCat) &&
    (STATE.invSearch === "" ||
     it.name.toLowerCase().includes(STATE.invSearch) ||
     it.cat.toLowerCase().includes(STATE.invSearch) ||
     it.resref.toLowerCase().includes(STATE.invSearch))
  );
  const key = STATE.invSort;
  rows.sort((a,b) => {
    let av = a[key], bv = b[key];
    if (typeof av === "string") return STATE.invDir * av.localeCompare(bv);
    return STATE.invDir * (av - bv);
  });
  $("#inv-count").textContent = `${rows.length} of ${STATE.items.length} items`;

  $("#inv-body").innerHTML = rows.map((it, idx) => {
    const isExpanded = STATE.invExpanded === it.resref;
    return `
      <button class="inv-row ${isExpanded?"is-active":""}" data-resref="${it.resref}">
        <span class="name">${it.name}</span>
        <span class="cat">${it.cat}</span>
        <span class="num-cell">T${it.tier}</span>
        <span class="qty-cell">${it.stack>1?"x"+it.stack:""}</span>
        <span class="num-cell">${it.cost.toLocaleString()}g</span>
        <span class="chev">&rsaquo;</span>
      </button>
      ${isExpanded ? renderItemEditor(it) : ""}
    `;
  }).join("");
  $$(".inv-row").forEach(r => r.addEventListener("click", e => {
    if (e.target.closest(".inv-expand")) return;
    STATE.invExpanded = STATE.invExpanded === r.dataset.resref ? null : r.dataset.resref;
    renderInventory();
  }));
  $$(".inv-expand").forEach(el => el.addEventListener("click", e => e.stopPropagation()));
  $$(".inv-expand .prop-chip button").forEach(b => b.addEventListener("click", e => {
    e.stopPropagation();
    const it = STATE.items.find(i => i.resref === b.dataset.resref);
    it.props.splice(+b.dataset.idx, 1);
    renderInventory();
  }));
  $$(".inv-expand .prop-add-btn").forEach(b => b.addEventListener("click", e => {
    e.stopPropagation();
    const it = STATE.items.find(i => i.resref === b.dataset.resref);
    it.props.push({ n:"New property", p:"+1" });
    renderInventory();
  }));
  $$(".inv-expand .clone-btn").forEach(b => b.addEventListener("click", e => {
    e.stopPropagation();
    const idx = STATE.items.findIndex(i => i.resref === b.dataset.resref);
    const copy = structuredClone(STATE.items[idx]);
    copy.resref = copy.resref + "_copy";
    STATE.items.splice(idx+1, 0, copy);
    renderInventory();
  }));
  $$(".inv-expand .remove-btn").forEach(b => b.addEventListener("click", e => {
    e.stopPropagation();
    const idx = STATE.items.findIndex(i => i.resref === b.dataset.resref);
    STATE.items.splice(idx, 1);
    STATE.invExpanded = null;
    renderInventory();
  }));
}

function renderItemEditor(it, opts) {
  const allowRemove = !opts || opts.allowRemove !== false;
  const allowClone = !opts || opts.allowClone !== false;
  return `
    <div class="inv-expand">
      <div class="expand-head">
        <div>
          <div style="font-family: var(--font-display); font-size: 1.05rem; color: var(--gold-2); letter-spacing: 0.04em;">${it.name}</div>
          <div class="expand-meta">
            ${(window.TWEAKS && window.TWEAKS.showItemResref !== false) ? `<span class="mono" style="color:var(--ink-3); font-size:0.7rem;">${it.resref}</span><span style="color:var(--ink-4);">&middot;</span>` : ""}
            <a href="#" onclick="event.preventDefault();">open wiki page &rarr;</a>
          </div>
        </div>
        <div class="row" style="gap:0.4rem;">
          ${allowClone ? `<button class="btn ghost sm clone-btn" data-resref="${it.resref}">Clone</button>` : ""}
          ${allowRemove ? `<button class="btn ghost sm remove-btn" data-resref="${it.resref}" style="color:#e89a92;">Remove</button>` : ""}
        </div>
      </div>
      <div class="inv-expand-fields">
        <label class="num"><span class="lab">Material</span><input class="inp" value="${it.mat}"/></label>
        <label class="num"><span class="lab">Item level</span><input class="inp" type="number" value="${it.lvl}"/></label>
        <label class="num"><span class="lab">Cost</span><input class="inp" type="number" value="${it.cost}"/></label>
        ${it.stack>1 ? `<label class="num"><span class="lab">Stack size</span><input class="inp" type="number" value="${it.stack}"/></label>` : `<div></div>`}
      </div>
      <div class="props-area">
        <div class="props-head">
          <span class="tag-line">Properties (${it.props.length})</span>
        </div>
        <div class="props-list">
          ${it.props.map((p, idx) => `
            <span class="prop-chip">${p.n} <span class="pwr">${p.p}</span><button data-resref="${it.resref}" data-idx="${idx}" title="remove">&times;</button></span>
          `).join("")}
          <button class="prop-add-btn" data-resref="${it.resref}">+ add property</button>
        </div>
      </div>
    </div>
  `;
}

$("#party-gold").addEventListener("input", e => { STATE.partyGold = +e.target.value || 0; });

// ---------- Plot Flags ----------
function renderPfFilters() {
  $("#pf-filters").innerHTML = PLOT_ERAS.map(c => `
    <button class="cat-chip ${c===STATE.pfEra?"is-active":""}" data-era="${c}">${c}</button>
  `).join("");
  $$("#pf-filters .cat-chip").forEach(b => b.addEventListener("click", () => {
    STATE.pfEra = b.dataset.era;
    renderPlotFlags();
  }));
}

$("#pf-search").addEventListener("input", e => { STATE.pfSearch = e.target.value.toLowerCase(); renderPlotFlags(); });

function renderPlotFlags() {
  const list = STATE.decisions.filter(d =>
    (STATE.pfEra === "All" || d.era === STATE.pfEra) &&
    (STATE.pfSearch === "" ||
     d.q.toLowerCase().includes(STATE.pfSearch) ||
     String(d.id).includes(STATE.pfSearch) ||
     d.opts.some(o => o.toLowerCase().includes(STATE.pfSearch)))
  );
  const modCount = STATE.decisions.filter(d => d.modified).length;
  $("#pf-modified").textContent = `${modCount} of ${STATE.decisions.length} modified`;

  $("#plot-grid").innerHTML = list.map(d => `
    <div class="plot-card">
      <div class="question">
        <div>
          <h3 class="question-text">${d.q}</h3>
          <div class="tag-line" style="margin-top:0.2rem;">${d.era}</div>
        </div>
        <span class="question-id">PLT_${d.id}</span>
      </div>
      <div class="plot-options">
        ${d.opts.map((o, i) => `
          <label class="plot-opt ${i===d.picked?"is-active":""}" data-id="${d.id}" data-i="${i}">
            <span class="marker"></span>
            <span class="opt-text">${o}</span>
          </label>
        `).join("")}
      </div>
      <div class="footer-row">
        <span>${d.opts.length} options</span>
        ${d.modified ? `<span class="modified-pip"><span class="dot"></span>modified</span>` : `<span style="color:var(--ink-4);">unchanged</span>`}
      </div>
    </div>
  `).join("");

  $$(".plot-opt").forEach(o => o.addEventListener("click", () => {
    const d = STATE.decisions.find(x => x.id === +o.dataset.id);
    const newPick = +o.dataset.i;
    if (d.picked !== newPick) {
      d.picked = newPick;
      d.modified = true;
      renderPlotFlags();
    }
  }));
}

// ---------- Equipment (per-character flat list) ----------
$("#equip-search").addEventListener("input", e => {
  STATE.equipSearch = e.target.value.toLowerCase();
  renderEquipment();
});

$$('.inv-thead .th[data-scope="equip"]').forEach(th => th.addEventListener("click", () => {
  if (STATE.equipSort === th.dataset.sort) STATE.equipDir *= -1;
  else { STATE.equipSort = th.dataset.sort; STATE.equipDir = 1; }
  $$('.inv-thead .th[data-scope="equip"]').forEach(t => t.classList.remove("sorted"));
  th.classList.add("sorted");
  renderEquipment();
}));

function renderEquipment() {
  const body = $("#equip-body");
  if (!body) return;
  const list = STATE.equipped[STATE.charKey] || [];
  let rows = list.filter(it =>
    STATE.equipSearch === "" ||
    it.name.toLowerCase().includes(STATE.equipSearch) ||
    it.cat.toLowerCase().includes(STATE.equipSearch) ||
    it.resref.toLowerCase().includes(STATE.equipSearch)
  );
  const key = STATE.equipSort;
  rows.sort((a, b) => {
    let av = a[key], bv = b[key];
    if (typeof av === "string") return STATE.equipDir * av.localeCompare(bv);
    return STATE.equipDir * (av - bv);
  });
  $("#equip-count").textContent = `${rows.length} of ${list.length} items`;

  body.innerHTML = rows.map(it => {
    const isExpanded = STATE.equipExpanded === it.resref;
    return `
      <button class="inv-row ${isExpanded?"is-active":""}" data-resref="${it.resref}">
        <span class="name">${it.name}</span>
        <span class="cat">${it.cat}</span>
        <span class="num-cell">T${it.tier}</span>
        <span class="qty-cell">${it.stack>1?"x"+it.stack:""}</span>
        <span class="num-cell">${it.cost.toLocaleString()}g</span>
        <span class="chev">&rsaquo;</span>
      </button>
      ${isExpanded ? renderItemEditor(it, { allowRemove: false, allowClone: false }) : ""}
    `;
  }).join("");

  $$("#equip-body .inv-row").forEach(r => r.addEventListener("click", e => {
    if (e.target.closest(".inv-expand")) return;
    STATE.equipExpanded = STATE.equipExpanded === r.dataset.resref ? null : r.dataset.resref;
    renderEquipment();
  }));
  $$("#equip-body .inv-expand").forEach(el => el.addEventListener("click", e => e.stopPropagation()));
  $$("#equip-body .inv-expand .prop-chip button").forEach(b => b.addEventListener("click", e => {
    e.stopPropagation();
    const it = (STATE.equipped[STATE.charKey] || []).find(i => i.resref === b.dataset.resref);
    if (!it) return;
    it.props.splice(+b.dataset.idx, 1);
    renderEquipment();
  }));
  $$("#equip-body .inv-expand .prop-add-btn").forEach(b => b.addEventListener("click", e => {
    e.stopPropagation();
    const it = (STATE.equipped[STATE.charKey] || []).find(i => i.resref === b.dataset.resref);
    if (!it) return;
    it.props.push({ n: "New property", p: "+1" });
    renderEquipment();
  }));
}

// ---------- Tweaks subscription ----------
if (typeof window.onTweaksChange === "function") {
  window.onTweaksChange(() => {
    renderPartyRail();
    renderInventory();
    renderEquipment();
  });
}

// ---------- Init ----------
renderPartyRail();
renderCharacter();
renderInvFilters();
renderInventory();
renderPfFilters();
renderPlotFlags();
