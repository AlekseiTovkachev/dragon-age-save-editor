import { useCallback, useMemo, useRef } from "react";

export type DraftCheckpoint<TDraft> = {
  current: TDraft | null;
  checkpoint: (draft: TDraft) => void;
  reset: () => TDraft | null;
  clear: () => void;
};

type DraftCheckpointOptions<TDraft> = {
  clone: (draft: TDraft) => TDraft;
};

export function useDraftCheckpoint<TDraft>({
  clone,
}: DraftCheckpointOptions<TDraft>): DraftCheckpoint<TDraft> {
  const checkpointRef = useRef<TDraft | null>(null);

  const checkpoint = useCallback((draft: TDraft) => {
    checkpointRef.current = clone(draft);
  }, [clone]);

  const reset = useCallback(() => {
    return checkpointRef.current === null ? null : clone(checkpointRef.current);
  }, [clone]);

  const clear = useCallback(() => {
    checkpointRef.current = null;
  }, []);

  return useMemo(() => ({
    get current() {
      return checkpointRef.current;
    },
    checkpoint,
    reset,
    clear,
  }), [checkpoint, clear, reset]);
}
