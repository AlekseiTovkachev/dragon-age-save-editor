import { defineConfig, devices } from "@playwright/test";

export default defineConfig({
  testDir: "ingame",
  timeout: 0, // no timeout — user needs time to verify in-game
  use: {
    baseURL: "http://127.0.0.1:5173",
    ...devices["Desktop Chrome"],
  },
  webServer: [
    {
      command: "npm run dev -- --host 127.0.0.1 --port 5173 --strictPort",
      url: "http://127.0.0.1:5173",
      reuseExistingServer: true,
      env: { VITE_INGAME_SERVER: "http://localhost:7373" },
    },
    {
      command: "node tools/ingame-server.mjs",
      url: "http://localhost:7373/health",
      reuseExistingServer: true,
    },
  ],
});
