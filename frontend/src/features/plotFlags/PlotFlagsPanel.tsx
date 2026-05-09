import { useMemo, useState, type ReactNode } from "react";
import { EmptyState, Panel, PanelBody } from "../../components/ui";
import type { PlotBooleanFlag, PlotIntegerFlag, SaveSummary } from "../../types";

type PlotFlagsPanelProps = {
  state: PlotFlagsPanelState;
  actions: PlotFlagsPanelActions;
  summary: SaveSummary;
  canEdit: boolean;
  busy: boolean;
};

export type PlotBooleanGroup = {
  category: string;
  flags: PlotBooleanFlag[];
};

export type PlotIntegerGroup = {
  category: string;
  flags: PlotIntegerFlag[];
};

export type PlotFlagsPanelState = {
  plotIntegerValues: Record<number, number>;
  plotIntegerDrafts: Record<number, number>;
  plotBooleanValues: Record<number, boolean>;
  plotBooleanDrafts: Record<number, boolean>;
  groupedPlotIntegers: PlotIntegerGroup[];
  groupedPlotBooleans: PlotBooleanGroup[];
};

export type PlotFlagsPanelActions = {
  handleIntegerChange: (id: number, value: number) => void;
  handleBooleanToggle: (id: number, value: boolean) => void;
};

// ─── Declarative spec types ──────────────────────────────────────────────────

type ExclusiveBooleanOption = {
  label: string;
  setTrue: number[];  // flags to set true; all other flagIds in group are set false
};

type ExclusiveBooleanGroup = {
  label: string;
  flagIds: number[];       // ALL flag IDs in this group (for EXCLUSIVE_FLAG_IDS set)
  options?: ExclusiveBooleanOption[];  // if absent, derive one option per flagId using flag description
  hasNone?: boolean;       // show a "None" option that sets all flagIds to false
};

type PlotItem =
  | { kind: "exclusive"; def: ExclusiveBooleanGroup; showIf?: { id: number; value: boolean } }
  | { kind: "boolean"; id: number; label?: string; showIf?: { id: number; value: boolean } }
  | { kind: "integer"; id: number; showIf?: { id: number; value: boolean } };

type PlotSection = {
  title: string;
  items: PlotItem[];
};

// ─── Card sub-component props ─────────────────────────────────────────────────

type PlotCardFooterProps = {
  meta: string;
  modified: boolean;
};

type PlotChoiceCardProps = {
  flag: PlotIntegerFlag;
  committedValue: number | undefined;
  draftValue: number | undefined;
  disabled: boolean;
  onChange: (id: number, value: number) => void;
};

type PlotBooleanCardProps = {
  flag: PlotBooleanFlag;
  committedValue: boolean | undefined;
  draftValue: boolean | undefined;
  disabled: boolean;
  onToggle: (id: number, value: boolean) => void;
  labelOverride?: string;
};

type PlotExclusiveGroupCardProps = {
  def: ExclusiveBooleanGroup;
  flags: PlotBooleanFlag[];  // flags for this group (looked up by ID from boolFlagMap)
  committedValues: Record<number, boolean>;
  draftValues: Record<number, boolean>;
  disabled: boolean;
  onToggle: (id: number, value: boolean) => void;
};

// ─── PLOT_SECTIONS ────────────────────────────────────────────────────────────

