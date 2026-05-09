const ORIGIN_GROUP = [2000, 2001, 2002, 2003, 2004, 2005];

type ImplicationRule = {
  triggerId: number;
  setBooleans: Array<[number, boolean]>;
  setIntegers: Array<[number, number]>;
  clearOriginGroup: boolean;
};

const IMPLICATIONS: ImplicationRule[] = [
  { triggerId: 2026, setBooleans: [[2005, true]], setIntegers: [[1000, 2], [1001, 3]], clearOriginGroup: true },
  { triggerId: 2024, setBooleans: [[2005, true]], setIntegers: [[1000, 1], [1001, 3]], clearOriginGroup: true },
  { triggerId: 2053, setBooleans: [[2038, true]], setIntegers: [], clearOriginGroup: false },
  { triggerId: 2054, setBooleans: [[2039, true]], setIntegers: [], clearOriginGroup: false },
  { triggerId: 2055, setBooleans: [[2038, true], [2039, true]], setIntegers: [], clearOriginGroup: false },
  { triggerId: 2042, setBooleans: [[2038, true]], setIntegers: [], clearOriginGroup: false },
  { triggerId: 2048, setBooleans: [[2039, true]], setIntegers: [], clearOriginGroup: false },
];

export function applyImplications(
  bools: Record<number, boolean>,
  ints: Record<number, number>,
): { bools: Record<number, boolean>; ints: Record<number, number> } {
  let resultBools = { ...bools };
  let resultInts = { ...ints };

  for (const rule of IMPLICATIONS) {
    if (!resultBools[rule.triggerId]) continue;
    for (const [id, value] of rule.setBooleans) {
      resultBools[id] = value;
    }
    for (const [id, value] of rule.setIntegers) {
      resultInts[id] = value;
    }
    if (rule.clearOriginGroup) {
      const keepIds = new Set(
        rule.setBooleans.filter(([, v]) => v).map(([id]) => id),
      );
      for (const id of ORIGIN_GROUP) {
        if (!keepIds.has(id)) resultBools[id] = false;
      }
    }
  }

  return { bools: resultBools, ints: resultInts };
}
