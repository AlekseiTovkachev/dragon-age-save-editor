export type Section = "characters" | "inventory" | "recipes" | "plot_flags";
export type CharacterTab = "overview" | "abilities" | "equipment";

export const SECTIONS: Section[] = ["characters", "inventory", "recipes", "plot_flags"];
export const SECTION_TITLES: Record<Section, string> = {
  characters: "Characters",
  inventory: "Inventory",
  recipes: "Recipes",
  plot_flags: "Plot Flags",
};

export const CHARACTER_TABS: CharacterTab[] = ["overview", "abilities", "equipment"];
export const CHARACTER_TAB_TITLES: Record<CharacterTab, string> = {
  overview: "Overview",
  abilities: "Abilities",
  equipment: "Equipment",
};
