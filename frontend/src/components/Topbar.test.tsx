import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { summary } from "../test/factories";
import { Topbar } from "./Topbar";

describe("Topbar", () => {
  it("renders empty document state and disables saving", () => {
    render(
      <Topbar
        summary={null}
        screenshotDataUrl={null}
        busy={false}
        onOpen={vi.fn()}
        onSaveAs={vi.fn()}
        onCommitDrafts={vi.fn()}
        onResetDrafts={vi.fn()}
      />,
    );

    expect(screen.getByText("No save loaded")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /Save As/ })).toBeDisabled();
    expect(screen.queryByRole("button", { name: /Commit Changes/ })).not.toBeInTheDocument();
    expect(screen.queryByRole("button", { name: /Reset Drafts/ })).not.toBeInTheDocument();
  });

  it("renders save metadata and screenshot states", () => {
    render(
      <Topbar
        summary={summary({ main_character_name: "Aeducan", preferred_game: "dao", dirty: true })}
        screenshotDataUrl="data:image/png;base64,abc"
        busy={false}
        onOpen={vi.fn()}
        onSaveAs={vi.fn()}
        onCommitDrafts={vi.fn()}
        onResetDrafts={vi.fn()}
      />,
    );

    expect(screen.getByText("Aeducan")).toBeInTheDocument();
    expect(screen.getByText(/DAO/)).toBeInTheDocument();
    expect(screen.getByAltText("Save screenshot")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /Commit Changes/ })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /Reset Drafts/ })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /Save As/ })).not.toBeDisabled();
  });

  it("disables saving when the loaded save is unchanged", () => {
    render(
      <Topbar
        summary={summary({ dirty: false })}
        screenshotDataUrl={null}
        busy={false}
        onOpen={vi.fn()}
        onSaveAs={vi.fn()}
        onCommitDrafts={vi.fn()}
        onResetDrafts={vi.fn()}
      />,
    );

    expect(screen.getByRole("button", { name: /Save As/ })).toBeDisabled();
  });
});
