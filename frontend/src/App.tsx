import { AppShell } from "./components/AppShell";
import { ApplyOnSavePrompt } from "./components/ApplyOnSavePrompt";
import { Sidebar } from "./components/Sidebar/Sidebar";
import { CharacterPanel } from "./features/characters/CharacterPanel";
import { CraftingPanel } from "./features/crafting/CraftingPanel";
import { useSaveEditorApp } from "./features/app/useSaveEditorApp";
import { InventoryPanel } from "./features/inventory/InventoryPanel";
import { PlotFlagsPanel } from "./features/plotFlags/PlotFlagsPanel";

function App() {
  const app = useSaveEditorApp();

  return (
    <AppShell
      sidebar={
        <Sidebar
          summary={app.summary}
          screenshotDataUrl={app.screenshotDataUrl}
          sections={app.visibleSections}
          activeSection={app.section}
          sectionCounts={app.sectionCounts}
          busy={app.operation.busy}
          hasPlotWarnings={app.hasPlotWarnings}
          hasPendingDrafts={app.hasPendingDrafts}
          onSectionSelect={app.setSection}
          onOpen={() => void app.handleOpen()}
          onSaveAs={() => void app.handleSaveAs()}
          onCommitDrafts={() => void app.commitDrafts()}
          onResetDrafts={app.resetToCommittedDrafts}
        />
      }
    >
      <div className="content">
        {!app.summary ? (
          <section className="panel empty-state">
            <h2>No save open</h2>
            <p>Use Open Save to start a single-document editing session.</p>
          </section>
        ) : null}

        {app.summary && app.section === "characters" ? (
          <CharacterPanel
            state={app.characterPanel.state}
            actions={app.characterPanel.actions}
            inventoryState={app.inventoryPanel.state}
            inventoryActions={app.inventoryPanel.actions}
            characterTab={app.characterTab}
            setCharacterTab={app.setCharacterTab}
            canEdit={app.canEdit}
            busy={app.operation.busy}
          />
        ) : null}

        {app.summary && app.section === "inventory" ? (
          <InventoryPanel
            state={app.inventoryPanel.state}
            actions={app.inventoryPanel.actions}
            canEdit={app.canEdit}
            busy={app.operation.busy}
            hideMaterial={app.summary.preferred_game === "da2"}
          />
        ) : null}

        {app.summary && app.section === "recipes" ? (
          <CraftingPanel
            state={app.craftingPanel.state}
            actions={app.craftingPanel.actions}
            canEdit={app.canEdit}
            busy={app.operation.busy}
          />
        ) : null}

        {app.summary && app.section === "plot_flags" ? (
          <PlotFlagsPanel
            state={app.plotFlagsPanel.state}
            actions={app.plotFlagsPanel.actions}
            summary={app.summary}
            canEdit={app.canEdit}
            busy={app.operation.busy}
          />
        ) : null}
      </div>

      {app.operation.error ? (
        <div className="error-banner">
          <span>{app.operation.error}</span>
          <button className="dismiss-button" onClick={app.operation.clearError}>Dismiss</button>
        </div>
      ) : null}
      <ApplyOnSavePrompt
        open={app.saveAsPrompt.open}
        busy={app.operation.busy}
        onConfirm={() => void app.saveAsPrompt.onConfirm()}
        onCancel={app.saveAsPrompt.onCancel}
      />
    </AppShell>
  );
}

export default App;
