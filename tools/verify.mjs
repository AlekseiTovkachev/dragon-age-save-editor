import { spawnSync } from "node:child_process";

const steps = [
  ["npm run check", "npm", ["run", "check"]],
  ["cargo test", "cargo", ["test"]],
  ["cargo check", "cargo", ["check"]],
  ["cargo check --manifest-path src-tauri/Cargo.toml", "cargo", ["check", "--manifest-path", "src-tauri/Cargo.toml"]],
  ["python tools/gamedata/verify_gamedata.py", "python", ["tools/gamedata/verify_gamedata.py"]],
];

for (const [label, command, args] of steps) {
  console.log(`\n> ${label}`);
  const result = process.platform === "win32"
    ? spawnSync("powershell", ["-NoProfile", "-Command", label], { stdio: "inherit" })
    : spawnSync(command, args, { stdio: "inherit" });
  if (result.error) {
    console.error(result.error.message);
    process.exit(1);
  }
  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}