const PLOT_SECTIONS: PlotSection[] = [
  {
    title: "Warden",
    items: [
      { kind: "integer", id: 1000 },
      { kind: "integer", id: 1001 },
      {
        kind: "exclusive",
        def: {
          label: "Origin",
          flagIds: [2000, 2001, 2002, 2003, 2004, 2005],
          options: [
            { label: "Circle Mage", setTrue: [2000] },
            { label: "Dwarf Commoner", setTrue: [2001] },
            { label: "Dwarf Noble", setTrue: [2002] },
            { label: "City Elf", setTrue: [2003] },
            { label: "Dalish Elf", setTrue: [2004] },
            { label: "Human Noble", setTrue: [2005] },
          ],
        },
      },
    ],
  },
  {
    title: "Broken Circle",
    items: [
      {
        kind: "exclusive",
        def: {
          label: "Sided with",
          flagIds: [2012, 2013],
          options: [
            { label: "Circle of Magi", setTrue: [2012] },
            { label: "Templars", setTrue: [2013] },
          ],
        },
      },
    ],
  },
  {
    title: "Nature of the Beast",
    items: [
      {
        kind: "exclusive",
        def: {
          label: "Sided with",
          flagIds: [2015, 2016, 2017],
          options: [
            { label: "Dalish Elves (werewolves killed)", setTrue: [2015] },
            { label: "Both sides — Zathrian's sacrifice", setTrue: [2015, 2017] },
            { label: "Werewolves (elves killed)", setTrue: [2016] },
          ],
        },
      },
    ],
  },
  {
    title: "Orzammar",
    items: [
      {
        kind: "exclusive",
        def: {
          label: "King",
          flagIds: [2018, 2019],
          options: [
            { label: "King Bhelen", setTrue: [2018] },
            { label: "King Harrowmont", setTrue: [2019] },
          ],
        },
      },
      {
        kind: "exclusive",
        def: {
          label: "Anvil of the Void",
          flagIds: [2081, 2082, 2083],
          options: [
            { label: "Branka victorious — Anvil preserved", setTrue: [2081] },
            { label: "Branka — destroyed herself and the Anvil", setTrue: [2082] },
            { label: "Caridin's wish — Anvil destroyed", setTrue: [2083] },
          ],
        },
      },
    ],
  },
  {
    title: "Arl of Redcliffe",
    items: [
      { kind: "boolean", id: 2099, label: "Village abandoned" },
      {
        kind: "exclusive",
        def: {
          label: "Connor's Fate",
          flagIds: [2006, 2007, 2008, 2086, 2087],
          options: [
            { label: "Saved via Circle mages", setTrue: [2007, 2087] },
            { label: "Saved via blood magic (Jowan)", setTrue: [2007, 2086] },
            { label: "Isolde killed Connor", setTrue: [2006] },
            { label: "Connor killed by Warden", setTrue: [2008] },
          ],
        },
      },
      { kind: "boolean", id: 2014, label: "Andraste's ashes revealed" },
    ],
  },
  {
    title: "Landsmeet",
    items: [
      {
        kind: "exclusive",
        def: {
          label: "Who Rules Ferelden",
          flagIds: [2020, 2021, 2024, 2026, 2027],
          options: [
            { label: "Alistair and Anora (co-rulers)", setTrue: [2020] },
            { label: "Alistair and the Warden (wed)", setTrue: [2026] },
            { label: "Anora and the Warden (wed)", setTrue: [2027, 2024] },
            { label: "Alistair alone", setTrue: [2021] },
            { label: "Anora alone", setTrue: [2027] },
          ],
        },
      },
      {
        kind: "exclusive",
        def: {
          label: "Alistair's Fate",
          flagIds: [2022, 2023],
          options: [
            { label: "Exiled", setTrue: [2022] },
            { label: "Executed", setTrue: [2023] },
          ],
          hasNone: true,
        },
      },
      {
        kind: "exclusive",
        def: {
          label: "Loghain",
          flagIds: [2025, 2097],
          options: [
            { label: "Recruited to Grey Wardens", setTrue: [2097] },
            { label: "Executed", setTrue: [2025] },
          ],
        },
      },
    ],
  },
  {
    title: "Morrigan's Ritual",
    items: [
      {
        kind: "exclusive",
        def: {
          label: "Ritual Partner",
          flagIds: [2034, 2035, 2036, 2100, 2101, 2102],
          options: [
            { label: "Alistair", setTrue: [2034, 2100] },
            { label: "Warden", setTrue: [2035, 2102] },
            { label: "Loghain", setTrue: [2036, 2101] },
          ],
          hasNone: true,
        },
      },
      { kind: "boolean", id: 2104, label: "Ritual accepted" },
    ],
  },
  {
    title: "Finale",
    items: [
      {
        kind: "exclusive",
        def: {
          label: "Slew the Archdemon",
          flagIds: [2028, 2029, 2030],
          options: [
            { label: "Alistair", setTrue: [2028] },
            { label: "The Warden", setTrue: [2030] },
            { label: "Loghain", setTrue: [2029] },
          ],
        },
      },
      {
        kind: "exclusive",
        def: {
          label: "Epilogue Boon",
          flagIds: [2031, 2032, 2033],
          options: [
            { label: "Circle of Magi restored", setTrue: [2032] },
            { label: "Dalish granted land", setTrue: [2033] },
            { label: "Warden made Chancellor", setTrue: [2031] },
          ],
          hasNone: true,
        },
      },
    ],
  },
  {
    title: "Companions",
    items: [
      { kind: "boolean", id: 2040, label: "Alistair romance active" },
      { kind: "boolean", id: 2041, label: "Alistair slept with Warden" },
    ],
  },
  {
    title: "Leliana",
    items: [
      {
        kind: "exclusive",
        def: {
          label: "Status",
          flagIds: [2038, 2044],
          options: [
            { label: "Recruited in Lothering", setTrue: [2038] },
            { label: "Not recruited", setTrue: [2044] },
          ],
          hasNone: true,
        },
      },
      { kind: "boolean", id: 2045, label: "Attacked the Warden" },
      { kind: "boolean", id: 2042, label: "Romance active" },
      { kind: "boolean", id: 2043, label: "Slept with Warden" },
    ],
  },
  {
    title: "Zevran",
    items: [
      {
        kind: "exclusive",
        def: {
          label: "Recruitment",
          flagIds: [2039, 2105, 2106],
          options: [
            { label: "Recruited", setTrue: [2039] },
            { label: "Spared and sent away", setTrue: [2105] },
            { label: "Killed at ambush", setTrue: [2106] },
          ],
          hasNone: true,
        },
      },
      {
        kind: "exclusive",
        def: {
          label: "Fate",
          flagIds: [2050, 2051, 2107, 2108],
          options: [
            { label: "Left the Wardens", setTrue: [2051] },
            { label: "Left with a farewell", setTrue: [2107] },
            { label: "Left for good (no return)", setTrue: [2108] },
            { label: "Turned hostile", setTrue: [2050] },
          ],
          hasNone: true,
        },
      },
      { kind: "boolean", id: 2048, label: "Romance active" },
      { kind: "boolean", id: 2049, label: "Slept with Warden" },
    ],
  },
  {
    title: "Isabela",
    items: [
      {
        kind: "exclusive",
        def: {
          label: "Encounter",
          flagIds: [2052, 2053, 2054, 2055, 2056],
          options: [
            { label: "With Warden only", setTrue: [2056] },
            { label: "Threesome with Alistair", setTrue: [2052] },
            { label: "Threesome with Leliana", setTrue: [2053] },
            { label: "Threesome with Zevran", setTrue: [2054] },
            { label: "Foursome with Zevran and Leliana", setTrue: [2055] },
          ],
          hasNone: true,
        },
      },
    ],
  },
  {
    title: "Awakening",
    items: [
      { kind: "boolean", id: 2057, label: "Orlesian Warden-Commander" },
    ],
  },
  {
    title: "Anders",
    items: [
      { kind: "boolean", id: 2064, label: "Recruited" },
      { kind: "boolean", id: 2066, label: "Died during siege" },
    ],
  },
  {
    title: "Nathaniel",
    items: [
      { kind: "boolean", id: 2065, label: "Recruited" },
      { kind: "boolean", id: 2084, label: "Died during siege" },
      { kind: "boolean", id: 2085, label: "Remained friendly" },
    ],
  },
  {
    title: "Vigil's Keep",
    items: [
      {
        kind: "exclusive",
        def: {
          label: "Reinforced",
          flagIds: [2060, 2061],
          options: [
            { label: "Roads", setTrue: [2060] },
            { label: "Farms", setTrue: [2061] },
            { label: "Both roads and farms", setTrue: [2060, 2061] },
          ],
        },
      },
      { kind: "boolean", id: 2062, label: "Silverite walls upgraded" },
      {
        kind: "exclusive",
        def: {
          label: "Siege — saved",
          flagIds: [2058, 2059],
          options: [
            { label: "Vigil's Keep", setTrue: [2058] },
            { label: "Amaranthine", setTrue: [2059] },
          ],
        },
      },
      {
        kind: "exclusive",
        def: {
          label: "The Architect",
          flagIds: [2063, 2096],
          options: [
            { label: "Killed", setTrue: [2063] },
            { label: "Spared", setTrue: [2096] },
          ],
        },
      },
    ],
  },
  {
    title: "Warden's Keep (DLC)",
    items: [
      { kind: "boolean", id: 2069, label: "Completed" },
      { kind: "boolean", id: 2067, label: "Sophia killed", showIf: { id: 2069, value: true } },
      {
        kind: "exclusive",
        def: {
          label: "Avernus",
          flagIds: [2068, 2070, 2071],
          options: [
            { label: "Killed", setTrue: [2068] },
            { label: "Allowed evil experiments", setTrue: [2070] },
            { label: "Redirected to ethical research", setTrue: [2071] },
          ],
          hasNone: true,
        },
        showIf: { id: 2069, value: true },
      },
    ],
  },
  {
    title: "The Stone Prisoner (DLC)",
    items: [
      {
        kind: "exclusive",
        def: {
          label: "Shale",
          flagIds: [2075, 2076, 2077],
          options: [
            { label: "Left in Honnleath", setTrue: [2075] },
            { label: "Recruited and survived", setTrue: [2076] },
            { label: "Turned on the Warden", setTrue: [2077] },
          ],
          hasNone: true,
        },
      },
    ],
  },
  {
    title: "Return to Ostagar (DLC)",
    items: [
      {
        kind: "exclusive",
        def: {
          label: "Cailan's Body",
          flagIds: [2072, 2073, 2074],
          options: [
            { label: "Given a proper burial (burned)", setTrue: [2072] },
            { label: "Left to the darkspawn", setTrue: [2073] },
            { label: "Fed to wolves", setTrue: [2074] },
          ],
          hasNone: true,
        },
      },
    ],
  },
  {
    title: "Witch Hunt (DLC)",
    items: [
      {
        kind: "exclusive",
        def: {
          label: "Morrigan's Fate",
          flagIds: [2078, 2079, 2080],
          options: [
            { label: "Let go — stepped through the mirror", setTrue: [2078] },
            { label: "Warden followed her through", setTrue: [2079] },
            { label: "Stabbed before she could escape", setTrue: [2080] },
          ],
          hasNone: true,
        },
      },
    ],
  },
];

