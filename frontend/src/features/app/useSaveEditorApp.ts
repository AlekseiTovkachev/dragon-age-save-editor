import { useCallback, useEffect, useMemo, useState } from "react";
import { executeCommand, expectResult, hasDocument, openDocument } from "../../api";
import { useAsyncOperation } from "../../hooks/useAsyncOperation";
import { openSaveDialog, saveAsDialog } from "../../lib/dialog";
import { SECTIONS } from "../../lib/navigation";
import type { CharacterTab, Section } from "../../lib/navigation";
import type { InventoryContainer, SaveSummary } from "../../types";
import { useCharacterEditor } from "../characters/useCharacterEditor";
import { useCraftingEditor } from "../crafting/useCraftingEditor";
import { useInventoryEditor } from "../inventory/useInventoryEditor";
import { usePlotFlagsEditor } from "../plotFlags/usePlotFlagsEditor";

export function useSaveEditorApp() {
  const [section, setSection] = useState<Section>("characters");
  const [characterTab, setCharacterTab] = useState<CharacterTab>("overview");
  const [summary, setSummary] = useState<SaveSummary | null>(null);
  const [screenshotDataUrl, setScreenshotDataUrl] = useState<string | null>(null);
  const operation = useAsyncOperation();
  const { run, setError } = operation;
  const activeSection = section === "plot_flags" && summary?.preferred_game !== "da2" ? "characters" : section;

  const refreshSummary = useCallback(async () => {
    const response = expectResult(await executeCommand({ command: "get_summary" }), "summary");
    setSummary(response.summary);
    return response.summary;
  }, []);

  const refreshDocumentAssets = useCallback(async () => {
    const response = expectResult(await executeCommand({ command: "get_document_assets" }), "document_assets");
    setScreenshotDataUrl(response.assets.screenshot_data_url);
  }, []);

  const characterEditor = useCharacterEditor({
    summary,
    run,
    refreshSummary,
  });
  const {
    refreshAvailableAbilities,
    refreshCharacters,
    clear: clearCharacters,
    commitDrafts: commitCharacterDrafts,
    resetToCommittedDrafts: resetCharacterDraftsToCommitted,
  } = characterEditor;

  const selectedInventoryContainer = useMemo<InventoryContainer>(() => {
    if (activeSection === "characters" && characterTab === "equipment") {
      return { equipment: { target: characterEditor.selectedCharacterTarget } };
    }
    return "backpack";
  }, [activeSection, characterEditor.selectedCharacterTarget, characterTab]);

  const isBackpackInventory = activeSection === "inventory" && selectedInventoryContainer === "backpack";
  const inventoryEditor = useInventoryEditor({
    summary,
    container: selectedInventoryContainer,
    isBackpackInventory,
    run,
    setError,
    refreshSummary,
  });
  const {
    refreshAvailableItemProperties,
    refreshItems,
    clear: clearInventory,
    commitDrafts: commitInventoryDrafts,
    resetToCommittedDrafts: resetInventoryDraftsToCommitted,
  } = inventoryEditor;
  const craftingEditor = useCraftingEditor({ run, refreshSummary });
  const {
    refreshAvailableCraftingRecipes,
    refreshCraftingRecipes,
    clear: clearCrafting,
    commitDrafts: commitCraftingDrafts,
    resetToCommittedDrafts: resetCraftingDraftsToCommitted,
  } = craftingEditor;
  const plotFlagsEditor = usePlotFlagsEditor({ run, refreshSummary });
  const {
    refreshAvailablePlotFlags,
    refreshPlotFlags,
    clear: clearPlotFlags,
    commitDrafts: commitPlotFlagDrafts,
    resetToCommittedDrafts: resetPlotFlagDraftsToCommitted,
  } = plotFlagsEditor;

  const resetFeatureState = useCallback(() => {
    clearCharacters();
    clearInventory();
    clearCrafting();
    clearPlotFlags();
  }, [clearCharacters, clearCrafting, clearInventory, clearPlotFlags]);

  const clearDocumentState = useCallback(() => {
    setSummary(null);
    setScreenshotDataUrl(null);
    resetFeatureState();
  }, [resetFeatureState]);

  const hydrateDocument = useCallback(
    async (preferredGame: SaveSummary["preferred_game"]) => {
      await refreshCharacters();
      await refreshAvailableAbilities(preferredGame);
      await refreshAvailableItemProperties();
      await refreshCraftingRecipes();
      await refreshAvailableCraftingRecipes();
      if (preferredGame === "da2") {
        await refreshPlotFlags();
        await refreshAvailablePlotFlags();
      }
    },
    [
      refreshAvailableAbilities,
      refreshAvailableCraftingRecipes,
      refreshAvailableItemProperties,
      refreshAvailablePlotFlags,
      refreshCharacters,
      refreshCraftingRecipes,
      refreshPlotFlags,
    ],
  );

  const hydrateCurrentDocument = useCallback(async () => {
    const refreshed = await refreshSummary();
    await refreshDocumentAssets();
    await hydrateDocument(refreshed.preferred_game);
  }, [hydrateDocument, refreshDocumentAssets, refreshSummary]);

  useEffect(() => {
    void hasDocument().then(async (present) => {
      if (present) {
        await hydrateCurrentDocument();
      }
    });
  }, [hydrateCurrentDocument]);

  const handleSectionSelect = useCallback(
    (nextSection: Section) => {
      setSection(nextSection === "plot_flags" && summary?.preferred_game !== "da2" ? "characters" : nextSection);
    },
    [summary?.preferred_game],
  );

  const shouldLoadItems = activeSection === "inventory" || (activeSection === "characters" && characterTab === "equipment");
  useEffect(() => {
    if (summary && shouldLoadItems) {
      void refreshItems();
    }
  }, [refreshItems, shouldLoadItems, summary]);

  const handleOpen = useCallback(async () => {
    const path = await openSaveDialog();
    if (!path || Array.isArray(path)) {
      return;
    }
    await run(async () => {
      const opened = await openDocument(path);
      const validationResult = expectResult(await executeCommand({ command: "validate" }), "validation");
      if (!validationResult.report.is_valid) {
        clearDocumentState();
        throw new Error("Failed to open save: validation reported an invalid save structure.");
      }
      setSummary(opened);
      await refreshDocumentAssets();
      await hydrateDocument(opened.preferred_game);
      setSection("characters");
      setCharacterTab("overview");
    });
  }, [clearDocumentState, hydrateDocument, refreshDocumentAssets, run]);

  const handleSaveAs = useCallback(async () => {
    if (!summary || !summary.dirty) {
      return;
    }
    const path = await saveAsDialog(summary.source_path);
    if (!path) {
      return;
    }
    await run(async () => {
      const validationResult = expectResult(await executeCommand({ command: "validate" }), "validation");
      if (!validationResult.report.is_valid) {
        throw new Error("Failed to save: the current document is not structurally valid.");
      }
      const response = expectResult(await executeCommand({ command: "save_as", output_path: path }), "saved");
      setSummary(response.summary);
    });
  }, [run, summary]);

  const commitDrafts = useCallback(async () => {
    if (!await commitCharacterDrafts()) {
      return;
    }
    if (!await commitInventoryDrafts()) {
      return;
    }
    if (!await commitCraftingDrafts()) {
      return;
    }
    await commitPlotFlagDrafts();
  }, [commitCharacterDrafts, commitCraftingDrafts, commitInventoryDrafts, commitPlotFlagDrafts]);

  const resetToCommittedDrafts = useCallback(() => {
    resetCharacterDraftsToCommitted();
    resetInventoryDraftsToCommitted();
    resetCraftingDraftsToCommitted();
    resetPlotFlagDraftsToCommitted();
  }, [
    resetCharacterDraftsToCommitted,
    resetCraftingDraftsToCommitted,
    resetInventoryDraftsToCommitted,
    resetPlotFlagDraftsToCommitted,
  ]);

  const visibleSections = useMemo(
    () => SECTIONS.filter((entry) => entry !== "plot_flags" || summary?.preferred_game === "da2"),
    [summary?.preferred_game],
  );

  const characterPanel = {
    state: {
      characters: characterEditor.characters,
      characterKey: characterEditor.characterKey,
      character: characterEditor.character,
      isDa2: characterEditor.isDa2,
      levelDraft: characterEditor.levelDraft,
      experienceDraft: characterEditor.experienceDraft,
      approvalDraft: characterEditor.approvalDraft,
      statsDraft: characterEditor.statsDraft,
      pointPoolsDraft: characterEditor.pointPoolsDraft,
      visibleAbilityKinds: characterEditor.visibleAbilityKinds,
      selectedAbilityToAdd: characterEditor.selectedAbilityToAdd,
      availableAbilities: characterEditor.availableAbilities,
      abilityDrafts: characterEditor.abilityDrafts,
    },
    actions: {
      setCharacterKey: characterEditor.setCharacterKey,
      setLevelDraft: characterEditor.setLevelDraft,
      setExperienceDraft: characterEditor.setExperienceDraft,
      setApprovalDraft: characterEditor.setApprovalDraft,
      setStatsDraft: characterEditor.setStatsDraft,
      setPointPoolsDraft: characterEditor.setPointPoolsDraft,
      coreAbilityOptions: characterEditor.coreAbilityOptions,
      visibleTreeAbilities: characterEditor.visibleTreeAbilities,
      setSelectedAbilityToAdd: characterEditor.setSelectedAbilityToAdd,
      handleAbilityAdd: characterEditor.handleAbilityAdd,
      visibleAbilities: characterEditor.visibleAbilities,
      abilityIsLocked: characterEditor.abilityIsLocked,
      handleAbilityRemove: characterEditor.handleAbilityRemove,
      handleVisibleAbilityAdd: characterEditor.handleVisibleAbilityAdd,
    },
  };

  const inventoryPanel = {
    state: {
      moneyDraft: inventoryEditor.moneyDraft,
      items: inventoryEditor.items,
      itemIndex: inventoryEditor.itemIndex,
      selectedItem: inventoryEditor.selectedItem,
      canEditStackSize: inventoryEditor.canEditStackSize,
      canCloneBackpackItem: inventoryEditor.canCloneBackpackItem,
      canEditMaterial: inventoryEditor.canEditMaterial,
      itemMetadataDraft: inventoryEditor.itemMetadataDraft,
      propertyDraft: inventoryEditor.propertyDraft,
      itemPropertiesDraft: inventoryEditor.itemPropertiesDraft,
      availableItemProperties: inventoryEditor.availableItemProperties,
    },
    actions: {
      setMoneyDraft: inventoryEditor.setMoneyDraft,
      setItemIndex: inventoryEditor.setItemIndex,
      setItemMetadataDraft: inventoryEditor.setItemMetadataDraft,
      setPropertyDraft: inventoryEditor.setPropertyDraft,
      handlePropertyAddDraft: inventoryEditor.handlePropertyAddDraft,
      handlePropertyRemoveDraft: inventoryEditor.handlePropertyRemoveDraft,
      handlePropertyUpdateDraft: inventoryEditor.handlePropertyUpdateDraft,
      handleBackpackRemove: inventoryEditor.handleBackpackRemove,
      handleBackpackClone: inventoryEditor.handleBackpackClone,
      handleWikiOpen: inventoryEditor.handleWikiOpen,
    },
  };

  const craftingPanel = {
    state: {
      sortedRecipeIds: craftingEditor.sortedRecipeIds,
      groupedRecipeIds: craftingEditor.groupedRecipeIds,
      craftingRecipeDrafts: craftingEditor.craftingRecipeDrafts,
    },
    actions: {
      handleToggle: craftingEditor.handleToggle,
      recipeIsKnown: craftingEditor.recipeIsKnown,
      recipeLabel: craftingEditor.recipeLabel,
    },
  };

  const plotFlagsPanel = {
    state: {
      plotIntegerValues: plotFlagsEditor.plotIntegerValues,
      plotIntegerDrafts: plotFlagsEditor.plotIntegerDrafts,
      plotBooleanValues: plotFlagsEditor.plotBooleanValues,
      plotBooleanDrafts: plotFlagsEditor.plotBooleanDrafts,
      groupedPlotIntegers: plotFlagsEditor.groupedPlotIntegers,
      groupedPlotBooleans: plotFlagsEditor.groupedPlotBooleans,
    },
    actions: {
      handleIntegerChange: plotFlagsEditor.handleIntegerChange,
      handleBooleanToggle: plotFlagsEditor.handleBooleanToggle,
    },
  };

  const sectionCounts = {
    characters: characterPanel.state.characters.length,
    inventory: summary?.backpack_count || 0,
    recipes: craftingPanel.state.craftingRecipeDrafts.length,
    plot_flags: plotFlagsPanel.state.groupedPlotIntegers.reduce((total, group) => total + group.flags.length, 0),
  } satisfies Record<Section, number>;

  return {
    section: activeSection,
    setSection: handleSectionSelect,
    characterTab,
    setCharacterTab,
    summary,
    screenshotDataUrl,
    visibleSections,
    sectionCounts,
    canEdit: Boolean(summary),
    operation,
    characterPanel,
    inventoryPanel,
    craftingPanel,
    plotFlagsPanel,
    refreshSummary,
    hydrateDocument,
    clearDocumentState,
    commitDrafts,
    resetToCommittedDrafts,
    handleOpen,
    handleSaveAs,
  };
}
