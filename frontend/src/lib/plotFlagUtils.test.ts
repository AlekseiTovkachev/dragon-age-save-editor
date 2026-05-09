import { describe, expect, it } from "vitest";
import { groupedPlotBooleans, plotBooleanValueMap, plotIntegerValueMap } from "./plotFlagUtils";
import type { PlotBooleanFlag } from "../types";

describe("plotFlagUtils", () => {
  it("groups flags by category", () => {
    const flags: PlotBooleanFlag[] = [
      { id: 1, name: "a", description: "A", category: "Act 1" },
      { id: 2, name: "b", description: "B", category: "Act 1" },
      { id: 3, name: "c", description: "C", category: "Act 2" },
    ];

    expect(groupedPlotBooleans(flags).map((group) => [group.category, group.flags.length])).toEqual([
      ["Act 1", 2],
      ["Act 2", 1],
    ]);
  });

  it("maps backend value arrays into draft records", () => {
    expect(plotBooleanValueMap([{ id: 4, value: true }])).toEqual({ 4: true });
    expect(plotIntegerValueMap([{ id: 5, value: 2 }])).toEqual({ 5: 2 });
  });
});
