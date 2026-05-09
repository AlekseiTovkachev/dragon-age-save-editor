import type { SaveSummary } from "../types";

export function parseNumber(value: string): number | null {
  if (value.trim() === "") {
    return null;
  }
  const parsed = Number(value);
  return Number.isFinite(parsed) ? parsed : null;
}

export function titleCase(value: string): string {
  return value
    .split("_")
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join(" ");
}

export function gameLabel(value: SaveSummary["preferred_game"]): string {
  switch (value) {
    case "dao":
      return "DAO";
    case "dao_awakening":
      return "DAO Awakening";
    case "da2":
      return "DA2";
    default:
      return "Unknown Game";
  }
}
