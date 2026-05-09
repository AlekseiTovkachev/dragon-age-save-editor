import type { PlotBooleanFlag, PlotBooleanValue, PlotIntegerFlag, PlotIntegerValue } from "../types";

export type PlotFlagGroup<T> = {
  category: string;
  flags: T[];
};

export function groupedPlotBooleans(flags: PlotBooleanFlag[]): PlotFlagGroup<PlotBooleanFlag>[] {
  return groupedPlotFlags(flags);
}

export function groupedPlotIntegers(flags: PlotIntegerFlag[]): PlotFlagGroup<PlotIntegerFlag>[] {
  return groupedPlotFlags(flags);
}

export function plotBooleanValueMap(values: PlotBooleanValue[]): Record<number, boolean> {
  return Object.fromEntries(values.map((entry) => [entry.id, entry.value]));
}

export function plotIntegerValueMap(values: PlotIntegerValue[]): Record<number, number> {
  return Object.fromEntries(values.map((entry) => [entry.id, entry.value]));
}

function groupedPlotFlags<T extends { category: string }>(flags: T[]): PlotFlagGroup<T>[] {
  const groups = new Map<string, T[]>();
  for (const flag of flags) {
    groups.set(flag.category, [...(groups.get(flag.category) ?? []), flag]);
  }
  return Array.from(groups, ([category, entries]) => ({ category, flags: entries }));
}
