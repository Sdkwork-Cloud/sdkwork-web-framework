#!/usr/bin/env node
/** Collects QUALITY_GATE_SPEC / RELEASE_SPEC release evidence bundle metadata. */

import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import path from "node:path";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const outputDir = path.join(repoRoot, "target", "release-evidence");
const outputPath = path.join(outputDir, "release-evidence.json");

function gitRev() {
  const result = spawnSync("git", ["rev-parse", "HEAD"], {
    cwd: repoRoot,
    encoding: "utf8",
  });
  if (result.status !== 0) {
    return "unknown";
  }
  return result.stdout.trim();
}

const componentSpec = JSON.parse(
  readFileSync(path.join(repoRoot, "specs", "component.spec.json"), "utf8"),
);

const bundle = {
  schemaVersion: 1,
  kind: "sdkwork.web-framework.release-evidence",
  collectedAt: new Date().toISOString(),
  gitCommit: gitRev(),
  component: componentSpec.component,
  metadata: componentSpec.metadata,
  verification: {
    entrypoints: ["scripts/verify.ps1", "scripts/verify.sh"],
    commands: componentSpec.verification.commands,
  },
  releaseArtifacts: {
    changelog: "CHANGELOG.md",
    rolloutDoc: "docs/24-production-rollout-and-adoption.md",
    adoptionTemplate: "specs/production-adoption.evidence.template.json",
    frameworkPathfinderAdoptions: "specs/framework-adoption.evidence.json",
    adminServerBinary: "target/release/sdkwork-web-admin-server",
    workflowManifest: "sdkwork.workflow.json",
  },
  benchmark: {
    command:
      "cargo test --release -p sdkwork-web-architecture-tests --test pipeline_benchmark",
    scripts: ["scripts/benchmark-pipeline.ps1", "scripts/benchmark-pipeline.sh"],
  },
  deploymentProfile: "cloud",
  runtimeTarget: "server",
};

mkdirSync(outputDir, { recursive: true });
writeFileSync(outputPath, `${JSON.stringify(bundle, null, 2)}\n`, "utf8");

if (process.argv.includes("--print-path")) {
  console.log(outputPath);
} else {
  console.log(`release-evidence: wrote ${outputPath}`);
}
