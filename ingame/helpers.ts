import { execFileSync } from "child_process";
import { copyFileSync, existsSync, rmSync } from "fs";
import { expect, type Page, type TestInfo } from "@playwright/test";

const DAO_SAVE_ENV = process.env.DAO_SAVE;
const DA2_SAVE_ENV = process.env.DA2_SAVE;
const INGAME_GAME_ENV = process.env.INGAME_GAME;
const NPM_SCRIPT = process.env.npm_lifecycle_event;

function selectedGame(): "dao" | "da2" | null {
  if (INGAME_GAME_ENV === "dao" || INGAME_GAME_ENV === "da2") {
    return INGAME_GAME_ENV;
  }
  if (NPM_SCRIPT === "ingame-test:dao") {
    return "dao";
  }
  if (NPM_SCRIPT === "ingame-test:da2") {
    return "da2";
  }
  return null;
}

function resolveSavePath() {
  const game = selectedGame();

  if (game === "dao") {
    if (!DAO_SAVE_ENV) {
      throw new Error("Set DAO_SAVE env var to run DAO-family in-game tests");
    }
    return DAO_SAVE_ENV;
  }

  if (game === "da2") {
    if (!DA2_SAVE_ENV) {
      throw new Error("Set DA2_SAVE env var to run DA2 in-game tests");
    }
    return DA2_SAVE_ENV;
  }

  if (!DAO_SAVE_ENV && !DA2_SAVE_ENV) {
    throw new Error("Set DAO_SAVE or DA2_SAVE env var to the path of your test save file");
  }
  if (DAO_SAVE_ENV && DA2_SAVE_ENV) {
    throw new Error(
      "Both DAO_SAVE and DA2_SAVE are set. Set INGAME_GAME=dao or INGAME_GAME=da2, or use npm run ingame-test:dao / ingame-test:da2.",
    );
  }
  return (DAO_SAVE_ENV ?? DA2_SAVE_ENV) as string;
}

export const SAVE_PATH = resolveSavePath();
// Back-compat alias for legacy DAO specs. Prefer SAVE_PATH in new code.
export const DAO_SAVE = SAVE_PATH;
const APPLY_EDIT = process.platform === "win32" ? "target/debug/apply_edit.exe" : "target/debug/apply_edit";

type CharacterTarget = "main_character" | { companion: { index: number } };

type Ability = {
  id: number;
  name: string | null;
};

type MaterialOption = {
  code: number;
  tier: number;
  name: string;
};

type Item = {
  name: string | null;
  resref: string | null;
  category: { label: string; value: string };
  stackable: boolean;
  material: number | null;
  material_options: MaterialOption[];
  properties: unknown[];
  item_stacksize: number | null;
};

type IndexedItem = {
  index: number;
  item: Item;
};

type CharacterSummary = {
  target: CharacterTarget;
  name: string;
};

type Character = {
  name: string;
  approval: number | null;
  level: number | null;
  core_stats: Record<string, number>;
  equipment: Item[];
  skills: Ability[];
  talents: Ability[];
  spells: Ability[];
};

type SaveSummary = {
  preferred_game: string;
  money: number;
};

export type SaveSnapshot = {
  summary: SaveSummary;
  characters: CharacterSummary[];
  characterDetails: Map<string, Character>;
  backpackItems: IndexedItem[];
};

export type InGamePrerequisite = {
  label: string;
  check: (snapshot: SaveSnapshot) => boolean;
};

type CharacterKind = "main" | `companion:${string}`;
type AbilityList = "skills" | "talents" | "spells";

function characterKey(summary: CharacterSummary) {
  return summary.target === "main_character" ? "main" : `companion:${summary.name}`;
}

function namedCharacter(snapshot: SaveSnapshot, kind: CharacterKind) {
  return snapshot.characterDetails.get(kind) ?? null;
}

function itemName(item: Item) {
  return item.name ?? item.resref ?? "";
}

function itemsMatchingName(items: Item[], pattern: RegExp) {
  return items.filter((item) => pattern.test(itemName(item)));
}

function hasMaterialOption(item: Item, tier: number, name: string) {
  return item.material_options.some((option) => option.tier === tier && option.name.toLowerCase() === name.toLowerCase());
}

