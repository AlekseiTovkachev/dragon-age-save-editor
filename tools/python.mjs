import { spawnSync } from "node:child_process";

const scriptArgs = process.argv.slice(2);

if (scriptArgs.length === 0) {
  console.error("Usage: node tools/python.mjs <script.py> [args...]");
  process.exit(2);
}

const candidates = process.platform === "win32"
  ? [
      ["py", ["-3"]],
      ["python", []],
      ["python3", []],
    ]
  : [
      ["python3", []],
      ["python", []],
    ];

const candidate = candidates.find(([command, prefixArgs]) => {
  const result = spawnSync(command, [...prefixArgs, "--version"], {
    stdio: "ignore",
  });
  return result.status === 0;
});

if (!candidate) {
  console.error("Could not find Python. Install Python 3 and make it available as python3 or python.");
  process.exit(1);
}

const [command, prefixArgs] = candidate;
const result = spawnSync(command, [...prefixArgs, ...scriptArgs], {
  stdio: "inherit",
});

if (result.error) {
  console.error(result.error.message);
  process.exit(1);
}

process.exit(result.status ?? 1);
