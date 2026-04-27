import { spawnSync } from "node:child_process";

const npmCommand = process.platform === "win32" ? "npm.cmd" : "npm";

const steps = [
  ["npm run check", npmCommand, ["run", "check"]],
  ["cargo test", "cargo", ["test"]],
  ["cargo check", "cargo", ["check"]],
  ["cargo check --manifest-path src-tauri/Cargo.toml", "cargo", ["check", "--manifest-path", "src-tauri/Cargo.toml"]],
  ["npm run data:verify", npmCommand, ["run", "data:verify"]],
];

for (const [label, command, args] of steps) {
  console.log(`\n> ${label}`);
  const result = spawnSync(command, args, { stdio: "inherit" });
  if (result.error) {
    console.error(result.error.message);
    process.exit(1);
  }
  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}
