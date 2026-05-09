import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { executeCommand, expectResult } from "../../api";
import { useDraftCheckpoint } from "../../hooks/useDraftCheckpoint";
import {
  groupedPlotBooleans,
  groupedPlotIntegers,
  plotBooleanValueMap,
  plotIntegerValueMap,
} from "../../lib/plotFlagUtils";
import type { PlotBooleanFlag, PlotIntegerFlag } from "../../types";
import type { AsyncRun } from "../shared/types";
import { applyImplications } from "./plotFlagImplications";
import { validatePlotFlags } from "./plotFlagValidation";

type UsePlotFlagsEditorOptions = {
  run: AsyncRun;
  refreshSummary: () => Promise<unknown>;
};

type PlotFlagDraftCheckpoint = {
  plotBooleanDrafts: Record<number, boolean>;
  plotIntegerDrafts: Record<number, number>;
};

const clonePlotFlagCheckpoint = (draft: PlotFlagDraftCheckpoint): PlotFlagDraftCheckpoint => ({
  plotBooleanDrafts: { ...draft.plotBooleanDrafts },
  plotIntegerDrafts: { ...draft.plotIntegerDrafts },
});

export function usePlotFlagsEditor({ run, refreshSummary }: UsePlotFlagsEditorOptions) {
  const [plotBooleanValues, setPlotBooleanValues] = useState<Record<number, boolean>>({});
  const [plotBooleanDrafts, setPlotBooleanDrafts] = useState<Record<number, boolean>>({});
  const [plotIntegerValues, setPlotIntegerValues] = useState<Record<number, number>>({});
  const [plotIntegerDrafts, setPlotIntegerDrafts] = useState<Record<number, number>>({});
  const [availablePlotBooleans, setAvailablePlotBooleans] = useState<PlotBooleanFlag[]>([]);
  const [availablePlotIntegers, setAvailablePlotIntegers] = useState<PlotIntegerFlag[]>([]);
  const draftCheckpoint = useDraftCheckpoint<PlotFlagDraftCheckpoint>({ clone: clonePlotFlagCheckpoint });
  const plotBooleanDraftsRef = useRef(plotBooleanDrafts);
  const plotIntegerDraftsRef = useRef(plotIntegerDrafts);

  useEffect(() => {
    plotBooleanDraftsRef.current = plotBooleanDrafts;
    plotIntegerDraftsRef.current = plotIntegerDrafts;
  }, [plotBooleanDrafts, plotIntegerDrafts]);

  const refreshPlotFlags = useCallback(async () => {
    const response = expectResult(await executeCommand({ command: "list_plot_flags" }), "plot_flags");
    const booleanValues = plotBooleanValueMap(response.booleans);
    const integerValues = plotIntegerValueMap(response.integers);
    setPlotBooleanValues(booleanValues);
    setPlotBooleanDrafts(booleanValues);
    setPlotIntegerValues(integerValues);
    setPlotIntegerDrafts(integerValues);
    draftCheckpoint.checkpoint({
      plotBooleanDrafts: { ...booleanValues },
      plotIntegerDrafts: { ...integerValues },
    });
  }, [draftCheckpoint]);

  const refreshAvailablePlotFlags = useCallback(async () => {
    const response = expectResult(await executeCommand({ command: "list_available_plot_flags" }), "available_plot_flags");
    setAvailablePlotBooleans(response.booleans);
    setAvailablePlotIntegers(response.integers);
  }, []);

  const handleBooleanToggle = useCallback((id: number, value: boolean) => {
    const merged = applyImplications(
      { ...plotBooleanDraftsRef.current, [id]: value },
      plotIntegerDraftsRef.current,
    );
    setPlotBooleanDrafts(merged.bools);
    setPlotIntegerDrafts(merged.ints);
  }, []);

  const handleIntegerChange = useCallback((id: number, value: number) => {
    setPlotIntegerDrafts((current) => ({ ...current, [id]: value }));
  }, []);

  const handleBooleanBatch = useCallback(
    (boolChanges: Record<number, boolean>, intChanges: Record<number, number> = {}) => {
      setPlotBooleanDrafts((current) => ({ ...current, ...boolChanges }));
      if (Object.keys(intChanges).length > 0) {
        setPlotIntegerDrafts((current) => ({ ...current, ...intChanges }));
      }
    },
    [],
  );

  const commitPlotFlagDrafts = useCallback(async () => {
    return run(async () => {
      const response = expectResult(
        await executeCommand({
          command: "patch_plot_flags",
          booleans: availablePlotBooleans
            .filter((flag) => plotBooleanValues[flag.id] !== undefined || Boolean(plotBooleanDraftsRef.current[flag.id]))
            .map((flag) => ({
              id: flag.id,
              value: Boolean(plotBooleanDraftsRef.current[flag.id]),
            })),
          integers: availablePlotIntegers
            .filter((flag) => plotIntegerDraftsRef.current[flag.id] !== undefined)
            .map((flag) => ({
              id: flag.id,
              value: plotIntegerDraftsRef.current[flag.id],
            })),
        }),
        "plot_flags",
      );
      const booleanValues = plotBooleanValueMap(response.booleans);
      const integerValues = plotIntegerValueMap(response.integers);
      setPlotBooleanValues(booleanValues);
      setPlotBooleanDrafts(booleanValues);
      setPlotIntegerValues(integerValues);
      setPlotIntegerDrafts(integerValues);
      await refreshSummary();
    });
  }, [
    availablePlotBooleans,
    availablePlotIntegers,
    plotBooleanValues,
    refreshSummary,
    run,
  ]);

  const resetLoadedDrafts = useCallback(() => {
    setPlotBooleanDrafts(plotBooleanValues);
    setPlotIntegerDrafts(plotIntegerValues);
  }, [plotBooleanValues, plotIntegerValues]);

  const checkpointDrafts = useCallback(() => {
    draftCheckpoint.checkpoint({
      plotBooleanDrafts: { ...plotBooleanDrafts },
      plotIntegerDrafts: { ...plotIntegerDrafts },
    });
  }, [draftCheckpoint, plotBooleanDrafts, plotIntegerDrafts]);

  const commitDrafts = useCallback(async () => {
    if (!await commitPlotFlagDrafts()) {
      return false;
    }
    checkpointDrafts();
    return true;
  }, [checkpointDrafts, commitPlotFlagDrafts]);

  const resetToCommittedDrafts = useCallback(() => {
    const checkpoint = draftCheckpoint.reset();
    if (!checkpoint) {
      return;
    }
    setPlotBooleanDrafts(checkpoint.plotBooleanDrafts);
    setPlotIntegerDrafts(checkpoint.plotIntegerDrafts);
  }, [draftCheckpoint]);

  const clear = useCallback(() => {
    setPlotBooleanValues({});
    setPlotBooleanDrafts({});
    setPlotIntegerValues({});
    setPlotIntegerDrafts({});
    setAvailablePlotBooleans([]);
    setAvailablePlotIntegers([]);
    draftCheckpoint.clear();
  }, [draftCheckpoint]);

  const groupedBooleanFlags = useMemo(
    () => groupedPlotBooleans(availablePlotBooleans),
    [availablePlotBooleans],
  );
  const groupedIntegerFlags = useMemo(
    () => groupedPlotIntegers(availablePlotIntegers),
    [availablePlotIntegers],
  );

  const hasPlotWarnings = useMemo(
    () => validatePlotFlags(plotBooleanDrafts, plotIntegerDrafts).length > 0,
    [plotBooleanDrafts, plotIntegerDrafts],
  );

  return {
    plotBooleanValues,
    plotBooleanDrafts,
    plotIntegerValues,
    plotIntegerDrafts,
    groupedPlotBooleans: groupedBooleanFlags,
    groupedPlotIntegers: groupedIntegerFlags,
    refreshPlotFlags,
    refreshAvailablePlotFlags,
    handleBooleanToggle,
    handleIntegerChange,
    handleBooleanBatch,
    commitPlotFlagDrafts,
    resetLoadedDrafts,
    commitDrafts,
    resetToCommittedDrafts,
    clear,
    hasPlotWarnings,
  };
}
