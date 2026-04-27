import { spawnSync } from "node:child_process";
import { mkdirSync } from "node:fs";

const ignoredFiles = "src/gff4/fields.rs";

const check = spawnSync("cargo", ["llvm-cov", "--version"], { stdio: "ignore" });

if (check.status !== 0) {
  console.error("Rust coverage requires cargo-llvm-cov.");
  console.error("Install it with: cargo install cargo-llvm-cov");
  process.exit(1);
}

mkdirSync("coverage/rust/html", { recursive: true });

const lcov = spawnSync("cargo", [
  "llvm-cov",
  "--workspace",
  "--ignore-filename-regex",
  ignoredFiles,
  "--lcov",
  "--output-path",
  "coverage/rust/lcov.info",
], { stdio: "inherit" });

if (lcov.error) {
  console.error(lcov.error.message);
  process.exit(1);
}

if (lcov.status !== 0) {
  process.exit(lcov.status ?? 1);
}

const html = spawnSync("cargo", [
  "llvm-cov",
  "--workspace",
  "--ignore-filename-regex",
  ignoredFiles,
  "--html",
  "--output-dir",
  "coverage/rust",
  "--no-clean",
], { stdio: "inherit" });

if (html.error) {
  console.error(html.error.message);
  process.exit(1);
}

process.exit(html.status ?? 1);
