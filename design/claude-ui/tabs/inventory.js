// Inventory + Item editor variants
WF.add("inventory", `
  ${WF.intro("Inventory &amp; Item Editor", "Today: backpack list on left, scrollable detail on right with Overview fields + a Properties grid (Property / Power / Action). Three rethinks &mdash; reduce list-detail repetition, make item identity stronger, and make the property table feel less like a spreadsheet.")}

  <div class="variants cols-1">

    <!-- VARIANT A: Grid of icon tiles + slide-over editor -->
    <div class="variant">
      <div class="variant-head">
        <span class="variant-tag">A</span>
        <span class="variant-title">Icon grid + side-rail editor</span>
        <span class="variant-note">most game-like &mdash; feels like a real backpack</span>
      </div>
      <div class="variant-body">
        <div class="frame" style="display:grid; grid-template-columns:1fr 380px; min-height:520px;">
          <div style="display:flex; flex-direction:column; min-width:0;">
            <div class="filter-row">
              <input class="search" placeholder="Search backpack&hellip;" />
              <button class="btn ghost sm">All</button>
              <button class="btn ghost sm">Weapons</button>
              <button class="btn ghost sm">Armor</button>
              <button class="btn ghost sm">Consumable</button>
              <button class="btn ghost sm">Misc</button>
              <div class="spacer"></div>
              <span class="chip gold">Gold &middot; <strong class="mono">42,314</strong></span>
            </div>
            <div style="padding:0.6rem; overflow:auto; display:grid; grid-template-columns:repeat(auto-fill, minmax(72px, 1fr)); gap:0.4rem; align-content:start;">
              ${Array.from({length:36}).map((_,i)=>{
                const active = i===5;
                const stack = [3,7,12,18,28].includes(i) ? Math.floor(Math.random()*40)+2 : null;
                return `
                  <div class="card ${active?"hi":""}" style="padding:0.3rem; aspect-ratio:1; position:relative; border-color:${active?"var(--gold)":"var(--line)"};">
                    <div class="placeholder" style="aspect-ratio:1; min-height:auto; height:100%;"></div>
                    ${stack ? `<span class="mono" style="position:absolute; bottom:3px; right:5px; font-size:0.65rem; color:var(--gold-2); text-shadow:0 0 4px #000;">x${stack}</span>` : ""}
                  </div>
                `;
              }).join("")}
            </div>
          </div>
          <aside style="border-left:1px solid var(--line); background:var(--bg-1); padding:0.7rem; display:grid; gap:0.55rem; align-content:start; overflow:auto;">
            <div class="row" style="gap:0.55rem;">
              <div class="placeholder" style="width:64px; height:64px; min-height:auto;"></div>
              <div class="col" style="gap:0.15rem; flex:1;">
                <div class="h-display" style="font-size:1.05rem;">Starfang</div>
                <div class="mono muted" style="font-size:0.65rem;">gen_im_wep_swd_lng_blk</div>
                <div class="row" style="gap:0.3rem;">
                  <span class="chip gold">Longsword</span>
                  <span class="chip">Tier 7</span>
                </div>
              </div>
            </div>

            <div class="card">
              <div class="h-eyebrow">Overview</div>
              <div style="display:grid; grid-template-columns:1fr 1fr; gap:0.4rem; margin-top:0.4rem;">
                <div class="field"><span class="field-label">Material</span><div class="field-input">Veridium</div></div>
                <div class="field"><span class="field-label">Item level</span><div class="field-input mono">14</div></div>
              </div>
            </div>

            <div class="card" style="padding:0;">
              <div class="row" style="padding:0.4rem 0.6rem; border-bottom:1px solid var(--line);"><span class="h-eyebrow grow">Properties (3)</span><button class="btn sm gold">+ Add</button></div>
              ${[["Damage bonus","+5"],["Attack bonus","+3"],["Critical chance","+2.5%"]].map(([n,p])=>`
                <div class="row between" style="padding:0.45rem 0.6rem; border-bottom:1px dashed var(--line); gap:0.4rem;">
                  <span style="font-size:0.85rem;">${n}</span>
                  <span class="mono" style="color:var(--gold-2);">${p}</span>
                  <button class="btn ghost sm" style="font-size:0.65rem;">&times;</button>
                </div>
              `).join("")}
            </div>

            <div class="row" style="gap:0.4rem;">
              <a class="mono" href="#" style="color:var(--rune); font-size:0.78rem;">Open wiki page &rarr;</a>
            </div>

            <div class="row" style="gap:0.4rem;">
              <button class="btn ghost grow">Clone</button>
              <button class="btn ghost grow" style="color:#e89a92;">Remove</button>
            </div>
          </aside>
        </div>
      </div>
      <div class="variant-caption">Tile grid mirrors how players actually picture inventory. Stack counts on tile, slide-over editor stays put. Filter chips replace category column.</div>
    </div>

    <!-- VARIANT B: Sortable table -->
    <div class="variant">
      <div class="variant-head">
        <span class="variant-tag">B</span>
        <span class="variant-title">Sortable table + inline expand</span>
        <span class="variant-note">power-user friendly</span>
      </div>
      <div class="variant-body">
        <div class="frame" style="min-height:520px; display:grid; grid-template-rows:auto auto 1fr;">
          <div class="filter-row">
            <input class="search" placeholder="Filter items&hellip;" />
            <span class="chip">Sort: Category &darr;</span>
            <div class="spacer"></div>
            <span class="chip gold">Gold <strong class="mono">42,314</strong></span>
          </div>
          <div class="row" style="display:grid; grid-template-columns: 36px 2fr 1fr 90px 90px 80px; gap:0.4rem; padding:0.4rem 0.7rem; border-bottom:1px solid var(--line); background:rgba(255,255,255,0.015);">
            <span></span>
            <span class="h-eyebrow">Item</span>
            <span class="h-eyebrow">Category</span>
            <span class="h-eyebrow">Tier</span>
            <span class="h-eyebrow">Stack</span>
            <span class="h-eyebrow">Cost</span>
          </div>
          <div style="overflow:auto;">
            ${[
              ["Starfang","Longsword",7,1,"450g",true],
              ["Vigilance","Greatsword",6,1,"380g"],
              ["Andruil's Blessing","Light armor",5,1,"420g"],
              ["Cailan's Shield","Shield",5,1,"310g"],
              ["Lyrium potion","Consumable",2,12,"40g"],
              ["Health poultice","Consumable",1,32,"15g"],
              ["Lifestone","Misc",4,2,"75g"],
              ["Volcanic Aurum","Misc",6,1,"180g"],
            ].map(([n,c,t,s,co,active])=>`
              <div class="row" style="display:grid; grid-template-columns: 36px 2fr 1fr 90px 90px 80px; gap:0.4rem; padding:0.45rem 0.7rem; border-bottom:1px dashed var(--line); align-items:center; ${active?"background:rgba(199,154,72,0.06);":""}">
                <div class="placeholder" style="width:26px; height:26px; min-height:auto;"></div>
                <span style="${active?"color:var(--gold-2); font-weight:600;":""}">${n}</span>
                <span class="muted" style="font-size:0.85rem;">${c}</span>
                <span class="mono">T${t}</span>
                <span class="mono">${s>1?"x"+s:""}</span>
                <span class="mono">${co}</span>
              </div>
              ${active?`
                <div style="padding: 0.6rem 1rem 0.8rem 3rem; border-bottom:1px solid var(--line); background: rgba(199,154,72,0.03); display:grid; gap:0.5rem;">
                  <div class="row" style="gap:0.4rem; flex-wrap:wrap;">
                    <div class="field"><span class="field-label">Material</span><div class="field-input">Veridium</div></div>
                    <div class="field"><span class="field-label">Item level</span><div class="field-input mono">14</div></div>
                    <div class="field"><span class="field-label">Cost</span><div class="field-input mono">450</div></div>
                    <div class="spacer"></div>
                    <button class="btn ghost sm">Clone</button>
                    <button class="btn ghost sm">Remove</button>
                  </div>
                  <div class="row" style="gap:0.3rem; flex-wrap:wrap;">
                    <span class="chip gold">+5 damage</span>
                    <span class="chip gold">+3 attack</span>
                    <span class="chip gold">+2.5% crit</span>
                    <button class="btn sm ghost" style="border-style:dashed;">+ property</button>
                  </div>
                </div>
              `:""}
            `).join("")}
          </div>
        </div>
      </div>
      <div class="variant-caption">Tabular for fast scanning. Click row &rarr; inline expand with full editor underneath. Properties shown as gold chips; click any to edit power. No spreadsheet-feel for a casual user.</div>
    </div>

    <!-- VARIANT C: Today's pattern, polished -->
    <div class="variant">
      <div class="variant-head">
        <span class="variant-tag">C</span>
        <span class="variant-title">Today's pattern, polished</span>
        <span class="variant-note">conservative refinement</span>
      </div>
      <div class="variant-body">
        <div class="frame" style="display:grid; grid-template-columns:300px 1fr; min-height:520px;">
          <div style="border-right:1px solid var(--line); display:flex; flex-direction:column;">
            <div class="filter-row">
              <input class="search" placeholder="Search&hellip;" />
            </div>
            <div style="overflow:auto;">
              ${["Starfang","Vigilance","Andruil's Blessing","Cailan's Shield","Lyrium potion","Health poultice","Lifestone","Volcanic Aurum","Elfroot","Spider Ichor"].map((n,i)=>{
                const a = i===0;
                return `
                  <div class="itemrow ${a?"is-active":""}">
                    <div class="icn"></div>
                    <div class="col" style="gap:0.05rem;">
                      <span class="name">${n}</span>
                      <span class="meta">${["Longsword","Greatsword","Light armor","Shield","Consumable","Consumable","Misc","Misc","Misc","Misc"][i]} &middot; T${[7,6,5,5,2,1,4,6,1,2][i]}</span>
                    </div>
                    <div></div>
                    <span class="qty">${[1,1,1,1,12,32,2,1,5,3][i]>1?"x"+[1,1,1,1,12,32,2,1,5,3][i]:""}</span>
                  </div>
                `;
              }).join("")}
            </div>
          </div>
          <div style="padding:0.7rem; display:grid; gap:0.6rem; align-content:start; overflow:auto;">
            <div class="row" style="gap:0.7rem;">
              <div class="placeholder" style="width:64px; height:64px; min-height:auto;"></div>
              <div class="col" style="flex:1; gap:0.2rem;">
                <div class="h-display" style="font-size:1.15rem;">Starfang</div>
                <div class="mono muted" style="font-size:0.7rem;">gen_im_wep_swd_lng_blk &middot; <a href="#" style="color:var(--rune);">wiki &rarr;</a></div>
              </div>
              <div class="row">
                <button class="btn ghost sm">Clone</button>
                <button class="btn ghost sm" style="color:#e89a92;">Remove</button>
              </div>
            </div>
            <div class="card">
              <div class="h-eyebrow">Overview</div>
              <div style="display:grid; grid-template-columns:repeat(3,1fr); gap:0.4rem; margin-top:0.4rem;">
                <div class="field"><span class="field-label">Category</span><div class="field-input dim">Longsword</div></div>
                <div class="field"><span class="field-label">Material</span><div class="field-input">Veridium</div></div>
                <div class="field"><span class="field-label">Item Level</span><div class="field-input mono">14</div></div>
              </div>
            </div>
            <div class="card" style="padding:0;">
              <div class="row" style="padding:0.4rem 0.6rem; border-bottom:1px solid var(--line);"><span class="h-eyebrow grow">Properties</span><span class="mono muted" style="font-size:0.7rem;">3 of 8 slots</span></div>
              <div class="prop-row" style="background:rgba(255,255,255,0.015);">
                <span class="h-eyebrow">Property</span>
                <span class="h-eyebrow">Power</span>
                <span></span>
              </div>
              ${[["Damage bonus","+5"],["Attack bonus","+3"],["Critical chance","2.5"]].map(([n,p])=>`
                <div class="prop-row">
                  <span>${n}</span>
                  <span class="mono" style="color:var(--gold-2);">${p}</span>
                  <button class="btn ghost sm">&times;</button>
                </div>
              `).join("")}
              <div class="prop-row" style="background:rgba(199,154,72,0.04);">
                <div class="field-input dim" style="padding:0.25rem 0.4rem;">Choose property&hellip;</div>
                <div class="field-input dim mono" style="padding:0.25rem 0.4rem;">Power</div>
                <button class="btn sm gold">Add</button>
              </div>
            </div>
          </div>
        </div>
      </div>
      <div class="variant-caption">Same shape as today, but: richer item rows (icon + meta + quantity column), proper item-header strip with portrait + actions, properties as a real card not a bare grid.</div>
    </div>

  </div>
`);
