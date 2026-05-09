// Abilities — the most important rethink. Today: 3 column grid, each a dropdown + collapsible groups.
WF.add("abilities", `
  ${WF.intro("Abilities &mdash; The Big Rethink", "Today: three cramped columns (Skills / Talents / Spells), each with a dropdown to add a core tree, then collapsible accordion groups of dropdowns. Hard to scan, no sense of progression, no visual relationship between abilities. Four directions below from least to most ambitious.")}

  <div class="variants cols-2">

    <!-- VARIANT A: Tabbed kinds + filterable list -->
    <div class="variant">
      <div class="variant-head">
        <span class="variant-tag">A</span>
        <span class="variant-title">Tabbed kinds + filterable browser</span>
        <span class="variant-note">simplest fix &mdash; one kind at a time, full width</span>
      </div>
      <div class="variant-body">
        <div class="frame" style="min-height:520px; display:grid; grid-template-rows:auto auto 1fr; gap:0;">
          <div class="tbar between">
            <div class="toggle-bar">
              <button class="is-active">Skills <span class="mono muted" style="margin-left:0.3rem; font-size:0.7rem;">12</span></button>
              <button>Talents <span class="mono muted" style="margin-left:0.3rem; font-size:0.7rem;">8</span></button>
              <button>Spells <span class="mono muted" style="margin-left:0.3rem; font-size:0.7rem;">23</span></button>
            </div>
            <div class="row">
              <span class="chip">Skill points: <strong class="mono" style="color:var(--gold-2);">1</strong></span>
            </div>
          </div>
          <div class="filter-row">
            <input class="search" placeholder="Search abilities&hellip;" />
            <button class="btn ghost sm">All trees</button>
            <button class="btn ghost sm">Selected only</button>
            <button class="btn ghost sm">Available</button>
          </div>
          <div style="display:grid; grid-template-columns:200px 1fr; min-height:0;">
            <div class="col" style="border-right:1px solid var(--line); padding:0.4rem;">
              ${["Combat Training","Coercion","Stealing","Trap-Making","Survival","Herbalism","Poison-Making","Runecrafting"].map((t,i)=>`
                <div class="side-link ${i===0?"is-active":""}" style="padding:0.4rem 0.5rem;">
                  <span class="ico" style="background:${i===0?"var(--gold)":"var(--ink-4)"};"></span>
                  <span style="font-family:var(--font-body); font-size:0.82rem; letter-spacing:0;">${t}</span>
                </div>
              `).join("")}
            </div>
            <div style="padding:0.6rem; display:grid; gap:0.4rem; align-content:start;">
              <div class="h-eyebrow">Combat Training &middot; ranks</div>
              ${[["Combat Training",true,true],["Improved Combat Training",true,false],["Expert Combat Training",false,false],["Master Combat Training",false,false]].map(([n,owned,locked])=>`
                <div class="row between card" style="padding:0.5rem 0.6rem;">
                  <div class="row" style="gap:0.5rem;">
                    <div class="placeholder" style="width:32px; height:32px; min-height:auto;"></div>
                    <div class="col" style="gap:0.1rem;">
                      <strong>${n}</strong>
                      <span class="mono muted" style="font-size:0.7rem;">+1 weapon damage class</span>
                    </div>
                  </div>
                  ${owned ? `<button class="btn sm ghost" ${locked?"disabled style='opacity:0.5'":""}>${locked?"Locked":"Remove"}</button>` : `<button class="btn sm gold">Add</button>`}
                </div>
              `).join("")}
            </div>
          </div>
        </div>
      </div>
      <div class="variant-caption">Pick the kind first (one mode at a time), then a tree, then see ranks as a real list. No more 3-up cramp. The "locked because required by another" state becomes obvious instead of a footnote.</div>
    </div>

    <!-- VARIANT B: Tree visualization -->
    <div class="variant">
      <div class="variant-head">
        <span class="variant-tag">B</span>
        <span class="variant-title">Visual tree &mdash; in-game style</span>
        <span class="variant-note">most personality &mdash; mirrors the in-game UI</span>
      </div>
      <div class="variant-body">
        <div class="frame" style="min-height:520px; padding:0.6rem; display:grid; gap:0.5rem;">
          <div class="row between">
            <div class="toggle-bar">
              <button class="is-active">Spells</button>
              <button>Skills</button>
              <button>Talents</button>
            </div>
            <div class="row">
              <span class="chip rune">5 unlocked</span>
              <span class="chip">3 points to spend</span>
            </div>
          </div>

          <div class="row" style="gap:0.4rem; flex-wrap:wrap;">
            ${["Primal","Creation","Spirit","Entropy","Arcane","Blood Mage","Spirit Healer","Shapeshifter"].map((t,i)=>`
              <span class="tree-pill ${i===0?"is-active":""}"><span class="icon-sq"></span>${t}</span>
            `).join("")}
          </div>

          <div class="tree-canvas" style="min-height:300px;">
            <svg viewBox="0 0 600 300" style="position:absolute; inset:0; width:100%; height:100%; pointer-events:none;">
              <g stroke="#594a37" stroke-width="1.5" fill="none">
                <path d="M300,40 L160,110" />
                <path d="M300,40 L300,110" />
                <path d="M300,40 L440,110" />
                <path d="M160,110 L100,200" />
                <path d="M160,110 L210,200" />
                <path d="M300,110 L300,200" />
                <path d="M440,110 L390,200" />
                <path d="M440,110 L500,200" />
              </g>
              <g stroke="#c79a48" stroke-width="2" fill="none">
                <path d="M300,40 L300,110" />
                <path d="M300,110 L300,200" />
              </g>
            </svg>
            <div style="position:relative; height:280px;">
              <div class="tree-node unlocked" style="position:absolute; left:50%; top:18px; transform:translateX(-50%);">FB</div>
              <div class="tree-node" style="position:absolute; left:25%; top:88px; transform:translateX(-50%);">FL</div>
              <div class="tree-node unlocked" style="position:absolute; left:50%; top:88px; transform:translateX(-50%);">FZ</div>
              <div class="tree-node locked" style="position:absolute; left:75%; top:88px; transform:translateX(-50%);">FS</div>
              <div class="tree-node" style="position:absolute; left:13%; top:178px; transform:translateX(-50%);">CL</div>
              <div class="tree-node" style="position:absolute; left:33%; top:178px; transform:translateX(-50%);">FW</div>
              <div class="tree-node required" style="position:absolute; left:50%; top:178px; transform:translateX(-50%);">IN</div>
              <div class="tree-node locked" style="position:absolute; left:67%; top:178px; transform:translateX(-50%);">SF</div>
              <div class="tree-node locked" style="position:absolute; left:87%; top:178px; transform:translateX(-50%);">FN</div>
            </div>
          </div>

          <div class="card row between" style="padding:0.55rem 0.7rem;">
            <div class="row" style="gap:0.5rem;">
              <div class="placeholder" style="width:34px; height:34px; min-height:auto;"></div>
              <div class="col" style="gap:0.1rem;">
                <strong>Inferno</strong>
                <span class="mono muted" style="font-size:0.7rem;">Required by Storm of the Century</span>
              </div>
            </div>
            <div class="row">
              <button class="btn ghost sm">Hover for details</button>
              <button class="btn sm" disabled style="opacity:0.5;">Cannot remove (locked)</button>
            </div>
          </div>
        </div>
      </div>
      <div class="variant-caption">Click a node to add/remove. Gold lines = currently selected path; red rings = required by another. Probably most fun for casual users. Costlier to build &mdash; needs hard-coded tree positions per spec.</div>
    </div>

    <!-- VARIANT C: Loadout list with quick-add picker -->
    <div class="variant">
      <div class="variant-head">
        <span class="variant-tag">C</span>
        <span class="variant-title">Loadout list &mdash; what you have</span>
        <span class="variant-note">selected-first, browser-second</span>
      </div>
      <div class="variant-body">
        <div class="frame" style="min-height:520px; padding:0.6rem; display:grid; gap:0.55rem; align-content:start;">
          <div class="row between">
            <div class="h-display" style="font-size:1.05rem;">Solona's loadout</div>
            <div class="row">
              <button class="btn ghost sm">+ Add ability&hellip;</button>
            </div>
          </div>

          <div class="card" style="padding:0;">
            <div class="row" style="padding:0.4rem 0.6rem; border-bottom:1px solid var(--line); background:rgba(255,255,255,0.015);">
              <span class="h-eyebrow grow">Skills (3)</span>
              <span class="mono muted" style="font-size:0.7rem;">1 point unspent</span>
            </div>
            ${[["Combat Training",true],["Improved Combat Training",false],["Coercion",false]].map(([n,locked])=>`
              <div class="row between" style="padding:0.45rem 0.6rem; border-bottom:1px dashed var(--line);">
                <div class="row" style="gap:0.5rem;"><div class="placeholder" style="width:24px; height:24px; min-height:auto;"></div><span>${n}</span></div>
                <div class="row">${locked?'<span class="chip blood">required</span>':""}<button class="btn ghost sm">Remove</button></div>
              </div>
            `).join("")}
          </div>

          <div class="card" style="padding:0;">
            <div class="row" style="padding:0.4rem 0.6rem; border-bottom:1px solid var(--line);"><span class="h-eyebrow grow">Spells (5)</span><span class="mono muted" style="font-size:0.7rem;">3 points unspent</span></div>
            ${["Fireball","Flame Blast","Inferno","Heal","Glyph of Paralysis"].map((n,i)=>`
              <div class="row between" style="padding:0.45rem 0.6rem; border-bottom:1px dashed var(--line);">
                <div class="row" style="gap:0.5rem;"><div class="placeholder" style="width:24px; height:24px; min-height:auto;"></div><span>${n}</span><span class="chip" style="font-size:0.65rem;">Primal</span></div>
                <div class="row"><button class="btn ghost sm">Remove</button></div>
              </div>
            `).join("")}
          </div>

          <div class="card" style="padding:0;">
            <div class="row" style="padding:0.4rem 0.6rem; border-bottom:1px solid var(--line);"><span class="h-eyebrow grow">Talents (0)</span></div>
            <div style="padding:0.6rem;" class="muted hand">Nothing yet &mdash; click "+ Add ability" up top.</div>
          </div>
        </div>
      </div>
      <div class="variant-caption">Inverted: starts with what the character HAS. Add via a popover/modal triggered by + Add ability. Compact, scannable, removes the "every tree expanded vs collapsed" headache entirely.</div>
    </div>

    <!-- VARIANT D: Two-pane combo -->
    <div class="variant">
      <div class="variant-head">
        <span class="variant-tag">D</span>
        <span class="variant-title">Two-pane: catalog &harr; loadout</span>
        <span class="variant-note">hybrid &mdash; closest to today, more legible</span>
      </div>
      <div class="variant-body">
        <div class="frame" style="min-height:520px; display:grid; grid-template-columns:1fr 1fr;">
          <div style="border-right:1px solid var(--line); display:flex; flex-direction:column;">
            <div class="filter-row">
              <input class="search" placeholder="Catalog (Skills / Talents / Spells)&hellip;" />
            </div>
            <div style="padding:0.5rem; display:grid; gap:0.4rem; overflow:auto;">
              ${["Combat Training","Coercion","Stealing","Trap-Making","Survival","Herbalism","Poison-Making","Runecrafting"].map((t,i)=>`
                <div class="card" style="padding:0.4rem 0.55rem;">
                  <div class="row between" style="margin-bottom:0.2rem;">
                    <span style="font-weight:600;">${t}</span>
                    <span class="chip ${i<3?"gold":""}" style="font-size:0.65rem;">${i<3?"unlocked":"locked"}</span>
                  </div>
                  <div class="row" style="gap:0.3rem; flex-wrap:wrap;">
                    ${[1,2,3,4].map(r=>`<button class="btn sm ${r<=i?"":"ghost"}" style="padding:0.15rem 0.4rem; font-size:0.65rem;">${r<=i?'\u2713 ':''}Rank ${r}</button>`).join("")}
                  </div>
                </div>
              `).join("")}
            </div>
          </div>
          <div style="display:flex; flex-direction:column;">
            <div class="row" style="padding:0.5rem 0.6rem; border-bottom:1px solid var(--line); background:var(--bg-1);">
              <span class="h-eyebrow grow">Selected (8)</span>
              <span class="chip">1 skill pt unspent</span>
            </div>
            <div style="padding:0.5rem; display:grid; gap:0.3rem; overflow:auto;">
              ${["Combat Training I","Combat Training II","Coercion I","Fireball","Flame Blast","Heal","Inferno (locked)","Glyph of Paralysis"].map(n=>`
                <div class="row between card" style="padding:0.4rem 0.55rem; font-size:0.85rem;">
                  <span>${n}</span>
                  <button class="btn ghost sm">&times;</button>
                </div>
              `).join("")}
            </div>
          </div>
        </div>
      </div>
      <div class="variant-caption">Left = catalog with rank chips you click to toggle. Right = current loadout, removable inline. No accordion expand/collapse, no nested dropdowns.</div>
    </div>

  </div>
`);
