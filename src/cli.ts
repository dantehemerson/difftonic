#!/usr/bin/env bun

import { parseArgs } from "./args";
import { readPatch } from "./patch/sanitize";
import { parsePatch } from "./patch/parse";
import { renderPatch } from "./render";
import { resolveDiffTheme } from "./theme";
import { DEFAULT_SYNTAX_THEME, preloadLanguage } from "./highlight";
import { languageForPath } from "./language";

const VERSION = "0.1.0";

function printHelp(): void {
  process.stdout.write(
    `diffview v${VERSION} - syntax-highlighted terminal diff renderer

Usage:
  diffview [options] < patch

Reads a unified git diff from stdin and writes a syntax-highlighted,
gutter-numbered ANSI rendering to stdout. Suitable for use as a LazyGit
\`stdinFilter\` diffRenderer.

Options:
  --syntax-theme <id>   Shiki theme for code highlighting
                        (default: ${DEFAULT_SYNTAX_THEME})
                        Example ids: github-dark-default, github-light-default,
                        monokai, dracula, nord, one-dark-pro, catppuccin-mocha
  --theme <name>        Diff color theme: dark, light, or auto
                        (default: auto - picks dark or light based on the
                        syntax theme)
  --no-line-numbers     Hide line number gutter
  -h, --help            Show this help text
  -v, --version         Print version

Examples:
  git diff --no-color | diffview
  git diff --no-color | diffview --syntax-theme dracula
  git diff --no-color | diffview --syntax-theme github-light-default --theme auto
  git diff --no-color | diffview --no-line-numbers
`,
  );
}

let parsed;
try {
  parsed = parseArgs(process.argv.slice(2));
} catch (err) {
  process.stderr.write(`diffview: ${(err as Error).message}\n`);
  process.stderr.write(`Try 'diffview --help' for usage information.\n`);
  process.exit(2);
}

if (parsed.help) {
  printHelp();
  process.exit(0);
}

if (parsed.version) {
  process.stdout.write(`diffview v${VERSION}\n`);
  process.exit(0);
}

const syntaxTheme = parsed.syntaxTheme ?? DEFAULT_SYNTAX_THEME;
const theme = resolveDiffTheme(parsed.theme, syntaxTheme);

const stdin = await readPatch();

if (stdin.length === 0) {
  process.exit(0);
}

const pathRegex = /^\+\+\+ b\/(.+)$/gm;
const preloadPromises: Array<Promise<void>> = [];
const seenLangs = new Set<string>();
for (const match of stdin.matchAll(pathRegex)) {
  const lang = languageForPath(match[1]);
  if (lang !== "text" && !seenLangs.has(lang)) {
    seenLangs.add(lang);
    preloadPromises.push(preloadLanguage(lang, syntaxTheme));
  }
}

let parsedPatches;
try {
  parsedPatches = parsePatch(stdin);
} catch (err) {
  await Promise.allSettled(preloadPromises);
  process.stderr.write(`diffview: failed to parse patch: ${(err as Error).message}\n`);
  process.exit(1);
}

const output = await renderPatch(parsedPatches, {
  theme,
  syntaxTheme,
  showLineNumbers: !parsed.noLineNumbers,
});

await Promise.allSettled(preloadPromises);

process.stdout.write(output);
