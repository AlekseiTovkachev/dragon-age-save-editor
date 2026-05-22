import { useCallback, useMemo, useState } from "react";
import { executeCommand, expectResult } from "../../api";
import { useDraftCheckpoint } from "../../hooks/useDraftCheckpoint";
import {
  groupedPlotBooleans,
  groupedPlotIntegers,
  plotBooleanValueMap,
  plotIntegerValueMap,
} from "../../lib/plotFlagUtils";
import type { PlotBooleanFlag, PlotIntegerFlag, SaveCommand } from "../../types";
import { validatePlotFlags } from "./plotFlagValidation";

type PlotFlagDraftCheckpoint = {
  plotBooleanDrafts: Record<number, boolean>;
  plotIntegerDrafts: Record<number, number>;
};

type PlotFlagCommandPlan = {
  batch: SaveCommand[];
};

const clonePlotFlagCheckpoint = (draft: PlotFlagDraftCheckpoint): PlotFlagDraftCheckpoint => ({
  plotBooleanDrafts: { ...draft.plotBooleanDrafts },
  plotIntegerDrafts: { ...draft.plotIntegerDrafts },
});

export function usePlotFlagsEditor() {
  const [plotBooleanValues, setPlotBooleanValues] = useState<Record<number, boolean>>({});
  const [plotBooleanDrafts, setPlotBooleanDrafts] = useState<Record<number, boolean>>({});
  const [plotIntegerValues, setPlotIntegerValues] = useState<Record<number, number>>({});
  const [plotIntegerDrafts, setPlotIntegerDrafts] = useState<Record<number, number>>({});
  const [availablePlotBooleans, setAvailablePlotBooleans] = useState<PlotBooleanFlag[]>([]);
  const [availablePlotIntegers, setAvailablePlotIntegers] = useState<PlotIntegerFlag[]>([]);
  const draftCheckpoint = useDraftCheckpoint<PlotFlagDraftCheckpoint>({ clone: clonePlotFlagCheckpoint });
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
    setPlotBooleanDrafts((current) => ({ ...current, [id]: value }));
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

  const checkpointDrafts = useCallback(() => {
    draftCheckpoint.checkpoint({
      plotBooleanDrafts: { ...plotBooleanDrafts },
      plotIntegerDrafts: { ...plotIntegerDrafts },
    });
  }, [draftCheckpoint, plotBooleanDrafts, plotIntegerDrafts]);

  const markDraftsCommitted = useCallback(() => {
    checkpointDrafts();
  }, [checkpointDrafts]);

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

  const modifiedCount = useMemo(() => {
    const boolModified = Object.keys(plotBooleanDrafts).filter(
      (k) => plotBooleanDrafts[Number(k)] !== plotBooleanValues[Number(k)],
    ).length;
    const intModified = Object.keys(plotIntegerDrafts).filter(
      (k) => plotIntegerDrafts[Number(k)] !== plotIntegerValues[Number(k)],
    ).length;
    return boolModified + intModified;
  }, [plotBooleanDrafts, plotBooleanValues, plotIntegerDrafts, plotIntegerValues]);

  const planCommands = useCallback((): PlotFlagCommandPlan => {
    if (modifiedCount === 0) {
      return { batch: [] };
    }
    return {
      batch: [{
        command: "patch_plot_flags",
        booleans: availablePlotBooleans
          .filter((flag) => plotBooleanValues[flag.id] !== undefined || Boolean(plotBooleanDrafts[flag.id]))
          .map((flag) => ({
            id: flag.id,
            value: Boolean(plotBooleanDrafts[flag.id]),
          })),
        integers: availablePlotIntegers
          .filter((flag) => plotIntegerDrafts[flag.id] !== undefined)
          .map((flag) => ({
            id: flag.id,
            value: plotIntegerDrafts[flag.id],
          })),
      }],
    };
  }, [
    availablePlotBooleans,
    availablePlotIntegers,
    modifiedCount,
    plotBooleanDrafts,
    plotBooleanValues,
    plotIntegerDrafts,
  ]);

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
    planCommands,
    markDraftsCommitted,
    resetToCommittedDrafts,
    clear,
    hasPlotWarnings,
    modifiedCount,
  };
}