// Fast lookup set — all boolean flag IDs that are part of an exclusive group
const EXCLUSIVE_FLAG_IDS = new Set(
  PLOT_SECTIONS.flatMap((s) =>
    s.items.flatMap((item) => (item.kind === "exclusive" ? item.def.flagIds : [])),
  ),
);

// ─── Helpers ──────────────────────────────────────────────────────────────────

function getSelectedOption(
  def: ExclusiveBooleanGroup & { options: ExclusiveBooleanOption[] },
  draftValues: Record<number, boolean>,
): ExclusiveBooleanOption | null {
  return (
    def.options.find((opt) =>
      opt.setTrue.every((id) => draftValues[id]) &&
      def.flagIds.filter((id) => !opt.setTrue.includes(id)).every((id) => !draftValues[id]),
    ) ?? null
  );
}

function handleExclusiveSelect(
  def: ExclusiveBooleanGroup,
  opt: ExclusiveBooleanOption | null,
  onToggle: (id: number, value: boolean) => void,
) {
  for (const id of def.flagIds) {
    onToggle(id, opt !== null && opt.setTrue.includes(id));
  }
}

function stripGroupPrefix(description: string, groupLabel: string): string {
  const prefix = groupLabel + ": ";
  return description.startsWith(prefix) ? description.slice(prefix.length) : description;
}

