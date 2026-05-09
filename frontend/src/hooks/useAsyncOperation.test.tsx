import { act, renderHook } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { useAsyncOperation } from "./useAsyncOperation";

describe("useAsyncOperation", () => {
  it("tracks busy state and clears stale errors", async () => {
    const { result } = renderHook(() => useAsyncOperation());

    act(() => result.current.setError("old"));
    let succeeded = false;
    await act(async () => {
      succeeded = await result.current.run(async () => {});
    });

    expect(succeeded).toBe(true);
    expect(result.current.busy).toBe(false);
    expect(result.current.error).toBeNull();
  });

  it("captures thrown errors as messages", async () => {
    const { result } = renderHook(() => useAsyncOperation());

    let succeeded = true;
    await act(async () => {
      succeeded = await result.current.run(async () => {
        throw new Error("boom");
      });
    });

    expect(succeeded).toBe(false);
    expect(result.current.busy).toBe(false);
    expect(result.current.error).toBe("boom");
  });
});
