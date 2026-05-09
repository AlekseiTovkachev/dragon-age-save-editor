// Summary / recommended mix
WF.add("summary", `
  ${WF.intro("Recommended Mix", "If I were the one shipping this, here's the blend I'd take into a hi-fi pass. It assumes a casual editor who opens the tool occasionally to make a tweak, with the DAO palette and DA2-leaning chrome.")}

  <div class="variants cols-1">

    <div class="variant">
      <div class="variant-head">
        <span class="variant-tag">PICK</span>
        <span class="variant-title">My recommendation</span>
      </div>
      <div class="variant-body">
        <div class="col" style="gap:0.6rem;">
          <div class="card">
            <div class="row" style="gap:0.5rem; align-items:flex-start;">
              <span class="chip gold">Shell</span>
              <div class="col grow" style="gap:0.2rem;">
                <strong>Variant A &mdash; Save Identity Card + sidebar nav</strong>
                <span class="muted">The screenshot deserves its space; sidebar nav also gives a permanent home for Save As / Reset / Open without cluttering. Money input moves to Inventory where it makes more sense.</span>
              </div>
            </div>
          </div>
          <div class="card">
            <div class="row" style="gap:0.5rem; align-items:flex-start;">
              <span class="chip gold">Characters</span>
              <div class="col grow" style="gap:0.2rem;">
                <strong>Variant B &mdash; Stat-block hero</strong>
                <span class="muted">Casual users can see and click numbers without a form. The 6-up STR/DEX/&hellip; cards are scannable, and inline-edit-on-click feels right for one-off tweaks.</span>
              </div>
            </div>
          </div>
          <div class="card">
            <div class="row" style="gap:0.5rem; align-items:flex-start;">
              <span class="chip gold">Abilities</span>
              <div class="col grow" style="gap:0.2rem;">
                <strong>Variant A (browser) as the core, with Variant C (loadout-first) as the landing screen</strong>
                <span class="muted">Land on "what this character knows," with a + Add ability button that opens the kind/tree/rank browser. Tree visualization (B) is fun but expensive and not strictly needed for casual editing.</span>
              </div>
            </div>
          </div>
          <div class="card">
            <div class="row" style="gap:0.5rem; align-items:flex-start;">
              <span class="chip gold">Inventory</span>
              <div class="col grow" style="gap:0.2rem;">
                <strong>Variant A &mdash; tile grid + side-rail editor</strong>
                <span class="muted">Most game-like, immediately recognizable as a backpack, and the side-rail keeps the editor visible while you keep clicking items. Tile stack count is iconic.</span>
              </div>
            </div>
          </div>
          <div class="card">
            <div class="row" style="gap:0.5rem; align-items:flex-start;">
              <span class="chip gold">Plot Flags</span>
              <div class="col grow" style="gap:0.2rem;">
                <strong>Variant A &mdash; choice cards</strong>
                <span class="muted">Casual users want to read "who did Hawke side with?", not parse a fieldset legend. Card-per-decision wins on readability.</span>
              </div>
            </div>
          </div>
        </div>

        <div class="sep dashed"></div>

        <div class="card hi">
          <div class="h-eyebrow">Cross-cutting moves</div>
          <ul style="margin:0.4rem 0 0; padding-left:1.1rem; color:var(--ink-2);">
            <li>Money lives in Inventory, not in the tab strip.</li>
            <li>Dirty state is a chip beside the character / save name, not a small caption.</li>
            <li>Blood-red is reserved for "Save As" only &mdash; one primary CTA on the screen at a time.</li>
            <li>Plot flag IDs stay visible (mono, dim) but never lead.</li>
            <li>Sub-tabbar (Overview / Abilities / Equipment) sits next to the character title, not on its own row.</li>
          </ul>
        </div>

        <div class="hand muted" style="font-size:1.05rem;">Tell me which combination resonates and I'll push it to a hi-fi mock next.</div>
      </div>
    </div>

  </div>
`);