function isItemVisible(item: PlotItem, boolDrafts: Record<number, boolean>): boolean {
  if (!item.showIf) return true;
  return Boolean(boolDrafts[item.showIf.id]) === item.showIf.value;
}

// ─── Main panel ───────────────────────────────────────────────────────────────

const ALL_CATEGORIES = "All";

export function PlotFlagsPanel({ state, actions, summary, canEdit, busy }: PlotFlagsPanelProps) {
  const [search, setSearch] = useState("");
  const [category, setCategory] = useState(ALL_CATEGORIES);
  const disabled = !canEdit || busy;

  const boolFlagMap = useMemo(() => {
    const map = new Map<number, PlotBooleanFlag>();
    for (const group of state.groupedPlotBooleans) {
      for (const flag of group.flags) map.set(flag.id, flag);
    }
    return map;
  }, [state.groupedPlotBooleans]);

  const intFlagMap = useMemo(() => {
    const map = new Map<number, PlotIntegerFlag>();
    for (const group of state.groupedPlotIntegers) {
      for (const flag of group.flags) map.set(flag.id, flag);
    }
    return map;
  }, [state.groupedPlotIntegers]);

  const categories = useMemo(
    () => [ALL_CATEGORIES, ...PLOT_SECTIONS.map((s) => s.title)],
    [],
  );

  const modifiedCount = useMemo(() => countModifiedFlags(state), [state]);

  // For search: total spec-visible item count
  const totalSpecCount = useMemo(() => {
    let n = 0;
    for (const section of PLOT_SECTIONS) {
      for (const item of section.items) {
        if (!isItemVisible(item, state.plotBooleanDrafts)) continue;
        n++;
      }
    }
    return n;
  }, [state.plotBooleanDrafts]);

  if (summary.preferred_game !== "da2") {
    return (
      <Panel className="detail-panel" title="DA2 Plot Flags" scroll>
        <PanelBody>
          <EmptyState>Plot flag editing is available for DA2 saves.</EmptyState>
        </PanelBody>
      </Panel>
    );
  }

  const query = search.trim().toLowerCase();
  const isFiltering = query !== "" || category !== ALL_CATEGORIES;

  return (
    <Panel
      className="detail-panel plot-flags-panel"
      title={
        <div>
          <div className="crumb">Edit &middot; DA2 Save</div>
          <h2>Plot Flags</h2>
        </div>
      }
      headingAction={
        <span className="mono muted plot-modified-count">
          {modifiedCount} of {totalSpecCount} modified
        </span>
      }
      scroll
    >
      <PanelBody>
        <div className="plot-flags-stack">
          <div className="plot-toolbar">
            <input
              className="search-input plot-search"
              type="search"
              value={search}
              onChange={(event) => setSearch(event.target.value)}
              placeholder="Search Warden's choices..."
              aria-label="Search Warden's choices"
            />
            <div className="plot-chip-row" aria-label="Plot flag categories">
              {categories.map((entry) => (
                <button
                  key={`plot-category-${entry}`}
                  type="button"
                  className={["cat-chip", entry === category ? "is-active" : ""].filter(Boolean).join(" ")}
                  onClick={() => setCategory(entry)}
                  aria-pressed={entry === category}
                >
                  {entry}
                </button>
              ))}
            </div>
          </div>

          {isFiltering ? (
            <FlatFilteredView
              query={query}
              category={category}
              state={state}
              boolFlagMap={boolFlagMap}
              intFlagMap={intFlagMap}
              disabled={disabled}
              actions={actions}
            />
          ) : (
            <SectionedView
              state={state}
              boolFlagMap={boolFlagMap}
              intFlagMap={intFlagMap}
              disabled={disabled}
              actions={actions}
            />
          )}
        </div>
      </PanelBody>
    </Panel>
  );
}

