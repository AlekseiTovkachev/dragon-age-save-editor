import { invoke } from "@tauri-apps/api/core";
import type { SaveCommand, SaveCommandResult, SaveSummary } from "./types";
import { mockExecuteCommand, mockHasDocument, mockOpenDocument } from "./test/mockBackend";

type TauriErrorShape = {
  code?: string;
  message?: string;
};

const smokeMockEnabled = import.meta.env.VITE_E2E_MOCK === "1";

export async function openDocument(path: string): Promise<SaveSummary> {
  if (smokeMockEnabled) {
    return mockOpenDocument();
  }
  return invoke<SaveSummary>("open_document", { path });
}

export async function hasDocument(): Promise<boolean> {
  if (smokeMockEnabled) {
    return mockHasDocument();
  }
  return invoke<boolean>("has_document");
}

export async function executeCommand(command: SaveCommand): Promise<SaveCommandResult> {
  if (smokeMockEnabled) {
    return mockExecuteCommand(command);
  }
  return invoke<SaveCommandResult>("execute_save_command", { command });
}

export function expectResult<K extends SaveCommandResult["result"]>(
  response: SaveCommandResult,
  result: K,
): Extract<SaveCommandResult, { result: K }> {
  if (response.result !== result) {
    throw new Error(`Expected ${result} result, received ${response.result}.`);
  }
  return response as Extract<SaveCommandResult, { result: K }>;
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
