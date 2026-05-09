// App Shell — topbar + section navigation variants
WF.add("shell", `
  ${WF.intro("App Shell &mdash; Topbar &amp; Navigation", "Today: dense topbar with title, document path, character name, screenshot thumbnail, and 4 buttons; below it a tab strip with a Money input wedged on the right. Three rethinks below — each tries to fix the &lsquo;weak identity, awkward Money slot, redundant chrome&rsquo; problems.")}

  <div class="variants cols-1">

    <!-- VARIANT A: Save Identity Card -->
    <div class="variant">
      <div class="variant-head">
        <span class="variant-tag">A</span>
        <span class="variant-title">Save Identity Card &mdash; sidebar nav</span>
        <span class="variant-note">screenshot becomes the save's "face"</span>
      </div>
      <div class="variant-body">
        <div class="frame" style="display:grid; grid-template-columns: 240px 1fr; min-height:440px;">
          <!-- Sidebar -->
          <aside class="sidebar">
            <div class="card hi" style="margin-bottom:0.6rem; padding:0.55rem;">
              <div class="save-shot" style="aspect-ratio: 16/9; margin-bottom:0.55rem;"></div>
              <div class="h-display" style="font-size:0.9rem;">Solona Amell</div>
              <div class="mono muted" style="font-size:0.7rem;">DAO &middot; lvl 14 &middot; Lothering</div>
              <div class="row" style="margin-top:0.4rem; justify-content:space-between;">
                <span class="chip blood"><span class="dot" style="background:#b3302a;"></span>Unsaved</span>
                <span class="mono muted" style="font-size:0.65rem;">Slot 03</span>
              </div>
            </div>
            <div class="sidebar-section-label">Edit</div>
            <div class="side-link is-active"><span class="ico"></span>Characters</div>
            <div class="side-link"><span class="ico"></span>Inventory</div>
            <div class="side-link"><span class="ico"></span>Recipes</div>
            <div class="side-link"><span class="ico"></span>Plot Flags</div>
            <div class="spacer"></div>
            <div class="sidebar-section-label">Save</div>
            <div class="side-link"><span class="ico"></span>Open Save&hellip;</div>
            <div class="side-link"><span class="ico"></span>Reset Drafts</div>
            <div class="side-link" style="color:#f0c5c0; border-color:rgba(179,48,42,0.4); background:rgba(139,29,29,0.10);"><span class="ico" style="background:#b3302a;"></span>Save As&hellip;</div>
          </aside>
          <div style="padding: 1rem;">
            <div class="placeholder huge">Active section content area</div>
            <div class="muted hand" style="margin-top:0.5rem; font-size:1.05rem;">No more topbar. The save IS the identity card.</div>
          </div>
        </div>
      </div>
      <div class="variant-caption">
        The screenshot earns its keep: it's always visible at a glance and doubles as the "is this the right save?" check. Money moves into Inventory where it belongs. Save As gets primary-button treatment via blood-red accent.
      </div>
    </div>

    <!-- VARIANT B: Slim topbar with hero shot popover -->
    <div class="variant">
      <div class="variant-head">
        <span class="variant-tag">B</span>
        <span class="variant-title">Slim Topbar + Tabs &mdash; closer to today</span>
        <span class="variant-note">least disruptive change</span>
      </div>
      <div class="variant-body">
        <div class="frame frame-app">
          <div class="tbar between">
            <div class="row">
              <div class="save-shot" style="width:46px; height:46px; border-radius:4px;"></div>
              <div class="col" style="gap:0.1rem;">
                <div class="h-display" style="font-size:0.95rem;">Solona Amell</div>
                <div class="mono muted" style="font-size:0.7rem;">DAO &middot; Slot03_Lothering.das</div>
              </div>
              <span class="chip blood" style="margin-left:0.5rem;"><span class="dot" style="background:#b3302a;"></span>Unsaved changes</span>
            </div>
            <div class="row">
              <button class="btn ghost">Open</button>
              <button class="btn ghost">Reset</button>
              <button class="btn primary">&#8623; Save As</button>
            </div>
          </div>
          <div class="tbar" style="background:transparent; padding:0.4rem 0.85rem;">
            <div class="toggle-bar">
              <button class="is-active">Characters</button>
              <button>Inventory</button>
              <button>Recipes</button>
              <button>Plot Flags</button>
            </div>
            <div class="spacer"></div>
            <div class="row">
              <span class="h-eyebrow">PARTY GOLD</span>
              <input class="field-input mono" style="width:110px; padding:0.25rem 0.4rem;" value="42 314" />
            </div>
          </div>
          <div style="padding:1rem;">
            <div class="placeholder huge">Active section content area</div>
          </div>
        </div>
      </div>
      <div class="variant-caption">
        Same shape as today, but: screenshot+name as one identity unit, dirty-state as a chip beside the name, segmented control instead of tabs+borders, Save As is the only blood-red CTA.
      </div>
    </div>

    <!-- VARIANT C: Command bar -->
    <div class="variant">
      <div class="variant-head">
        <span class="variant-tag">C</span>
        <span class="variant-title">Codex-style header &mdash; no tabs at top</span>
        <span class="variant-note">most adventurous &mdash; nav becomes a left rail</span>
      </div>
      <div class="variant-body">
        <div class="frame" style="display:grid; grid-template-rows:auto 1fr; min-height:440px;">
          <div class="tbar between" style="padding: 0.85rem 1rem; background: linear-gradient(180deg, #1a140e, #0e0c0a);">
            <div class="row" style="gap:0.75rem;">
              <div class="save-shot" style="width:88px; height:54px; border-radius:4px;"></div>
              <div class="col" style="gap:0.15rem;">
                <div class="h-display" style="font-size:1.2rem;">Hawke &mdash; Champion of Kirkwall</div>
                <div class="mono muted" style="font-size:0.7rem;">DA2 &middot; Act 2 &middot; HighTown_Estate.das</div>
              </div>
            </div>
            <div class="row">
              <span class="chip gold"><span class="dot"></span>Saved copy ready</span>
              <button class="btn ghost">Open Save</button>
              <button class="btn">Commit</button>
              <button class="btn primary">Save As</button>
            </div>
          </div>
          <div style="display:grid; grid-template-columns:200px 1fr; min-height:0;">
            <aside class="sidebar" style="border-right:1px solid var(--line); padding-top:0.75rem;">
              <div class="side-link is-active"><span class="ico"></span>Characters &amp; Party</div>
              <div class="side-link"><span class="ico"></span>Inventory</div>
              <div class="side-link"><span class="ico"></span>Crafting</div>
              <div class="side-link"><span class="ico"></span>Plot &amp; Choices</div>
            </aside>
            <div style="padding:1rem;">
              <div class="placeholder huge">Active section content area</div>
            </div>
          </div>
        </div>
      </div>
      <div class="variant-caption">
        Hero header gives the save real presence (screenshot at 88×54 is enough to recognize without dominating). Nav is a left rail, eliminating the section-tab + character-tab nesting. Money moves to Inventory.
      </div>
    </div>

  </div>
`);
