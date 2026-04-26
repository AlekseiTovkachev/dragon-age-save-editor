import { MainTabs } from "./components/MainTabs";
import { Topbar } from "./components/Topbar";
import { CharacterPanel } from "./features/characters/CharacterPanel";
import { CraftingPanel } from "./features/crafting/CraftingPanel";
import { useSaveEditorApp } from "./features/app/useSaveEditorApp";
import { InventoryPanel } from "./features/inventory/InventoryPanel";
import { PlotFlagsPanel } from "./features/plotFlags/PlotFlagsPanel";

function App() {
  const app = useSaveEditorApp();

  return (
    <div className="app-shell">
      <div className="workspace">
        <Topbar
          summary={app.summary}
          screenshotDataUrl={app.screenshotDataUrl}
          busy={app.operation.busy}
          onOpen={() => void app.handleOpen()}
          onSaveAs={() => void app.handleSaveAs()}
          onCommitDrafts={() => void app.commitDrafts()}
          onResetDrafts={app.resetToCommittedDrafts}
        />
        <MainTabs
          sections={app.visibleSections}
          activeSection={app.section}
          onSelect={app.setSection}
          moneyDraft={app.summary ? app.inventoryPanel.state.moneyDraft : undefined}
          onMoneyChange={app.inventoryPanel.actions.setMoneyDraft}
          canEditMoney={app.canEdit}
          busy={app.operation.busy}
        />

        <main className="content">
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
        </main>
      </div>

      {app.operation.error ? (
        <div className="error-banner">
          <span>{app.operation.error}</span>
          <button className="dismiss-button" onClick={app.operation.clearError}>Dismiss</button>
        </div>
      ) : null}
    </div>
  );
}

export default App;
