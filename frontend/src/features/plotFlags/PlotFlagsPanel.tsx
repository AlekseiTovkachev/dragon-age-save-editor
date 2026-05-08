import { useMemo, useState } from "react";
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
};

type SearchablePlotFlag = {
  id: number;
  name: string;
  description: string;
  category: string;
  kind: "choice" | "boolean";
  flag: PlotIntegerFlag | PlotBooleanFlag;
};

const ALL_CATEGORIES = "All";

export function PlotFlagsPanel({ state, actions, summary, canEdit, busy }: PlotFlagsPanelProps) {
  const [search, setSearch] = useState("");
  const [category, setCategory] = useState(ALL_CATEGORIES);
  const disabled = !canEdit || busy;

  const flags = useMemo(() => flattenPlotFlags(state), [state]);
  const categories = useMemo(() => uniqueCategories(flags), [flags]);
  const filteredFlags = useMemo(
    () => filterPlotFlags(flags, category, search),
    [category, flags, search],
  );
  const modifiedCount = useMemo(() => countModifiedFlags(state), [state]);

  if (summary.preferred_game !== "da2") {
    return (
      <Panel className="detail-panel" title="DA2 Plot Flags" scroll>
        <PanelBody>
          <EmptyState>Plot flag editing is available for DA2 saves.</EmptyState>
        </PanelBody>
      </Panel>
    );
  }

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
          {modifiedCount} of {flags.length} modified
        </span>
      }
      scroll
    >
      <PanelBody>
        <div className="plot-flags-stack">
          <DaoBanner />

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

          {filteredFlags.length > 0 ? (
            <div className="plot-grid">
              {filteredFlags.map((entry) =>
                entry.kind === "choice" ? (
                  <PlotChoiceCard
                    key={`plot-int-${entry.id}`}
                    flag={entry.flag as PlotIntegerFlag}
                    committedValue={state.plotIntegerValues[entry.id]}
                    draftValue={state.plotIntegerDrafts[entry.id]}
                    disabled={disabled}
                    onChange={actions.handleIntegerChange}
                  />
                ) : (
                  <PlotBooleanCard
                    key={`plot-bool-${entry.id}`}
                    flag={entry.flag as PlotBooleanFlag}
                    committedValue={state.plotBooleanValues[entry.id]}
                    draftValue={state.plotBooleanDrafts[entry.id]}
                    disabled={disabled}
                    onToggle={actions.handleBooleanToggle}
                  />
                ),
              )}
            </div>
          ) : (
            <div className="plot-empty">No plot flags match the current filters.</div>
          )}
        </div>
      </PanelBody>
    </Panel>
  );
}

function DaoBanner() {
  return (
    <div className="dao-banner">
      <span className="icon" aria-hidden="true">!</span>
      <div>
        <strong>These are imported from DAO.</strong>{" "}
        <span>
          DA2 reads the Warden&apos;s choices from your DAO save — edit them here to change how the world remembers
          you. Hawke&apos;s own decisions stay in DA2 dialogue/quest data, not these flags.
        </span>
      </div>
    </div>
  );
}

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

function PlotBooleanCard({ flag, committedValue, draftValue, disabled, onToggle }: PlotBooleanCardProps) {
  const selectedValue = Boolean(draftValue);
  const modified = Boolean(draftValue) !== Boolean(committedValue);

  return (
    <article className={["plot-card", "plot-card-boolean", modified ? "is-modified" : ""].filter(Boolean).join(" ")}>
      <PlotQuestion flag={flag} />
      <div className="plot-options plot-boolean-options" role="radiogroup" aria-label={flag.description}>
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

function PlotQuestion({ flag }: { flag: PlotBooleanFlag | PlotIntegerFlag }) {
  return (
    <div className="question">
      <div>
        <h3 className="question-text">{flag.description}</h3>
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

function flattenPlotFlags(state: PlotFlagsPanelState): SearchablePlotFlag[] {
  return [
    ...state.groupedPlotIntegers.flatMap((group) =>
      group.flags.map((flag) => ({
        id: flag.id,
        name: flag.name,
        description: flag.description,
        category: group.category,
        kind: "choice" as const,
        flag,
      })),
    ),
    ...state.groupedPlotBooleans.flatMap((group) =>
      group.flags.map((flag) => ({
        id: flag.id,
        name: flag.name,
        description: flag.description,
        category: group.category,
        kind: "boolean" as const,
        flag,
      })),
    ),
  ];
}

function uniqueCategories(flags: SearchablePlotFlag[]) {
  const seen = new Set<string>();
  const categories = [ALL_CATEGORIES];
  for (const flag of flags) {
    if (!seen.has(flag.category)) {
      seen.add(flag.category);
      categories.push(flag.category);
    }
  }
  return categories;
}

function filterPlotFlags(flags: SearchablePlotFlag[], category: string, search: string) {
  const query = search.trim().toLowerCase();
  return flags.filter((entry) => {
    if (category !== ALL_CATEGORIES && entry.category !== category) {
      return false;
    }
    if (!query) {
      return true;
    }
    const optionLabels = entry.kind === "choice" ? (entry.flag as PlotIntegerFlag).options.map((option) => option.label) : [];
    return [
      entry.id.toString(),
      entry.name,
      entry.description,
      entry.category,
      ...optionLabels,
    ].some((value) => value.toLowerCase().includes(query));
  });
}

function countModifiedFlags(state: PlotFlagsPanelState) {
  const integerCount = state.groupedPlotIntegers.reduce(
    (total, group) =>
      total + group.flags.filter((flag) => state.plotIntegerDrafts[flag.id] !== state.plotIntegerValues[flag.id]).length,
    0,
  );
  const booleanCount = state.groupedPlotBooleans.reduce(
    (total, group) =>
      total
      + group.flags.filter((flag) => Boolean(state.plotBooleanDrafts[flag.id]) !== Boolean(state.plotBooleanValues[flag.id]))
        .length,
    0,
  );
  return integerCount + booleanCount;
}
