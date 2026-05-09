import { describe, expect, it } from "vitest";
import { applyImplications } from "./plotFlagImplications";

describe("applyImplications", () => {
  it("Alistair+Warden marriage sets female identity and clears other origins", () => {
    const { bools, ints } = applyImplications(
      { 2026: true, 2000: true, 2001: true },
      {},
    );
    // Sets Human Noble origin
    expect(bools[2005]).toBe(true);
    // Clears other origins
    expect(bools[2000]).toBe(false);
    expect(bools[2001]).toBe(false);
    // Sets gender=Female(2) and race=Human(3)
    expect(ints[1000]).toBe(2);
    expect(ints[1001]).toBe(3);
  });

  it("Anora+Warden marriage sets male identity and clears other origins", () => {
    const { bools, ints } = applyImplications(
      { 2024: true, 2002: true },
      {},
    );
    expect(bools[2005]).toBe(true);
    expect(bools[2002]).toBe(false);
    expect(ints[1000]).toBe(1);
    expect(ints[1001]).toBe(3);
  });

  it("Isabela+Leliana encounter sets Leliana recruited", () => {
    const { bools } = applyImplications({ 2053: true }, {});
    expect(bools[2038]).toBe(true);
    // Zevran should NOT be set
    expect(bools[2039]).toBeUndefined();
  });

  it("Isabela+Zevran encounter sets Zevran recruited", () => {
    const { bools } = applyImplications({ 2054: true }, {});
    expect(bools[2039]).toBe(true);
    expect(bools[2038]).toBeUndefined();
  });

  it("Isabela foursome sets both Leliana and Zevran recruited", () => {
    const { bools } = applyImplications({ 2055: true }, {});
    expect(bools[2038]).toBe(true);
    expect(bools[2039]).toBe(true);
  });

  it("Leliana romance implies she is recruited", () => {
    const { bools } = applyImplications({ 2042: true }, {});
    expect(bools[2038]).toBe(true);
  });

  it("Zevran romance implies he is recruited", () => {
    const { bools } = applyImplications({ 2048: true }, {});
    expect(bools[2039]).toBe(true);
  });

  it("unrelated flags are not touched by implications", () => {
    const input = { 9999: true, 1234: false };
    const { bools } = applyImplications(input, {});
    expect(bools[9999]).toBe(true);
    expect(bools[1234]).toBe(false);
    // No implications fire, so only the two input flags exist at keys not triggered
    expect(bools[2038]).toBeUndefined();
    expect(bools[2039]).toBeUndefined();
  });

  it("no implications fire when no trigger flags are set", () => {
    const { bools, ints } = applyImplications({}, {});
    expect(Object.keys(bools)).toHaveLength(0);
    expect(Object.keys(ints)).toHaveLength(0);
  });
});
