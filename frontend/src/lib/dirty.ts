export const isDirty = (draft: string, committed: number | null) => draft !== String(committed ?? "");
