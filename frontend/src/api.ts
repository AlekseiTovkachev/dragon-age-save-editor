import { invoke } from "@tauri-apps/api/core";
import type { SaveCommand, SaveCommandResult, SaveSummary } from "./types";

type TauriErrorShape = {
  code?: string;
  message?: string;
};

export async function openDocument(path: string): Promise<SaveSummary> {
  return invoke<SaveSummary>("open_document", { path });
}

export async function hasDocument(): Promise<boolean> {
  return invoke<boolean>("has_document");
}

export async function executeCommand(command: SaveCommand): Promise<SaveCommandResult> {
  return invoke<SaveCommandResult>("execute_save_command", { command });
}

export function toErrorMessage(error: unknown): string {
  if (typeof error === "string") {
    return error;
  }
  if (typeof error === "object" && error !== null) {
    const typed = error as TauriErrorShape;
    if (typed.message) {
      return typed.code ? `${typed.code}: ${typed.message}` : typed.message;
    }
  }
  return "An unexpected error occurred.";
}
