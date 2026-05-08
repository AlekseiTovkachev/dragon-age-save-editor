// Plot Flags variants
WF.add("plotflags", `
  ${WF.intro("Plot Flags &mdash; DA2 Choices", "Today: two parallel columns of grouped fieldsets &mdash; radio choices on the left, boolean checkboxes on the right. A wall of legends and tiny text. Three rethinks &mdash; treating choices like a Codex of decisions, not a settings panel.")}

  <div class="variants cols-1">

    <!-- VARIANT A: Choice cards by category -->
    <div class="variant">
      <div class="variant-head">
        <span class="variant-tag">A</span>
        <span class="variant-title">Choice Cards &mdash; one decision per card</span>
        <span class="variant-note">most readable, each choice gets room to breathe</span>
      </div>
      <div class="variant-body">
        <div class="frame" style="min-height:520px; display:grid; grid-template-rows:auto 1fr;">
          <div class="filter-row">
            <input class="search" placeholder="Search choices&hellip;" />
            <span class="chip gold">Act 1</span>
            <span class="chip">Act 2</span>
            <span class="chip">Act 3</span>
            <span class="chip">Companions</span>
            <div class="spacer"></div>
            <span class="mono muted" style="font-size:0.7rem;">14 of 87 modified</span>
          </div>
          <div style="padding:0.85rem; display:grid; grid-template-columns:repeat(2, 1fr); gap:0.65rem; align-content:start; overflow:auto;">
            ${[
              { q:"Who did you side with at the Qunari uprising?", opts:["Aided the Qunari","Sided with Meredith","Stayed neutral"], picked:1 },
              { q:"Did Anders blow up the Chantry?", opts:["Yes — let him","Stopped him","Anders not recruited"], picked:0 },
              { q:"Hawke's romance", opts:["Anders","Fenris","Isabela","Merrill","Sebastian","No romance"], picked:1 },
              { q:"Bethany's fate", opts:["Killed in Lothering","Joined Circle","Joined Wardens","Survived (rogue)"], picked:2 },
            ].map(c=>`
              <div class="flag-card">
                <div class="row between">
                  <strong style="font-size:0.95rem;">${c.q}</strong>
                  <span class="mono muted" style="font-size:0.65rem;">PLT_${Math.floor(Math.random()*9000)+1000}</span>
                </div>
                <div class="options">
                  ${c.opts.map((o,i)=>`<div class="flag-option ${i===c.picked?"is-active":""}"><span class="marker"></span>${o}</div>`).join("")}
                </div>
              </div>
            `).join("")}
          </div>
        </div>
      </div>
      <div class="variant-caption">Each decision becomes a card. Question first, then big tappable options. Plot flag ID is moved to a small mono code in the corner so casual users see the question, not the codename.</div>
    </div>

    <!-- VARIANT B: List view with inline radio -->
    <div class="variant">
      <div class="variant-head">
        <span class="variant-tag">B</span>
        <span class="variant-title">Codex &mdash; sectioned reading list</span>
        <span class="variant-note">most compact while still readable</span>
      </div>
      <div class="variant-body">
        <div class="frame" style="min-height:520px; display:grid; grid-template-columns:200px 1fr;">
          <aside class="sidebar">
            <div class="sidebar-section-label">Acts</div>
            <div class="side-link is-active"><span class="ico"></span>Act 1 &middot; 12</div>
            <div class="side-link"><span class="ico"></span>Act 2 &middot; 18</div>
            <div class="side-link"><span class="ico"></span>Act 3 &middot; 22</div>
            <div class="sidebar-section-label">Companions</div>
            <div class="side-link"><span class="ico"></span>Anders</div>
            <div class="side-link"><span class="ico"></span>Aveline</div>
            <div class="side-link"><span class="ico"></span>Bethany / Carver</div>
            <div class="side-link"><span class="ico"></span>Fenris</div>
            <div class="side-link"><span class="ico"></span>Isabela</div>
            <div class="side-link"><span class="ico"></span>Merrill</div>
            <div class="side-link"><span class="ico"></span>Varric</div>
          </aside>
          <div style="padding:0.7rem; overflow:auto; display:grid; gap:0.7rem; align-content:start;">
            <div class="h-display" style="font-size:1.1rem;">Act 1 &mdash; The Hawke Family Arrives in Kirkwall</div>

            <div class="col" style="gap:0.4rem;">
              <div class="divider-label">Boolean Flags</div>
              ${["Bethany travels with you","Carver joins the Templars","Found Bartrand's heirloom","Helped Fenris in Hightown","Sided with mages on first quest","Recruited Sebastian"].map((n,i)=>`
                <label class="row card" style="padding:0.4rem 0.55rem; gap:0.55rem; cursor:pointer; align-items:center;">
                  <span style="display:inline-block; width:14px; height:14px; border:1.5px solid ${i%2?"var(--gold)":"var(--ink-3)"}; background:${i%2?"var(--gold)":"transparent"}; border-radius:3px;"></span>
                  <span style="flex:1;">${n}</span>
                  <span class="mono muted" style="font-size:0.7rem;">PLT_${1000+i*37}</span>
                </label>
              `).join("")}
            </div>

            <div class="col" style="gap:0.4rem;">
              <div class="divider-label">Decisions</div>
              ${[
                {q:"Sided with Meeran or Athenril?", opts:["Meeran (mercenary)","Athenril (smuggler)"], picked:1},
                {q:"Class chosen for Hawke", opts:["Warrior","Mage","Rogue"], picked:1},
              ].map(c=>`
                <div class="card" style="padding:0.5rem 0.6rem;">
                  <div class="row between" style="margin-bottom:0.4rem;"><strong>${c.q}</strong><span class="mono muted" style="font-size:0.7rem;">PLT_${Math.floor(Math.random()*9000)+1000}</span></div>
                  <div class="row" style="gap:0.3rem; flex-wrap:wrap;">
                    ${c.opts.map((o,i)=>`<div class="tree-pill ${i===c.picked?"is-active":""}">${o}</div>`).join("")}
                  </div>
                </div>
              `).join("")}
            </div>
          </div>
        </div>
      </div>
      <div class="variant-caption">Sidebar by Act/companion. Mixed booleans + decisions on one page, but typographically distinct. Plot flag IDs on the right edge in mono so search-by-ID still works.</div>
    </div>

  </div>
`);
