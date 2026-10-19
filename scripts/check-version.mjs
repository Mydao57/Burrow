#!/usr/bin/env node
// Checks that the app version is identical everywhere it is declared and,
// when a tag is given (e.g. "v0.2.0"), that the tag matches it.
//
//   node scripts/check-version.mjs [vX.Y.Z]

import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const read = (rel) => readFileSync(fileURLToPath(new URL(`../${rel}`, import.meta.url)), "utf8");

const cargo = read("Cargo.toml");
const workspacePackage = cargo.match(/\[workspace\.package\][^[]*?\bversion\s*=\s*"([^"]+)"/s);

const versions = {
  "package.json": JSON.parse(read("package.json")).version,
  "Cargo.toml ([workspace.package])": workspacePackage?.[1],
  "src-tauri/tauri.conf.json": JSON.parse(read("src-tauri/tauri.conf.json")).version,
};

const unique = new Set(Object.values(versions));
let ok = true;

for (const [file, version] of Object.entries(versions)) console.log(`${version ?? "(missing)"}\t${file}`);

if (unique.size !== 1 || unique.has(undefined)) {
  console.error("\nVersion mismatch: all files must declare the same version.");
  ok = false;
}

const tag = process.argv[2];
if (tag) {
  const version = [...unique][0];
  if (tag !== `v${version}`) {
    console.error(`\nTag ${tag} does not match the app version (expected v${version}).`);
    ok = false;
  }
}

if (!/^\d+\.\d+\.\d+(-[0-9A-Za-z.-]+)?$/.test([...unique][0] ?? "")) {
  console.error("\nVersion is not valid semver (X.Y.Z or X.Y.Z-pre).");
  ok = false;
}

process.exit(ok ? 0 : 1);
