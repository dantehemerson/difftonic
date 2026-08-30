#!/usr/bin/env bun

import { readPatch } from "./patch/sanitize";
import { parsePatch } from "./patch/parse";
import { renderPatch } from "./render";

const stdin = await readPatch();

if (stdin.length === 0) {
  process.exit(0);
}

const parsed = parsePatch(stdin);
const output = await renderPatch(parsed);
process.stdout.write(output);
