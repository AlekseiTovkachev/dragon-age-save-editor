import { useCallback, useMemo, useRef } from "react";

export type DraftCheckpoint<TDraft> = {
  current: TDraft | null;
  checkpoint: (draft: TDraft) => void;
  reset: () => TDraft | null;
  clear: () => void;
  hasChanges: (draft: TDraft) => boolean;
};

type DraftCheckpointOptions<TDraft> = {
  clone: (draft: TDraft) => TDraft;
  equals?: (left: TDraft, right: TDraft) => boolean;
};

const jsonEquals = <TDraft,>(left: TDraft, right: TDraft) => JSON.stringify(left) === JSON.stringify(right);

export function useDraftCheckpoint<TDraft>({
  clone,
  equals = jsonEquals,
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

  const hasChanges = useCallback((draft: TDraft) => {
    return checkpointRef.current !== null && !equals(draft, checkpointRef.current);
  }, [equals]);

  return useMemo(() => ({
    get current() {
      return checkpointRef.current;
    },
    checkpoint,
    reset,
    clear,
    hasChanges,
  }), [checkpoint, clear, hasChanges, reset]);
}
