#!/usr/bin/env -S deno run -A

import { join, dirname, fromFileUrl } from "jsr:@std/path";
import { minify } from "npm:terser";

const scriptDir = dirname(fromFileUrl(import.meta.url));
const projectRoot = join(scriptDir, "../../..");
const libDir = join(projectRoot, "projects/lib");
const pkgDir = join(libDir, "pkg");

// Clean pkg directory
console.log("Cleaning pkg directory...");
try {
  await Deno.remove(pkgDir, { recursive: true });
} catch (e) {
  if (!(e instanceof Deno.errors.NotFound)) throw e;
}

// Build the WASM library
console.log("Building WASM library...");
const buildCmd = new Deno.Command("wasm-pack", {
  args: ["build", "--target", "web"],
  cwd: libDir,
  stdout: "inherit",
  stderr: "inherit",
});
const buildResult = await buildCmd.output();
if (!buildResult.success) {
  console.error("wasm-pack build failed");
  Deno.exit(1);
}

// Rename files to mod.*
console.log("Renaming output files...");
const renames: [string, string][] = [
  ["webterm_dos_ansi.js", "mod.js"],
  ["webterm_dos_ansi.d.ts", "mod.d.ts"],
  ["webterm_dos_ansi_bg.wasm", "mod.wasm"],
  ["webterm_dos_ansi_bg.wasm.d.ts", "mod.wasm.d.ts"],
];

for (const [oldName, newName] of renames) {
  const oldPath = join(pkgDir, oldName);
  const newPath = join(pkgDir, newName);
  try {
    await Deno.rename(oldPath, newPath);
  } catch (e) {
    if (!(e instanceof Deno.errors.NotFound)) throw e;
  }
}

// Update references in mod.js
console.log("Updating file references in mod.js...");
const modJsPath = join(pkgDir, "mod.js");
let modJs = await Deno.readTextFile(modJsPath);
modJs = modJs
  .replaceAll('webterm_dos_ansi_bg.wasm', 'mod.wasm');
await Deno.writeTextFile(modJsPath, modJs);

// Update references in mod.d.ts
console.log("Updating file references in mod.d.ts...");
const modDtsPath = join(pkgDir, "mod.d.ts");
let modDts = await Deno.readTextFile(modDtsPath);
modDts = modDts.replaceAll('webterm_dos_ansi_bg.wasm', 'mod.wasm');
await Deno.writeTextFile(modDtsPath, modDts);

// Patch package.json
console.log("Patching package.json...");
const packageJsonPath = join(pkgDir, "package.json");
const packageJson = JSON.parse(await Deno.readTextFile(packageJsonPath));

packageJson.name = "@bbs/webterm-dos-ansi";
packageJson.main = "mod.js";
packageJson.types = "mod.d.ts";
packageJson.files = [
  "mod.js",
  "mod.d.ts",
  "mod.wasm",
  "mod.wasm.d.ts",
  "README.md",
];

await Deno.writeTextFile(packageJsonPath, JSON.stringify(packageJson, null, 2) + "\n");

// Copy README.md to pkg/
console.log("Copying README.md to pkg/...");
const readmeSrc = join(libDir, "README.md");
const readmeDest = join(pkgDir, "README.md");
await Deno.copyFile(readmeSrc, readmeDest);

// Minify mod.js (do this last after all text replacements)
console.log("Minifying mod.js...");
modJs = await Deno.readTextFile(modJsPath);
const minified = await minify(modJs, {
  module: true,
  compress: true,
  mangle: true,
});
if (!minified.code) {
  console.error("Minification failed");
  Deno.exit(1);
}
await Deno.writeTextFile(modJsPath, minified.code);

console.log("Build complete!");
