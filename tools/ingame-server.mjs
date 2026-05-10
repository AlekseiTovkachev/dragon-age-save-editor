/**
 * HTTP sidecar that mirrors the Tauri invoke interface for in-game Playwright tests.
 * Keeps a working copy of the save and delegates all commands to apply_edit.
 * Port: 7373
 */

import { createServer } from "http";
import { copyFileSync } from "fs";
import { spawnSync } from "child_process";
import * as path from "path";
import { fileURLToPath } from "url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const ROOT = path.resolve(__dirname, "..");
const BINARY = path.join(ROOT, "target", "debug", "apply_edit.exe");
const PORT = 7373;

// Read-only commands — no --save flag
const READ_ONLY_COMMANDS = new Set([
  "validate",
  "get_summary",
  "get_document_assets",
  "get_character",
  "list_available_abilities",
  "list_available_item_properties",
  "list_available_crafting_recipes",
  "list_available_plot_flags",
  "list_characters",
  "list_backpack_items",
  "list_equipment_items",
  "list_crafting_recipes",
  "list_plot_flags",
]);

// Server state
let workingPath = null;

function addCorsHeaders(res) {
  res.setHeader("Access-Control-Allow-Origin", "*");
  res.setHeader("Access-Control-Allow-Headers", "Content-Type");
  res.setHeader("Access-Control-Allow-Methods", "GET, POST, OPTIONS");
}

function readBody(req) {
  return new Promise((resolve, reject) => {
    let data = "";
    req.on("data", (chunk) => { data += chunk; });
    req.on("end", () => {
      try {
        resolve(data ? JSON.parse(data) : {});
      } catch (err) {
        reject(err);
      }
    });
    req.on("error", reject);
  });
}

function runApplyEdit(savePath, command, withSave) {
  const args = [savePath, JSON.stringify(command)];
  if (withSave) args.push("--save");
  const result = spawnSync(BINARY, args, { encoding: "utf8" });
  return result;
}

function sendJson(res, status, body) {
  const json = JSON.stringify(body);
  res.writeHead(status, { "Content-Type": "application/json" });
  res.end(json);
}

const server = createServer(async (req, res) => {
  addCorsHeaders(res);

  // Handle CORS preflight
  if (req.method === "OPTIONS") {
    res.writeHead(204);
    res.end();
    return;
  }

  const url = req.url ?? "/";

  // Health check
  if (url === "/health") {
    sendJson(res, 200, { ok: true });
    return;
  }

  // POST /open_document — copies path to working copy, returns SaveSummary
  if (url === "/open_document" && req.method === "POST") {
    try {
      const body = await readBody(req);
      const sourcePath = body.path;
      if (!sourcePath) {
        sendJson(res, 400, { error: "missing path" });
        return;
      }
      workingPath = sourcePath + ".ingame-working";
      copyFileSync(sourcePath, workingPath);

      const result = runApplyEdit(workingPath, { command: "get_summary" }, false);
      if (result.status !== 0) {
        sendJson(res, 500, { error: result.stderr || `apply_edit exited with code ${result.status}` });
        return;
      }
      const parsed = JSON.parse(result.stdout);
      // open_document returns SaveSummary directly
      sendJson(res, 200, parsed.summary ?? parsed);
    } catch (err) {
      sendJson(res, 500, { error: String(err) });
    }
    return;
  }

  // POST /has_document
  if (url === "/has_document" && req.method === "POST") {
    sendJson(res, 200, workingPath !== null);
    return;
  }

  // POST /execute_save_command
  if (url === "/execute_save_command" && req.method === "POST") {
    try {
      const command = await readBody(req);

      if (!workingPath) {
        sendJson(res, 500, { error: "no document open" });
        return;
      }

      // save_as: copy working file to output_path
      if (command.command === "save_as") {
        const outputPath = command.output_path;
        if (!outputPath) {
          sendJson(res, 400, { error: "save_as requires output_path" });
          return;
        }
        copyFileSync(workingPath, outputPath);
        // Get summary for the response
        const result = runApplyEdit(workingPath, { command: "get_summary" }, false);
        if (result.status !== 0) {
          sendJson(res, 500, { error: result.stderr || `apply_edit exited with code ${result.status}` });
          return;
        }
        const summaryParsed = JSON.parse(result.stdout);
        sendJson(res, 200, {
          result: "saved",
          output_path: outputPath,
          summary: summaryParsed.summary ?? summaryParsed,
        });
        return;
      }

      const isReadOnly = READ_ONLY_COMMANDS.has(command.command);
      const result = runApplyEdit(workingPath, command, !isReadOnly);
      if (result.status !== 0) {
        sendJson(res, 500, { error: result.stderr || `apply_edit exited with code ${result.status}` });
        return;
      }
      const parsed = JSON.parse(result.stdout);
      sendJson(res, 200, parsed);
    } catch (err) {
      sendJson(res, 500, { error: String(err) });
    }
    return;
  }

  sendJson(res, 404, { error: "not found" });
});

server.listen(PORT, "127.0.0.1", () => {
  console.log(`ingame-server listening on http://127.0.0.1:${PORT}`);
});