// ─── Sectioned (no filter) view ───────────────────────────────────────────────

type ViewProps = {
  state: PlotFlagsPanelState;
  boolFlagMap: Map<number, PlotBooleanFlag>;
  intFlagMap: Map<number, PlotIntegerFlag>;
  disabled: boolean;
  actions: PlotFlagsPanelActions;
};

function SectionedView({ state, boolFlagMap, intFlagMap, disabled, actions }: ViewProps) {
  const nodes: ReactNode[] = [];

  for (const section of PLOT_SECTIONS) {
    const sectionItems: ReactNode[] = [];

    for (const item of section.items) {
      if (!isItemVisible(item, state.plotBooleanDrafts)) continue;

      if (item.kind === "integer") {
        const flag = intFlagMap.get(item.id);
        if (!flag) continue;
        sectionItems.push(
          <PlotChoiceCard
            key={`int-${item.id}`}
            flag={flag}
            committedValue={state.plotIntegerValues[item.id]}
            draftValue={state.plotIntegerDrafts[item.id]}
            disabled={disabled}
            onChange={actions.handleIntegerChange}
          />,
        );
      } else if (item.kind === "boolean") {
        const flag = boolFlagMap.get(item.id);
        if (!flag) continue;
        sectionItems.push(
          <PlotBooleanCard
            key={`bool-${item.id}`}
            flag={flag}
            committedValue={state.plotBooleanValues[item.id]}
            draftValue={state.plotBooleanDrafts[item.id]}
            disabled={disabled}
            onToggle={actions.handleBooleanToggle}
            labelOverride={item.label}
          />,
        );
      } else if (item.kind === "exclusive") {
        const flags = item.def.flagIds
          .map((id) => boolFlagMap.get(id))
          .filter((f): f is PlotBooleanFlag => f !== undefined);
        sectionItems.push(
          <PlotExclusiveGroupCard
            key={`excl-${item.def.label}`}
            def={item.def}
            flags={flags}
            committedValues={state.plotBooleanValues}
            draftValues={state.plotBooleanDrafts}
            disabled={disabled}
            onToggle={actions.handleBooleanToggle}
          />,
        );
      }
    }

    if (sectionItems.length === 0) continue;

    nodes.push(
      <h2 key={`section-${section.title}`} className="plot-section-header">
        {section.title}
      </h2>,
    );
    nodes.push(...sectionItems);
  }

  if (nodes.length === 0) {
    return <div className="plot-empty">No plot flags available.</div>;
  }

  return <div className="plot-grid">{nodes}</div>;
}

