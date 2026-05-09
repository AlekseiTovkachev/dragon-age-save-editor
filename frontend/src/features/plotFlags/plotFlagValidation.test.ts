import { describe, expect, it } from "vitest";
import { validatePlotFlags } from "./plotFlagValidation";

function bools(pairs: Array<[number, boolean]>): Record<number, boolean> {
  return Object.fromEntries(pairs);
}

function ints(pairs: Array<[number, number]>): Record<number, number> {
  return Object.fromEntries(pairs);
}

function warnsIn(warnings: ReturnType<typeof validatePlotFlags>, section: string, fragment: string): boolean {
  return warnings.some((w) => w.section === section && w.message.includes(fragment));
}

describe("validatePlotFlags", () => {
  it("tc01: multiple origins", () => {
    const w = validatePlotFlags(bools([[2000, true], [2001, true]]), ints([]));
    expect(warnsIn(w, "Warden", "Multiple origins")).toBe(true);
  });

  it("tc02: single origin — no warning", () => {
    const w = validatePlotFlags(bools([[2000, true]]), ints([]));
    expect(warnsIn(w, "Warden", "Multiple origins")).toBe(false);
  });

  it("tc03: Alistair+Warden marriage with wrong identity", () => {
    const w = validatePlotFlags(bools([[2026, true]]), ints([[1000, 1], [1001, 3]]));
    expect(warnsIn(w, "Landsmeet", "female human noble")).toBe(true);
  });

  it("tc04: Alistair+Warden marriage with correct identity — no warning", () => {
    const w = validatePlotFlags(bools([[2026, true], [2005, true]]), ints([[1000, 2], [1001, 3]]));
    expect(warnsIn(w, "Landsmeet", "female human noble")).toBe(false);
  });

  it("tc05: Alistair king and exiled", () => {
    const w = validatePlotFlags(bools([[2021, true], [2022, true]]), ints([]));
    expect(warnsIn(w, "Landsmeet", "exiled")).toBe(true);
  });

  it("tc06: Loghain killed and alive", () => {
    const w = validatePlotFlags(bools([[2025, true], [2097, true]]), ints([]));
    expect(warnsIn(w, "Landsmeet", "killed and alive")).toBe(true);
  });

  it("tc07: no archdemon killer", () => {
    const w = validatePlotFlags(bools([]), ints([]));
    expect(warnsIn(w, "Finale", "No Archdemon killer")).toBe(true);
  });

  it("tc08: multiple archdemon killers", () => {
    const w = validatePlotFlags(bools([[2028, true], [2030, true]]), ints([]));
    expect(warnsIn(w, "Finale", "More than one")).toBe(true);
  });

  it("tc09: Loghain kills Archdemon, no ritual, marked alive", () => {
    const w = validatePlotFlags(bools([[2029, true], [2097, true]]), ints([]));
    expect(warnsIn(w, "Landsmeet", "should be dead")).toBe(true);
  });

  it("tc10: Loghain kills Archdemon with ritual, not marked alive", () => {
    const w = validatePlotFlags(bools([[2029, true], [2104, true]]), ints([]));
    expect(warnsIn(w, "Landsmeet", "should be marked as living")).toBe(true);
  });

  it("tc11: Leliana romance but not recruited", () => {
    const w = validatePlotFlags(bools([[2042, true]]), ints([]));
    expect(warnsIn(w, "Leliana", "not marked as recruited")).toBe(true);
  });

  it("tc12: Zevran romance but not recruited", () => {
    const w = validatePlotFlags(bools([[2048, true]]), ints([]));
    expect(warnsIn(w, "Zevran", "not marked as recruited")).toBe(true);
  });

  it("tc13: Zevran recruited and hostile", () => {
    const w = validatePlotFlags(bools([[2039, true], [2050, true]]), ints([]));
    expect(warnsIn(w, "Zevran", "hostile")).toBe(true);
  });

  it("tc14: Isabela+Leliana threesome but Leliana not recruited", () => {
    const w = validatePlotFlags(bools([[2053, true]]), ints([]));
    expect(warnsIn(w, "Isabela", "Leliana")).toBe(true);
  });

  it("tc15: Isabela foursome but Zevran not recruited", () => {
    const w = validatePlotFlags(bools([[2055, true], [2038, true]]), ints([]));
    expect(warnsIn(w, "Isabela", "Zevran")).toBe(true);
  });

  it("tc16: Avernus both ethical and evil research", () => {
    const w = validatePlotFlags(bools([[2094, true], [2070, true], [2071, true]]), ints([]));
    expect(warnsIn(w, "Warden's Keep (DLC)", "both ethical and evil")).toBe(true);
  });

  it("tc17: Avernus research flags set but Avernus was killed", () => {
    const w = validatePlotFlags(bools([[2094, true], [2068, true], [2071, true]]), ints([]));
    expect(warnsIn(w, "Warden's Keep (DLC)", "Avernus was killed")).toBe(true);
  });

  it("tc18: Warden's Keep outcome flags without DLC started", () => {
    const w = validatePlotFlags(bools([[2067, true]]), ints([]));
    expect(warnsIn(w, "Warden's Keep (DLC)", "not started")).toBe(true);
  });

  it("tc19: Architect killed and spared", () => {
    const w = validatePlotFlags(bools([[2063, true], [2096, true]]), ints([]));
    expect(warnsIn(w, "Vigil's Keep", "killed and spared")).toBe(true);
  });

  it("tc20: Anders recruited and died at siege", () => {
    const w = validatePlotFlags(bools([[2064, true], [2066, true]]), ints([]));
    expect(warnsIn(w, "Anders", "died")).toBe(true);
  });

  it("tc21: canonical valid state — no relevant warnings", () => {
    const w = validatePlotFlags(
      bools([
        [2005, true], [2020, true], [2030, true], [2104, true],
        [2097, true], [2038, true], [2039, true], [2094, true],
        [2067, true], [2071, true],
      ]),
      ints([[1000, 2], [1001, 3]]),
    );
    expect(warnsIn(w, "Finale", "No Archdemon killer")).toBe(false);
    expect(warnsIn(w, "Warden", "Multiple origins")).toBe(false);
    expect(warnsIn(w, "Warden's Keep (DLC)", "")).toBe(false);
  });
});
