// Characters overview variants
WF.add("characters", `
  ${WF.intro("Characters &mdash; Overview", "Today: party list on the left, then a sub-tabbar (Overview / Abilities / Equipment), then a flat grid of inputs (Progress, Attributes, Point Pools). Three rethinks &mdash; reducing list-then-form repetition and giving each character a portrait identity.")}

  <div class="variants cols-3">

    <!-- VARIANT A: Compact party rail + dossier -->
    <div class="variant">
      <div class="variant-head">
        <span class="variant-tag">A</span>
        <span class="variant-title">Party Rail + Dossier</span>
      </div>
      <div class="variant-body">
        <div class="frame" style="display:grid; grid-template-columns: 76px 1fr; min-height:520px;">
          <div class="col" style="gap:0.4rem; padding:0.5rem; border-right:1px solid var(--line); background:var(--bg-1);">
            ${[
              { name:"Solona", lvl:14, active:true },
              { name:"Alistair", lvl:13 },
              { name:"Morrigan", lvl:13 },
              { name:"Leliana", lvl:12 },
              { name:"Sten", lvl:11 },
              { name:"Wynne", lvl:12 },
            ].map(p => `
              <div class="card ${p.active ? "hi" : ""}" style="padding:0.4rem; text-align:center; ${p.active ? "border-color: var(--gold);" : ""}">
                <div class="placeholder" style="aspect-ratio:1; min-height:auto; height:54px; padding:0.2rem;">${p.name}</div>
                <div class="mono" style="font-size:0.65rem; color:${p.active?"var(--gold-2)":"var(--ink-3)"}; margin-top:0.2rem;">L${p.lvl}</div>
              </div>
            `).join("")}
          </div>
          <div style="padding:0.85rem; display:grid; gap:0.65rem; align-content:start;">
            <div class="row between">
              <div class="col" style="gap:0.1rem;">
                <div class="h-display" style="font-size:1.3rem;">Solona Amell</div>
                <div class="mono muted" style="font-size:0.7rem;">Mage &middot; Circle of Magi &middot; Warden</div>
              </div>
              <div class="toggle-bar">
                <button class="is-active">Overview</button>
                <button>Abilities</button>
                <button>Equipment</button>
              </div>
            </div>

            <div class="card">
              <div class="h-eyebrow">Progress</div>
              <div class="row" style="gap:0.75rem; margin-top:0.4rem; flex-wrap:wrap;">
                <div class="field" style="min-width:90px;"><span class="field-label">Level</span><div class="field-input">14</div></div>
                <div class="field" style="min-width:130px;"><span class="field-label">Experience</span><div class="field-input">152,400</div></div>
                <div class="field" style="min-width:90px;"><span class="field-label">Approval</span><div class="field-input dim">&mdash;</div></div>
              </div>
            </div>

            <div class="card">
              <div class="h-eyebrow">Attributes</div>
              <div style="display:grid; grid-template-columns:repeat(3, 1fr); gap:0.5rem; margin-top:0.4rem;">
                ${["Strength 22","Dexterity 18","Willpower 35","Magic 42","Cunning 24","Constitution 26"].map(s=>`
                  <div class="field"><span class="field-label">${s.split(" ")[0]}</span><div class="field-input mono">${s.split(" ")[1]}</div></div>
                `).join("")}
              </div>
            </div>

            <div class="card">
              <div class="h-eyebrow">Point Pools</div>
              <div style="display:grid; grid-template-columns:repeat(2, 1fr); gap:0.5rem; margin-top:0.4rem;">
                ${["Attribute 0","Skill 1","Talent 0","Specialization 1"].map(s=>`
                  <div class="field"><span class="field-label">${s.split(" ")[0]} Points</span><div class="field-input mono">${s.split(" ")[1]}</div></div>
                `).join("")}
              </div>
            </div>
          </div>
        </div>
      </div>
      <div class="variant-caption">Party becomes a thin portrait rail (always visible, no name-list scrolling). Sub-tabbar slides next to the character title, removing one level of vertical chrome.</div>
    </div>

    <!-- VARIANT B: Stat block hero -->
    <div class="variant">
      <div class="variant-head">
        <span class="variant-tag">B</span>
        <span class="variant-title">Stat-block Hero</span>
      </div>
      <div class="variant-body">
        <div class="frame" style="min-height:520px; padding:0.85rem; display:grid; gap:0.7rem; align-content:start;">
          <div class="row" style="gap:0.75rem;">
            <div class="placeholder" style="width:120px; height:120px; min-height:auto;">PORTRAIT</div>
            <div class="col" style="gap:0.3rem; flex:1;">
              <div class="row between">
                <div class="h-display" style="font-size:1.5rem;">Solona Amell</div>
                <div class="row">
                  <button class="btn sm">Prev</button>
                  <button class="btn sm">Next</button>
                </div>
              </div>
              <div class="mono muted" style="font-size:0.75rem;">Mage &middot; Warden &middot; Hero of Ferelden</div>
              <div class="row" style="gap:0.4rem; margin-top:0.3rem;">
                <span class="chip gold">Level 14</span>
                <span class="chip">152,400 XP</span>
                <span class="chip rune">+0 unspent skill pts</span>
              </div>
              <div class="toggle-bar" style="align-self:flex-start; margin-top:0.4rem;">
                <button class="is-active">Overview</button>
                <button>Abilities</button>
                <button>Equipment</button>
              </div>
            </div>
          </div>

          <div style="display:grid; grid-template-columns:repeat(6, 1fr); gap:0.4rem;">
            ${[["STR",22],["DEX",18],["WIL",35],["MAG",42],["CUN",24],["CON",26]].map(([k,v])=>`
              <div class="card" style="text-align:center; padding:0.55rem;">
                <div class="h-eyebrow">${k}</div>
                <div class="h-display" style="font-size:1.6rem; color:var(--ink);">${v}</div>
              </div>
            `).join("")}
          </div>

          <div class="card">
            <div class="row between" style="margin-bottom:0.4rem;">
              <div class="h-eyebrow">Point Pools</div>
              <div class="hand muted" style="font-size:0.95rem;">click a number to edit</div>
            </div>
            <div style="display:grid; grid-template-columns:repeat(4, 1fr); gap:0.5rem;">
              ${[["Attribute",0],["Skill",1],["Talent",0],["Specialization",1]].map(([k,v])=>`
                <div class="row between" style="padding:0.4rem 0.55rem; border:1px solid var(--line); border-radius:var(--r-sm);">
                  <span class="mono" style="font-size:0.78rem;">${k}</span>
                  <span class="h-display" style="color:${v>0?"var(--gold-2)":"var(--ink-3)"};">${v}</span>
                </div>
              `).join("")}
            </div>
          </div>
        </div>
      </div>
      <div class="variant-caption">Stats become a 6-up dashboard, not a form. Cleaner at a glance, friendlier to casual editors. Inline-editable on click.</div>
    </div>

    <!-- VARIANT C: Single character per page, party as horizontal strip -->
    <div class="variant">
      <div class="variant-head">
        <span class="variant-tag">C</span>
        <span class="variant-title">Pager + Party Strip</span>
      </div>
      <div class="variant-body">
        <div class="frame" style="min-height:520px; padding:0.85rem; display:grid; gap:0.7rem; align-content:start;">
          <div class="row" style="gap:0.4rem; padding:0.4rem; background:var(--bg-1); border:1px solid var(--line); border-radius:var(--r);">
            ${[
              { name:"Solona", active:true },
              { name:"Alistair" },
              { name:"Morrigan" },
              { name:"Leliana" },
              { name:"Sten" },
              { name:"Wynne" },
              { name:"Zevran" },
              { name:"Dog" },
            ].map(p=>`
              <div class="${p.active ? "card hi":"card"}" style="flex:1; padding:0.35rem 0.4rem; text-align:center; border-color:${p.active?"var(--gold)":"var(--line)"};">
                <div class="mono" style="font-size:0.7rem; color:${p.active?"var(--gold-2)":"var(--ink-2)"};">${p.name}</div>
              </div>
            `).join("")}
          </div>

          <div class="row between">
            <div class="h-display" style="font-size:1.3rem;">Solona &mdash; Overview</div>
            <div class="row">
              <span class="chip">Lv 14</span>
              <span class="chip">Mage</span>
            </div>
          </div>

          <div style="display:grid; grid-template-columns: 1fr 1fr; gap:0.7rem;">
            <div class="card">
              <div class="h-eyebrow">Attributes</div>
              <div class="col" style="margin-top:0.4rem; gap:0.35rem;">
                ${[["Strength",22,40],["Dexterity",18,40],["Willpower",35,50],["Magic",42,50],["Cunning",24,40],["Constitution",26,40]].map(([k,v,max])=>`
                  <div class="row between" style="font-size:0.85rem;">
                    <span>${k}</span>
                    <div class="row" style="gap:0.5rem;">
                      <div style="width:80px; height:6px; background:var(--bg-3); border-radius:3px; overflow:hidden;">
                        <div style="width:${(v/max)*100}%; height:100%; background:linear-gradient(90deg, var(--gold), var(--gold-2));"></div>
                      </div>
                      <span class="mono" style="width:24px; text-align:right;">${v}</span>
                    </div>
                  </div>
                `).join("")}
              </div>
            </div>
            <div class="col" style="gap:0.7rem;">
              <div class="card">
                <div class="h-eyebrow">Progress</div>
                <div class="row" style="gap:0.5rem; margin-top:0.4rem;">
                  <div class="field grow"><span class="field-label">Level</span><div class="field-input mono">14</div></div>
                  <div class="field grow"><span class="field-label">XP</span><div class="field-input mono">152,400</div></div>
                </div>
              </div>
              <div class="card">
                <div class="h-eyebrow">Point Pools</div>
                <div style="display:grid; grid-template-columns:1fr 1fr; gap:0.5rem; margin-top:0.4rem;">
                  ${[["Attribute",0],["Skill",1],["Talent",0],["Spec.",1]].map(([k,v])=>`
                    <div class="field"><span class="field-label">${k}</span><div class="field-input mono">${v}</div></div>
                  `).join("")}
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
      <div class="variant-caption">Party as a horizontal pill strip frees the left edge entirely. Attributes shown with comparison bars instead of a 6-cell input grid &mdash; better intuition for casual editors.</div>
    </div>

  </div>
`);