// ─── Flat filtered view ───────────────────────────────────────────────────────

type FlatFilteredViewProps = ViewProps & {
  query: string;
  category: string;
};

function FlatFilteredView({ query, category, state, boolFlagMap, intFlagMap, disabled, actions }: FlatFilteredViewProps) {
  const nodes: ReactNode[] = [];

  for (const section of PLOT_SECTIONS) {
    if (category !== ALL_CATEGORIES && section.title !== category) continue;

    for (const item of section.items) {
      if (!isItemVisible(item, state.plotBooleanDrafts)) continue;

      if (item.kind === "integer") {
        const flag = intFlagMap.get(item.id);
        if (!flag) continue;
        if (query && !matchesQuery([flag.description, flag.name, flag.id.toString()], query)) continue;
        nodes.push(
          <PlotChoiceCard
            key={`int-${item.id}`}
            flag={flag}
            committedValue={state.plotIntegerValues[item.id]}
            draftValue={state.plotIntegerDrafts[item.id]}
            disabled={disabled}
            onChange={actions.handleIntegerChange}
          />,
        );
      } else if (item.kind === "boolean") {
        const flag = boolFlagMap.get(item.id);
        if (!flag) continue;
        const displayLabel = item.label ?? flag.description;
        if (query && !matchesQuery([displayLabel, flag.name, flag.id.toString()], query)) continue;
        nodes.push(
          <PlotBooleanCard
            key={`bool-${item.id}`}
            flag={flag}
            committedValue={state.plotBooleanValues[item.id]}
            draftValue={state.plotBooleanDrafts[item.id]}
            disabled={disabled}
            onToggle={actions.handleBooleanToggle}
            labelOverride={item.label}
          />,
        );
      } else if (item.kind === "exclusive") {
        const flags = item.def.flagIds
          .map((id) => boolFlagMap.get(id))
          .filter((f): f is PlotBooleanFlag => f !== undefined);
        const searchTexts = [
          item.def.label,
          ...(item.def.options ? item.def.options.map((o) => o.label) : flags.map((f) => f.description)),
          ...flags.map((f) => f.name),
        ];
        if (query && !matchesQuery(searchTexts, query)) continue;
        nodes.push(
          <PlotExclusiveGroupCard
            key={`excl-${item.def.label}`}
            def={item.def}
            flags={flags}
            committedValues={state.plotBooleanValues}
            draftValues={state.plotBooleanDrafts}
            disabled={disabled}
            onToggle={actions.handleBooleanToggle}
          />,
        );
      }
    }
  }

  if (nodes.length === 0) {
    return <div className="plot-empty">No plot flags match the current filters.</div>;
  }

  return <div className="plot-grid">{nodes}</div>;
}

