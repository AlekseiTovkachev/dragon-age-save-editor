import { spawnSync } from "node:child_process";

function npmRun(scriptName) {
  const npmExecPath = process.env.npm_execpath;
  if (npmExecPath && !/\.(?:cmd|bat|ps1)$/i.test(npmExecPath)) {
    return [process.execPath, [npmExecPath, "run", scriptName], {}];
  }

  if (process.platform === "win32") {
    return ["npm", ["run", scriptName], { shell: true }];
  }

  return ["npm", ["run", scriptName], {}];
}

const steps = [
  ["npm run check", ...npmRun("check")],
  ["cargo test", "cargo", ["test"]],
  ["cargo check", "cargo", ["check"]],
  ["cargo check --manifest-path src-tauri/Cargo.toml", "cargo", ["check", "--manifest-path", "src-tauri/Cargo.toml"]],
  ["npm run data:verify", ...npmRun("data:verify")],
];

for (const [label, command, args, options] of steps) {
  console.log(`\n> ${label}`);
  const result = spawnSync(command, args, { stdio: "inherit", ...options });
  if (result.error) {
    console.error(result.error.message);
    process.exit(1);
  }
  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}
