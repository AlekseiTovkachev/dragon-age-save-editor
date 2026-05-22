export type PlotWarning = {
  section: string;
  message: string;
};

export function validatePlotFlags(
  bools: Record<number, boolean>,
  ints: Record<number, number>,
): PlotWarning[] {
  const warnings: PlotWarning[] = [];
  const b = (id: number) => Boolean(bools[id]);
  const iv = (id: number) => ints[id] ?? 0;

  // Identity — origin
  const originCount = [2000, 2001, 2002, 2003, 2004, 2005].filter(b).length;
  if (originCount > 1) {
    warnings.push({ section: "Warden", message: "Multiple origins are active — only one should be set." });
  }
  if (b(2000) && ![2, 3].includes(iv(1001))) {
    warnings.push({ section: "Warden", message: "Circle Mage origin requires an elf or human Warden race." });
  }
  if ((b(2001) || b(2002)) && iv(1001) !== 1) {
    warnings.push({ section: "Warden", message: "Dwarf origins require Warden race to be Dwarf." });
  }
  if ((b(2003) || b(2004)) && iv(1001) !== 2) {
    warnings.push({ section: "Warden", message: "Elf origins require Warden race to be Elf." });
  }
  if (b(2005) && iv(1001) !== 3) {
    warnings.push({ section: "Warden", message: "Human Noble origin requires Warden race to be Human." });
  }

  // Identity — political marriages require matching identity
  if (b(2026) && !(iv(1000) === 2 && iv(1001) === 3 && b(2005))) {
    warnings.push({ section: "Landsmeet", message: "Alistair + Warden marriage will force a female human noble Warden (gender=Female, race=Human, origin=Human Noble)." });
  }
  if (b(2024) && !(iv(1000) === 1 && iv(1001) === 3 && b(2005))) {
    warnings.push({ section: "Landsmeet", message: "Anora + Warden marriage will force a male human noble Warden (gender=Male, race=Human, origin=Human Noble)." });
  }

  // Landsmeet contradictions
  const alistairKing = b(2020) || b(2021) || b(2026);
  if (alistairKing && b(2022)) {
    warnings.push({ section: "Landsmeet", message: "Alistair cannot be both king and exiled." });
  }
  if (alistairKing && b(2023)) {
    warnings.push({ section: "Landsmeet", message: "Alistair cannot be both king and executed." });
  }
  if (b(2022) && b(2023)) {
    warnings.push({ section: "Landsmeet", message: "Alistair cannot be both exiled and executed." });
  }
  if (b(2025) && b(2097)) {
    warnings.push({ section: "Landsmeet", message: "Loghain cannot be both killed and alive." });
  }

  // Archdemon killer
  const archdemons = [2028, 2029, 2030].filter(b).length;
  if (archdemons > 1) {
    warnings.push({ section: "Finale", message: "More than one Archdemon killer is set — only one should be active." });
  }
  if (archdemons === 0) {
    warnings.push({ section: "Finale", message: "No Archdemon killer is set." });
  }

  // Ritual / ultimate sacrifice
  if (!b(2104)) {
    if (b(2029) && b(2097)) {
      warnings.push({ section: "Landsmeet", message: "No ritual — Loghain killed the Archdemon and should be dead, but is also marked alive." });
    }
  }
  if (b(2104) && b(2029) && !b(2097)) {
    warnings.push({ section: "Landsmeet", message: "Loghain killed the Archdemon (with ritual) — he should be marked as living." });
  }

  // Companions — Leliana
  if (b(2038) && b(2045)) {
    warnings.push({ section: "Leliana", message: "Leliana is marked as both recruited/stayed and attacked the Warden." });
  }
  if (b(2038) && b(2044)) {
    warnings.push({ section: "Leliana", message: "Leliana cannot be both recruited/stayed and not recruited." });
  }
  if (b(2042) && !b(2038)) {
    warnings.push({ section: "Leliana", message: "Leliana romance is active but she is not marked as recruited/stayed." });
  }

  // Companions — Zevran
  const zevranGone = [2050, 2051, 2105, 2106, 2107, 2108].some(b);
  if (b(2039) && zevranGone) {
    warnings.push({ section: "Zevran", message: "Zevran is marked as recruited/stayed but also has a dead, left, or hostile flag set." });
  }
  if (b(2048) && !b(2039)) {
    warnings.push({ section: "Zevran", message: "Zevran romance is active but he is not marked as recruited/stayed." });
  }

  // Isabela
  if ((b(2053) || b(2055)) && !b(2038)) {
    warnings.push({ section: "Isabela", message: "Isabela + Leliana encounter requires Leliana to have been recruited and stayed." });
  }
  if ((b(2054) || b(2055)) && !b(2039)) {
    warnings.push({ section: "Isabela", message: "Isabela + Zevran encounter requires Zevran to have been recruited and stayed." });
  }

  // Warden's Keep
  if (b(2070) && b(2071)) {
    warnings.push({ section: "Warden's Keep (DLC)", message: "Avernus cannot have both ethical and evil research active simultaneously." });
  }
  if (b(2068) && (b(2070) || b(2071))) {
    warnings.push({ section: "Warden's Keep (DLC)", message: "Avernus research flags are set but Avernus was killed." });
  }
  if (!b(2094) && (b(2067) || b(2068) || b(2070) || b(2071))) {
    warnings.push({ section: "Warden's Keep (DLC)", message: "Warden's Keep was not started but Sophia/Avernus outcome flags are set." });
  }

  // Awakening
  if (b(2063) && b(2096)) {
    warnings.push({ section: "Vigil's Keep", message: "The Architect cannot be both killed and spared." });
  }
  if (b(2065) && b(2084)) {
    warnings.push({ section: "Nathaniel", message: "Nathaniel is marked as both recruited/stayed and died at the Vigil's Keep siege." });
  }
  if (b(2064) && b(2066)) {
    warnings.push({ section: "Anders", message: "Anders is marked as both recruited/stayed and died at the Vigil's Keep siege." });
  }

  return warnings;
}
