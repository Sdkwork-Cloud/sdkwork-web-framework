#!/usr/bin/env node
/** Builds the PC admin console dist with E2E dual-token credentials baked in. */

import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import path from "node:path";
import { e2eAccessToken } from "./e2e-constants.mjs";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const pcApp = path.join(repoRoot, "apps", "sdkwork-web-framework-pc");

const result = spawnSync("npm", ["exec", "--", "tsc", "-b"], {
  cwd: pcApp,
  env: process.env,
  stdio: "inherit",
  shell: process.platform === "win32",
});
if (result.status !== 0) {
  process.exit(result.status ?? 1);
}

const build = spawnSync("npm", ["exec", "--", "vite", "build"], {
  cwd: pcApp,
  env: {
    ...process.env,
    SDKWORK_ACCESS_TOKEN: e2eAccessToken(),
  },
  stdio: "inherit",
  shell: process.platform === "win32",
});
process.exit(build.status ?? 1);