export function readSaveJson<T = unknown>(command: unknown, savePath: string = SAVE_PATH): T {
  const stdout = execFileSync(APPLY_EDIT, [savePath, JSON.stringify(command)], { encoding: "utf8" });
  return JSON.parse(stdout) as T;
}

export function loadSaveSnapshot(savePath: string = SAVE_PATH): SaveSnapshot {
  const summary = readSaveJson<{ summary: SaveSummary }>({ command: "get_summary" }, savePath).summary;
  const characters = readSaveJson<{ characters: CharacterSummary[] }>({ command: "list_characters" }, savePath).characters;
  const characterDetails = new Map<string, Character>();
  for (const character of characters) {
    const detail = readSaveJson<{ character: Character }>({ command: "get_character", target: character.target }, savePath).character;
    characterDetails.set(characterKey(character), detail);
  }
  const backpackItems = readSaveJson<{ items: IndexedItem[] }>({ command: "list_backpack_items" }, savePath).items;
  return { summary, characters, characterDetails, backpackItems };
}

export async function ensurePrerequisites(testInfo: TestInfo, prerequisites: InGamePrerequisite[]) {
  for (const prerequisite of prerequisites) {
    testInfo.annotations.push({ type: "prerequisite", description: prerequisite.label });
  }
  const snapshot = loadSaveSnapshot();
  const missing = prerequisites.filter((prerequisite) => !prerequisite.check(snapshot)).map((prerequisite) => prerequisite.label);
  expect(missing, `Save at SAVE_PATH does not fit this in-game test:\n${missing.map((item) => `- ${item}`).join("\n")}`).toEqual([]);
  return snapshot;
}

export const prereq = {
  daoFamilySave(): InGamePrerequisite {
    return {
      label: "DAO-family save (preferred_game is dao or dao_awakening)",
      check: (snapshot) => ["dao", "dao_awakening"].includes(snapshot.summary.preferred_game),
    };
  },
  da2Save(): InGamePrerequisite {
    return {
      label: "DA2 save (preferred_game is da2)",
      check: (snapshot) => snapshot.summary.preferred_game === "da2",
    };
  },
  mainCharacter(): InGamePrerequisite {
    return {
      label: "Main character can be loaded",
      check: (snapshot) => namedCharacter(snapshot, "main") !== null,
    };
  },
  companion(name: string): InGamePrerequisite {
    return {
      label: `${name} is present in the party list`,
      check: (snapshot) => namedCharacter(snapshot, `companion:${name}`) !== null,
    };
  },
  companionApproval(name: string): InGamePrerequisite {
    return {
      label: `${name} has an editable approval value`,
      check: (snapshot) => namedCharacter(snapshot, `companion:${name}`)?.approval !== null,
    };
  },
  mainHasAbility(list: AbilityList, abilityName: string): InGamePrerequisite {
    return {
      label: `Main character has ${abilityName} in ${list}`,
      check: (snapshot) => namedCharacter(snapshot, "main")?.[list].some((ability) => ability.name === abilityName) ?? false,
    };
  },
  mainDoesNotHaveAbility(list: AbilityList, abilityName: string): InGamePrerequisite {
    return {
      label: `Main character does not already have ${abilityName} in ${list}`,
      check: (snapshot) => !(namedCharacter(snapshot, "main")?.[list].some((ability) => ability.name === abilityName) ?? false),
    };
  },
  companionDoesNotHaveAbility(name: string, list: AbilityList, abilityName: string): InGamePrerequisite {
    return {
      label: `${name} does not already have ${abilityName} in ${list}`,
      check: (snapshot) =>
        !(namedCharacter(snapshot, `companion:${name}`)?.[list].some((ability) => ability.name === abilityName) ?? false),
    };
  },
  mainEquipmentItem(pattern: RegExp): InGamePrerequisite {
    return {
      label: `Main character has equipped item matching ${pattern}`,
      check: (snapshot) => itemsMatchingName(namedCharacter(snapshot, "main")?.equipment ?? [], pattern).length > 0,
    };
  },
  mainHasEquippedItemWithProperties(): InGamePrerequisite {
    return {
      label: "Main character has at least one equipped item with properties",
      check: (snapshot) =>
        (namedCharacter(snapshot, "main")?.equipment ?? []).some((item) => item.properties.length > 0),
    };
  },
  mainEquipmentItemWithMaterialOption(pattern: RegExp, tier: number, material: string): InGamePrerequisite {
    return {
      label: `Main character has equipped item matching ${pattern} with Tier ${tier} ${material} option`,
      check: (snapshot) =>
        itemsMatchingName(namedCharacter(snapshot, "main")?.equipment ?? [], pattern).some((item) =>
          hasMaterialOption(item, tier, material),
        ),
    };
  },
  companionArmorWithMaterialOption(name: string, tier: number, material: string): InGamePrerequisite {
    return {
      label: `${name} has at least one equipped armor piece with Tier ${tier} ${material} option`,
      check: (snapshot) =>
        (namedCharacter(snapshot, `companion:${name}`)?.equipment ?? []).some((item) =>
          item.category.label.startsWith("Armor") && hasMaterialOption(item, tier, material),
        ),
    };
  },
  backpackItem(pattern: RegExp): InGamePrerequisite {
    return {
      label: `Backpack contains item matching ${pattern}`,
      check: (snapshot) => snapshot.backpackItems.some((entry) => pattern.test(itemName(entry.item))),
    };
  },
  stackableBackpackItem(pattern: RegExp): InGamePrerequisite {
    return {
      label: `Backpack contains stackable item matching ${pattern}`,
      check: (snapshot) => snapshot.backpackItems.some((entry) => pattern.test(itemName(entry.item)) && entry.item.stackable),
    };
  },
  nonStackableBackpackItem(pattern: RegExp): InGamePrerequisite {
    return {
      label: `Backpack contains non-stackable item matching ${pattern}`,
      check: (snapshot) => snapshot.backpackItems.some((entry) => pattern.test(itemName(entry.item)) && !entry.item.stackable),
    };
  },
  backpackHasItemNotMatching(pattern: RegExp): InGamePrerequisite {
    return {
      label: `Backpack contains at least one item not matching ${pattern}`,
      check: (snapshot) => snapshot.backpackItems.some((entry) => !pattern.test(itemName(entry.item))),
    };
  },
};