function matchesQuery(texts: string[], query: string): boolean {
  return texts.some((t) => t.toLowerCase().includes(query));
}

// ─── Sub-components ───────────────────────────────────────────────────────────


function PlotChoiceCard({ flag, committedValue, draftValue, disabled, onChange }: PlotChoiceCardProps) {
  const modified = draftValue !== committedValue;

  return (
    <article className={["plot-card", modified ? "is-modified" : ""].filter(Boolean).join(" ")}>
      <PlotQuestion flag={flag} />
      <div className="plot-options" role="radiogroup" aria-label={flag.description}>
        {flag.options.map((option) => {
          const selected = draftValue === option.value;
          return (
            <button
              key={`plot-int-${flag.id}-${option.value}`}
              type="button"
              className={["plot-opt", selected ? "is-active" : ""].filter(Boolean).join(" ")}
              onClick={() => onChange(flag.id, option.value)}
              disabled={disabled}
              role="radio"
              aria-checked={selected}
            >
              <span className="marker" aria-hidden="true" />
              <span className="opt-text">{option.label}</span>
            </button>
          );
        })}
      </div>
      <PlotCardFooter meta={`${flag.options.length} options`} modified={modified} />
    </article>
  );
}

function PlotBooleanCard({ flag, committedValue, draftValue, disabled, onToggle, labelOverride }: PlotBooleanCardProps) {
  const selectedValue = Boolean(draftValue);
  const modified = Boolean(draftValue) !== Boolean(committedValue);

  return (
    <article className={["plot-card", "plot-card-boolean", modified ? "is-modified" : ""].filter(Boolean).join(" ")}>
      <PlotQuestion flag={flag} labelOverride={labelOverride} />
      <div className="plot-options plot-boolean-options" role="radiogroup" aria-label={labelOverride ?? flag.description}>
        {[true, false].map((value) => {
          const selected = selectedValue === value;
          return (
            <button
              key={`plot-bool-${flag.id}-${value ? "yes" : "no"}`}
              type="button"
              className={["plot-opt", selected ? "is-active" : ""].filter(Boolean).join(" ")}
              onClick={() => onToggle(flag.id, value)}
              disabled={disabled}
              role="radio"
              aria-checked={selected}
            >
              <span className="marker" aria-hidden="true" />
              <span className="opt-text">{value ? "Yes" : "No"}</span>
            </button>
          );
        })}
      </div>
      <PlotCardFooter meta="Yes / No" modified={modified} />
    </article>
  );
}

