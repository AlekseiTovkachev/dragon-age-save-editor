import { useCallback, useState } from "react";
import { toErrorMessage } from "../api";

export function useAsyncOperation() {
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const clearError = useCallback(() => setError(null), []);

  const run = useCallback(async (action: () => Promise<void>) => {
    setBusy(true);
    setError(null);
    try {
      await action();
      return true;
    } catch (caught) {
      setError(toErrorMessage(caught));
      return false;
    } finally {
      setBusy(false);
    }
  }, []);

  return { busy, error, run, setError, clearError };
}
