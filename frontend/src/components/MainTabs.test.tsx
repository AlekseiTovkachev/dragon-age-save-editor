import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { MainTabs } from "./MainTabs";

describe("MainTabs", () => {
  it("renders visible sections and emits selected section", () => {
    const onSelect = vi.fn();

    render(<MainTabs sections={["characters", "inventory"]} activeSection="characters" onSelect={onSelect} />);

    expect(screen.getByRole("button", { name: "Characters" })).toHaveClass("active");
    expect(screen.queryByRole("button", { name: "Plot Flags" })).not.toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: "Inventory" }));
    expect(onSelect).toHaveBeenCalledWith("inventory");
  });

  it("renders the money draft beside the tabs", () => {
    const onMoneyChange = vi.fn();

    render(
      <MainTabs
        sections={["characters", "inventory"]}
        activeSection="inventory"
        onSelect={vi.fn()}
        moneyDraft="123"
        onMoneyChange={onMoneyChange}
        canEditMoney
      />,
    );

    const moneyInput = screen.getByLabelText("Money");
    expect(moneyInput).toHaveValue("123");

    fireEvent.change(moneyInput, { target: { value: "456" } });
    expect(onMoneyChange).toHaveBeenCalledWith("456");
  });

  it("rejects non-numeric and negative money input", () => {
    const onMoneyChange = vi.fn();

    render(
      <MainTabs
        sections={["characters", "inventory"]}
        activeSection="inventory"
        onSelect={vi.fn()}
        moneyDraft="123"
        onMoneyChange={onMoneyChange}
        canEditMoney
      />,
    );

    const moneyInput = screen.getByLabelText("Money");

    fireEvent.change(moneyInput, { target: { value: "12a" } });
    fireEvent.change(moneyInput, { target: { value: "-12" } });

    expect(onMoneyChange).not.toHaveBeenCalled();
  });
});
