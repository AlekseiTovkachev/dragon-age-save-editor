import { open, save } from "@tauri-apps/plugin-dialog";
import { mockOpenDialog, mockSaveDialog } from "../test/mockBackend";

const smokeMockEnabled = import.meta.env.VITE_E2E_MOCK === "1";

export async function openSaveDialog() {
  if (smokeMockEnabled) {
    return mockOpenDialog();
  }
  return open({
    title: "Open Dragon Age Save",
    filters: [{ name: "Dragon Age Save", extensions: ["das"] }],
    multiple: false,
  });
}

export async function saveAsDialog(defaultPath: string) {
  if (smokeMockEnabled) {
    return mockSaveDialog();
  }
  return save({
    title: "Save Edited File As",
    defaultPath,
    filters: [{ name: "Dragon Age Save", extensions: ["das"] }],
  });
}
