import { spawn } from "node:child_process";
import os from "node:os";
import path from "node:path";

const workspace = process.cwd();
const home = os.homedir();
const srcTauri = path.join(workspace, "src-tauri");

const remaps = [
  ["--remap-path-prefix", `${workspace}=dragon-age-save-editor`],
  ["--remap-path-prefix", `${srcTauri}=dragon-age-save-editor/src-tauri`],
  ["--remap-path-prefix", `${home}=~`],
];

const existingRustflags = process.env.RUSTFLAGS?.trim();
const rustflags = [
  ...(existingRustflags ? [existingRustflags] : []),
  ...remaps.flat(),
].join(" ");

let child;
if (process.platform === "win32") {
  const escapedRustflags = rustflags.replaceAll("'", "''");
  child = spawn(
    "powershell.exe",
    [
      "-NoProfile",
      "-ExecutionPolicy",
      "Bypass",
      "-Command",
      `$env:RUSTFLAGS='${escapedRustflags}'; npm exec -- tauri build`,
    ],
    {
      cwd: workspace,
      stdio: "inherit",
    },
  );
} else {
  child = spawn("npm", ["exec", "--", "tauri", "build"], {
    cwd: workspace,
    env: {
      ...process.env,
      RUSTFLAGS: rustflags,
    },
    stdio: "inherit",
  });
}

child.on("exit", (code, signal) => {
  if (signal) {
    console.error(`release build terminated by ${signal}`);
    process.exit(1);
  }
  process.exit(code ?? 1);
});
