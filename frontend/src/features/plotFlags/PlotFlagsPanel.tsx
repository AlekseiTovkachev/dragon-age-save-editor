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
  plotIntegerDrafts: Record<number, number>;
  plotBooleanDrafts: Record<number, boolean>;
  groupedPlotIntegers: PlotIntegerGroup[];
  groupedPlotBooleans: PlotBooleanGroup[];
};

export type PlotFlagsPanelActions = {
  handleIntegerChange: (id: number, value: number) => void;
  handleBooleanToggle: (id: number, value: boolean) => void;
};

export function PlotFlagsPanel({ state, actions, summary, canEdit, busy }: PlotFlagsPanelProps) {
  return (
    <Panel className="detail-panel" title="DA2 Plot Flags" scroll>
      <PanelBody>
        {summary.preferred_game === "da2" ? (
          <div className="plot-flags-layout">
            <div className="plot-choice-list">
              <h3>Choices</h3>
              {state.groupedPlotIntegers.map((group) => (
                <div key={`plot-int-group-${group.category}`} className="plot-group">
                  <h4>{group.category}</h4>
                  {group.flags.map((flag) => (
                    <fieldset key={`plot-int-${flag.id}`} className="plot-radio-group">
                      <legend>{flag.description}</legend>
                      {flag.options.map((option) => (
                        <label key={`plot-int-${flag.id}-${option.value}`} className="radio-row">
                          <input
                            type="radio"
                            name={`plot-int-${flag.id}`}
                            checked={state.plotIntegerDrafts[flag.id] === option.value}
                            onChange={() => actions.handleIntegerChange(flag.id, option.value)}
                            disabled={!canEdit || busy}
                          />
                          <span>{option.label}</span>
                        </label>
                      ))}
                      <span className="plot-flag-code">{flag.name} ({flag.id})</span>
                    </fieldset>
                  ))}
                </div>
              ))}
            </div>
            <div className="plot-boolean-list">
              <h3>Booleans</h3>
              {state.groupedPlotBooleans.map((group) => (
                <div key={`plot-bool-group-${group.category}`} className="plot-group">
                  <h4>{group.category}</h4>
                  {group.flags.map((flag) => (
                    <label key={`plot-bool-${flag.id}`} className="check-row">
                      <input
                        type="checkbox"
                        checked={Boolean(state.plotBooleanDrafts[flag.id])}
                        onChange={(event) => actions.handleBooleanToggle(flag.id, event.target.checked)}
                        disabled={!canEdit || busy}
                      />
                      <span>
                        {flag.description}
                        <small className="plot-flag-code">{flag.name} ({flag.id})</small>
                      </span>
                    </label>
                  ))}
                </div>
              ))}
            </div>
          </div>
        ) : (
          <EmptyState>Plot flag editing is available for DA2 saves.</EmptyState>
        )}
      </PanelBody>
    </Panel>
  );
}