export function backupSave() {
  if (existsSync(SAVE_PATH + ".ingame-backup")) {
    console.warn("[ingame] Stale backup detected — restoring from previous crashed run");
    copyFileSync(SAVE_PATH + ".ingame-backup", SAVE_PATH);
    rmSync(SAVE_PATH + ".ingame-backup");
  }
  copyFileSync(SAVE_PATH, SAVE_PATH + ".ingame-backup");
}

export function restoreSave() {
  if (existsSync(SAVE_PATH + ".ingame-backup")) {
    copyFileSync(SAVE_PATH + ".ingame-backup", SAVE_PATH);
    rmSync(SAVE_PATH + ".ingame-backup");
  }
  if (existsSync(SAVE_PATH + ".ingame-working")) {
    rmSync(SAVE_PATH + ".ingame-working");
  }
}

export async function openSave(page: Page, outputPath: string = SAVE_PATH) {
  await page.addInitScript(({ savePath, outPath }) => {
    localStorage.setItem("ingameTestSave", savePath);
    localStorage.setItem("ingameTestSaveOutput", outPath);
  }, { savePath: SAVE_PATH, outPath: outputPath });
  await page.goto("/");
  await page.getByRole("button", { name: /open save/i }).click();
  // Wait until handleOpen FULLY completes. "Reset Drafts" only renders once summary is set,
  // and is disabled while busy=true. When it's present AND enabled, hydration is done
  // (including the setSection/setCharacterTab resets at the tail of handleOpen).
  const resetBtn = page.getByRole("button", { name: "Reset Drafts" });
  await resetBtn.waitFor({ timeout: 30_000 });
  await expect(resetBtn).toBeEnabled({ timeout: 30_000 });
}

