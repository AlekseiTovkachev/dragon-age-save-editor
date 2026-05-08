import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { summary } from "../../test/factories";
import { Sidebar } from "./Sidebar";

describe("Sidebar", () => {
  it("renders document identity, visible nav sections, and action state", () => {
    const onSectionSelect = vi.fn();

    render(
      <Sidebar
        summary={summary({ main_character_name: "Aeducan", dirty: true, preferred_game: "da2" })}
        screenshotDataUrl="data:image/png;base64,abc"
        sections={["characters", "inventory", "plot_flags"]}
        activeSection="inventory"
        sectionCounts={{ characters: 4, inventory: 42, recipes: 0, plot_flags: 5 }}
        busy={false}
        onSectionSelect={onSectionSelect}
        onOpen={vi.fn()}
        onSaveAs={vi.fn()}
        onCommitDrafts={vi.fn()}
        onResetDrafts={vi.fn()}
      />,
    );

    expect(screen.getByText("Aeducan")).toBeInTheDocument();
    expect(screen.getByText("Unsaved changes")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /Inventory42/ })).toHaveAttribute("aria-current", "page");
    expect(screen.queryByRole("button", { name: /Recipes/ })).not.toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: /Plot Flags5/ }));
    expect(onSectionSelect).toHaveBeenCalledWith("plot_flags");
    expect(screen.getByRole("button", { name: /Save As/ })).not.toBeDisabled();
  });

  it("hides draft actions and disables Save As before a save is open", () => {
    render(
      <Sidebar
        summary={null}
        screenshotDataUrl={null}
        sections={["characters", "inventory"]}
        activeSection="characters"
        sectionCounts={{ characters: 0, inventory: 0, recipes: 0, plot_flags: 0 }}
        busy={false}
        onSectionSelect={vi.fn()}
        onOpen={vi.fn()}
        onSaveAs={vi.fn()}
        onCommitDrafts={vi.fn()}
        onResetDrafts={vi.fn()}
      />,
    );

    expect(screen.getByText("No save loaded")).toBeInTheDocument();
    expect(screen.queryByRole("button", { name: /Apply Drafts/ })).not.toBeInTheDocument();
    expect(screen.queryByRole("button", { name: /Reset Drafts/ })).not.toBeInTheDocument();
    expect(screen.getByRole("button", { name: /Save As/ })).toBeDisabled();
  });
});
