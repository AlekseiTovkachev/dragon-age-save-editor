// System / palette / type sample
WF.add("system", `
  ${WF.intro("Visual System", "Dragon Age Origins palette: warm parchment ink on near-black, blood red for primary actions, gold for accents and section markers, faded mage blue for informational chips. DA2-style modern grid layout — clean panels and clear hierarchy, but with sketchy texture and a display serif for headings to keep the in-world feel.")}

  <div class="variants cols-2">

    <div class="variant">
      <div class="variant-head">
        <span class="variant-tag">PALETTE</span>
        <span class="variant-title">Color Tokens</span>
      </div>
      <div class="variant-body">
        <div class="row" style="gap:1rem; flex-wrap:wrap;">
          <div class="col" style="gap:0.4rem;">
            <span class="h-eyebrow">Surfaces</span>
            <div class="row"><span class="swatch" style="background:#0e0c0a"></span><span class="mono">--bg #0e0c0a</span></div>
            <div class="row"><span class="swatch" style="background:#15110d"></span><span class="mono">--bg-1 #15110d</span></div>
            <div class="row"><span class="swatch" style="background:#1c1814"></span><span class="mono">--bg-2 #1c1814</span></div>
            <div class="row"><span class="swatch" style="background:#25201a"></span><span class="mono">--bg-3 #25201a</span></div>
          </div>
          <div class="col" style="gap:0.4rem;">
            <span class="h-eyebrow">Ink</span>
            <div class="row"><span class="swatch" style="background:#f3ead6"></span><span class="mono">parchment</span></div>
            <div class="row"><span class="swatch" style="background:#c9bfa3"></span><span class="mono">dim</span></div>
            <div class="row"><span class="swatch" style="background:#807663"></span><span class="mono">muted</span></div>
            <div class="row"><span class="swatch" style="background:#594a37"></span><span class="mono">divider</span></div>
          </div>
          <div class="col" style="gap:0.4rem;">
            <span class="h-eyebrow">Accents</span>
            <div class="row"><span class="swatch" style="background:#8b1d1d"></span><span class="mono">blood</span></div>
            <div class="row"><span class="swatch" style="background:#b3302a"></span><span class="mono">blood bright</span></div>
            <div class="row"><span class="swatch" style="background:#c79a48"></span><span class="mono">gold</span></div>
            <div class="row"><span class="swatch" style="background:#e6c277"></span><span class="mono">gold high</span></div>
            <div class="row"><span class="swatch" style="background:#5a7794"></span><span class="mono">rune blue</span></div>
            <div class="row"><span class="swatch" style="background:#6a7a3f"></span><span class="mono">moss</span></div>
          </div>
        </div>
      </div>
      <div class="variant-caption">
        Blood red is reserved for primary CTAs (Save As) and "destructive committed" states. Gold marks active selection and section identity.
      </div>
    </div>

    <div class="variant">
      <div class="variant-head">
        <span class="variant-tag">TYPE</span>
        <span class="variant-title">Type Pairing</span>
      </div>
      <div class="variant-body">
        <h2 class="h-display" style="font-size:2rem; margin-bottom:0.25rem;">The Hero of Ferelden</h2>
        <div class="mono" style="color:var(--ink-3); margin-bottom:0.6rem;">IM Fell English SC &middot; display headings</div>
        <p style="margin:0 0 0.5rem; font-family:var(--font-body); font-size:0.95rem;">Inter for body and form copy. Comfortable at 14px in a desktop-tool density. Used for everything that isn't a section header or a number.</p>
        <p style="margin:0; font-family:var(--font-mono); font-size:0.8rem; color:var(--ink-2);">JetBrains Mono &middot; numbers, codes, paths, plot flag IDs</p>
      </div>
      <div class="variant-caption">
        Three families. Display serif for personality, sans for legibility, mono for the technical bits (resrefs, IDs, file paths).
      </div>
    </div>

    <div class="variant" style="grid-column: 1 / -1;">
      <div class="variant-head">
        <span class="variant-tag">CHIPS</span>
        <span class="variant-title">Status & Tag Vocabulary</span>
      </div>
      <div class="variant-body">
        <div class="row" style="gap:0.5rem;">
          <span class="chip blood"><span class="dot" style="background:#b3302a;"></span>Unsaved changes</span>
          <span class="chip gold"><span class="dot"></span>Saved copy ready</span>
          <span class="chip rune">DAO Awakening</span>
          <span class="chip moss">DA2</span>
          <span class="chip">resref&nbsp;<span class="mono">gen_im_wep_swd_lng_blk</span></span>
        </div>
      </div>
      <div class="variant-caption">
        Status uses small dots so colour-blind users still get a label. Game tags use muted accents instead of full colour washes.
      </div>
    </div>

  </div>
`);