// Commit current drafts to the in-memory state (NO write to disk). Use this between
// context switches (character changes, container changes) since drafts are scoped to
// the active character/container and would be silently discarded otherwise.
export async function applyDrafts(page: Page) {
  const applyBtn = page.getByRole("button", { name: /apply drafts/i });
  if (!(await applyBtn.isEnabled())) {
    return;
  }
  await applyBtn.click();
  // The commit can be fast enough to miss busy=true, and some editor state can
  // keep Apply Drafts enabled until the next render. Save As handles the
  // remaining "apply drafts first" dialog, so only wait for the app to be usable.
  await expect(page.getByRole("button", { name: /save as/i })).toBeEnabled({ timeout: 30_000 });
}

// Save current state to disk. If any drafts are still uncommitted, the app asks
// to apply them first — handle that dialog so the save always goes through.
export async function saveAs(page: Page) {
  const saveAsBtn = page.getByRole("button", { name: /save as/i });
  if (!(await saveAsBtn.isEnabled())) {
    return;
  }
  const saved = page.waitForResponse((response) =>
    response.url().includes("/execute_save_command") &&
    response.request().postData()?.includes('"save_as"') === true,
  );
  await saveAsBtn.click();
  const applyAndSaveBtn = page.getByRole("button", { name: "Apply drafts and save" });
  try {
    await applyAndSaveBtn.waitFor({ state: "visible", timeout: 3_000 });
    await applyAndSaveBtn.click();
  } catch {
    // No confirmation dialog — drafts were already committed.
  }
  await saved;
  await page.waitForSelector("text=Saved copy ready");
}

export async function applyAndSave(page: Page) {
  await applyDrafts(page);
  await saveAs(page);
}

// Inject an in-page verification panel and wait for pass/fail click
// Returns true if passed.
export async function verifyInGame(page: Page, checklist: string[]): Promise<boolean> {
  await page.evaluate((items: string[]) => {
    const el = document.createElement("div");
    el.id = "__verify_panel";
    el.style.cssText =
      "position:fixed;inset:0;z-index:9999;background:rgba(0,0,0,0.88);display:flex;align-items:center;justify-content:center;";
    el.innerHTML = `
      <div style="background:#14100c;border:1px solid #c79a48;border-radius:6px;padding:2rem;max-width:520px;color:#f3ead6;font-family:monospace;font-size:0.85rem;">
        <div style="font-size:0.6rem;letter-spacing:0.18em;color:#888;text-transform:uppercase;margin-bottom:0.75rem">In-Game Verification</div>
        <ol style="margin:0 0 1.25rem;padding:0 0 0 1.25rem;line-height:1.9">
          ${items.map((i) => `<li>${i}</li>`).join("")}
        </ol>
        <p style="color:#666;font-size:0.72rem;margin-bottom:1rem">Load the save in Dragon Age: Origins, verify the items above, then return here.</p>
        <div style="display:flex;gap:0.75rem">
          <button id="__vpass" style="flex:1;padding:0.65rem;background:rgba(42,100,42,0.3);border:1px solid rgba(42,150,42,0.6);color:#5fdf5f;cursor:pointer;font-family:monospace;font-size:0.8rem;border-radius:4px">&#10003;  PASS &mdash; all checks good</button>
          <button id="__vfail" style="flex:1;padding:0.65rem;background:rgba(100,28,28,0.3);border:1px solid rgba(150,42,42,0.6);color:#df5f5f;cursor:pointer;font-family:monospace;font-size:0.8rem;border-radius:4px">&#10007;  FAIL &mdash; something wrong</button>
        </div>
      </div>`;
    document.body.appendChild(el);
    document.getElementById("__vpass")!.onclick = () => {
      (window as unknown as Record<string, unknown>)["__verify_result"] = "pass";
    };
    document.getElementById("__vfail")!.onclick = () => {
      (window as unknown as Record<string, unknown>)["__verify_result"] = "fail";
    };
  }, checklist);

  // No timeout — wait indefinitely for the user
  await page.waitForFunction(() => (window as unknown as Record<string, unknown>)["__verify_result"] !== undefined, { timeout: 0 });
  const result = await page.evaluate(() => (window as unknown as Record<string, unknown>)["__verify_result"] as string);
  await page.evaluate(() => document.getElementById("__verify_panel")?.remove());
  return result === "pass";
}