function PlotExclusiveGroupCard({ def, flags, committedValues, draftValues, disabled, onToggle }: PlotExclusiveGroupCardProps) {
  const options: ExclusiveBooleanOption[] = def.options ?? flags.map((f) => ({
    label: stripGroupPrefix(f.description, def.label),
    setTrue: [f.id],
  }));

  const defWithOptions = { ...def, options };
  const selectedOpt = getSelectedOption(defWithOptions, draftValues);
  const committedOpt = getSelectedOption(defWithOptions, committedValues);
  const noneSelected = selectedOpt === null;
  const modified = selectedOpt !== committedOpt;

  return (
    <article className={["plot-card", modified ? "is-modified" : ""].filter(Boolean).join(" ")}>
      <div className="question">
        <h3 className="question-text">{def.label}</h3>
      </div>
      <div className="plot-options" role="radiogroup" aria-label={def.label}>
        {options.map((opt, i) => {
          const selected = selectedOpt === opt;
          return (
            <button
              key={`excl-${i}`}
              type="button"
              className={["plot-opt", selected ? "is-active" : ""].filter(Boolean).join(" ")}
              onClick={() => handleExclusiveSelect(def, opt, onToggle)}
              disabled={disabled}
              role="radio"
              aria-checked={selected}
            >
              <span className="marker" aria-hidden="true" />
              <span className="opt-text">{opt.label}</span>
            </button>
          );
        })}
        {def.hasNone ? (
          <button
            key="none"
            type="button"
            className={["plot-opt", noneSelected ? "is-active" : ""].filter(Boolean).join(" ")}
            onClick={() => handleExclusiveSelect(def, null, onToggle)}
            disabled={disabled}
            role="radio"
            aria-checked={noneSelected}
          >
            <span className="marker" aria-hidden="true" />
            <span className="opt-text">None</span>
          </button>
        ) : null}
      </div>
      <PlotCardFooter
        meta={`${options.length + (def.hasNone ? 1 : 0)} options`}
        modified={modified}
      />
    </article>
  );
}

function PlotQuestion({ flag, labelOverride }: { flag: PlotBooleanFlag | PlotIntegerFlag; labelOverride?: string }) {
  return (
    <div className="question">
      <div>
        <h3 className="question-text">{labelOverride ?? flag.description}</h3>
        <div className="tag-line">{flag.category}</div>
      </div>
      <span className="question-id">
        {flag.name} ({flag.id})
      </span>
    </div>
  );
}

function PlotCardFooter({ meta, modified }: PlotCardFooterProps) {
  return (
    <div className="footer-row">
      <span>{meta}</span>
      {modified ? (
        <span className="modified-pip">
          <span className="dot" aria-hidden="true" />
          modified
        </span>
      ) : (
        <span>unchanged</span>
      )}
    </div>
  );
}

// ─── countModifiedFlags ───────────────────────────────────────────────────────

function countModifiedFlags(state: PlotFlagsPanelState): number {
  let count = 0;
  for (const section of PLOT_SECTIONS) {
    for (const item of section.items) {
      if (item.kind === "boolean") {
        if (Boolean(state.plotBooleanDrafts[item.id]) !== Boolean(state.plotBooleanValues[item.id])) count++;
      } else if (item.kind === "integer") {
        if (state.plotIntegerDrafts[item.id] !== state.plotIntegerValues[item.id]) count++;
      } else if (item.kind === "exclusive") {
        // Count as 1 modified if any flag in group changed
        const changed = item.def.flagIds.some(
          (id) => Boolean(state.plotBooleanDrafts[id]) !== Boolean(state.plotBooleanValues[id]),
        );
        if (changed) count++;
      }
    }
  }
  return count;
}

// Suppress "unused" warning — the set is used by external consumers if needed
void EXCLUSIVE_FLAG_IDS;
