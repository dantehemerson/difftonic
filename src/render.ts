import type {
  FileDiffMetadata,
  ParsedPatch,
  SupportedLanguages,
  ThemedToken,
} from "@pierre/diffs";
import { setLanguageOverride } from "@pierre/diffs";
import { tokenizeLine } from "./highlight";
import { languageForPath } from "./language";
import { DEFAULT_THEME, type Theme } from "./theme";

type RenderLineKind =
  | "meta"
  | "hunk-header"
  | "context"
  | "addition"
  | "deletion"
  | "no-newline";

interface RenderLine {
  kind: RenderLineKind;
  text: string;
  oldLine?: number;
  newLine?: number;
  highlight?: boolean;
}

export interface RenderOptions {
  theme?: Theme;
  showLineNumbers?: boolean;
}

export async function renderPatch(
  patches: ParsedPatch[],
  options: RenderOptions = {},
): Promise<string> {
  const theme = options.theme ?? DEFAULT_THEME;
  const out: string[] = [];
  let first = true;

  for (const patch of patches) {
    for (const file of patch.files) {
      if (!first) out.push("");
      first = false;
      await renderFile(file, theme, out, options);
    }
  }

  if (out.length === 0) return "";
  return out.join("\n") + "\n";
}

async function renderFile(
  file: FileDiffMetadata,
  theme: Theme,
  out: string[],
  options: RenderOptions,
): Promise<void> {
  const lang = resolveLang(file);
  const header = fileHeader(file);
  const body = buildFileLines(file);
  const showLineNumbers = options.showLineNumbers !== false;

  for (const line of header) {
    out.push(renderMetaLine(line, theme));
  }
  out.push(renderSeparator(theme));

  for (const rl of body) {
    if (rl.kind === "hunk-header") {
      out.push(renderHunkHeader(rl, theme));
      continue;
    }
    if (rl.kind === "meta") {
      out.push(renderMetaLine(rl, theme));
      continue;
    }
    if (rl.kind === "no-newline") {
      out.push(renderNoNewline(rl, theme));
      continue;
    }

    let tokens: ThemedToken[];
    if (rl.highlight === false) {
      tokens = [{ content: rl.text, color: "#cccccc" } as ThemedToken];
    } else {
      try {
        tokens = await tokenizeLine(rl.text, lang);
      } catch {
        tokens = [{ content: rl.text, color: "#cccccc" } as ThemedToken];
      }
    }

    out.push(renderCodeLine(rl, tokens, theme, showLineNumbers));
  }
}

function fileHeader(file: FileDiffMetadata): RenderLine[] {
  const lines: RenderLine[] = [];
  lines.push({
    kind: "meta",
    text: `diff --git a/${file.prevName ?? file.name} b/${file.name}`,
    highlight: false,
  });

  if (file.newObjectId && file.prevObjectId) {
    lines.push({
      kind: "meta",
      text: `index ${file.prevObjectId}..${file.newObjectId}${file.mode ? ` ${file.mode}` : ""}`,
      highlight: false,
    });
  } else if (file.newObjectId) {
    lines.push({
      kind: "meta",
      text: `index ${file.newObjectId}${file.mode ? `..${file.mode}` : ""}`,
      highlight: false,
    });
  }

  if (file.type === "new") {
    lines.push({
      kind: "meta",
      text: "new file mode " + (file.mode ?? "100644"),
      highlight: false,
    });
  } else if (file.type === "deleted") {
    lines.push({
      kind: "meta",
      text: "deleted file mode " + (file.mode ?? "100644"),
      highlight: false,
    });
  } else if (file.prevMode && file.prevMode !== file.mode && file.mode) {
    lines.push({ kind: "meta", text: `old mode ${file.prevMode}`, highlight: false });
    lines.push({ kind: "meta", text: `new mode ${file.mode}`, highlight: false });
  }

  const oldPath = file.prevName ?? file.name;
  const oldLabel = file.type === "new" ? "/dev/null" : `a/${oldPath}`;
  const newLabel = file.type === "deleted" ? "/dev/null" : `b/${file.name}`;
  lines.push({ kind: "meta", text: `--- ${oldLabel}`, highlight: false });
  lines.push({ kind: "meta", text: `+++ ${newLabel}`, highlight: false });
  return lines;
}

function buildFileLines(file: FileDiffMetadata): RenderLine[] {
  const out: RenderLine[] = [];
  for (const hunk of file.hunks) {
    out.push({
      kind: "hunk-header",
      text:
        hunk.hunkSpecs ??
        `@@ -${hunk.deletionStart},${hunk.deletionCount} +${hunk.additionStart},${hunk.additionCount} @@` +
          (hunk.hunkContext ? ` ${hunk.hunkContext}` : ""),
    });

    for (const block of hunk.hunkContent) {
      if (block.type === "context") {
        const start = block.deletionLineIndex;
        for (let i = 0; i < block.lines; i++) {
          const text = file.deletionLines[start + i] ?? "";
          out.push({
            kind: "context",
            text,
            oldLine: hunk.deletionStart + i,
            newLine: hunk.additionStart + i,
          });
        }
      } else {
        const delStart = block.deletionLineIndex;
        const addStart = block.additionLineIndex;
        for (let i = 0; i < block.deletions; i++) {
          const text = file.deletionLines[delStart + i] ?? "";
          out.push({
            kind: "deletion",
            text,
            oldLine: hunk.deletionStart + i,
          });
        }
        for (let i = 0; i < block.additions; i++) {
          const text = file.additionLines[addStart + i] ?? "";
          out.push({
            kind: "addition",
            text,
            newLine: hunk.additionStart + i,
          });
        }
      }
    }

    if (hunk.noEOFCRDeletions) {
      out.push({ kind: "no-newline", text: "\\ No newline at end of file" });
    }
    if (hunk.noEOFCRAdditions) {
      out.push({ kind: "no-newline", text: "\\ No newline at end of file" });
    }
  }
  return out;
}

function resolveLang(file: FileDiffMetadata): SupportedLanguages {
  const lang = languageForPath(file.name);
  setLanguageOverride(file, lang);
  return lang;
}

function renderMetaLine(line: RenderLine, theme: Theme): string {
  return paint(line.text, {
    bg: theme.metaBg,
    fg: theme.metaFg,
    dim: true,
  });
}

function renderHunkHeader(line: RenderLine, theme: Theme): string {
  return paint(line.text, {
    bg: theme.hunkBg,
    fg: theme.hunkFg,
    bold: true,
  });
}

function renderNoNewline(line: RenderLine, theme: Theme): string {
  return paint(line.text, {
    fg: theme.metaFg,
    dim: true,
  });
}

function renderSeparator(theme: Theme): string {
  const width = 60;
  return paint("─".repeat(width), {
    bg: theme.metaBg,
    fg: theme.metaFg,
    dim: true,
  });
}

interface PaintStyle {
  bg?: number;
  fg: number;
  bold?: boolean;
  dim?: boolean;
}

function paint(text: string, style: PaintStyle): string {
  return openStyle(style) + text + closeStyle();
}

function openStyle(style: PaintStyle): string {
  return "\x1b[0m" + composeSgr(style);
}

function closeStyle(): string {
  return "";
}

function composeSgr(style: PaintStyle): string {
  const parts: string[] = [];
  if (style.bg !== undefined) parts.push(bg(style.bg));
  parts.push(fg(style.fg));
  if (style.bold) parts.push("1");
  if (style.dim) parts.push("2");
  return "\x1b[" + parts.join(";") + "m";
}

function renderCodeLine(
  line: RenderLine,
  tokens: ThemedToken[],
  theme: Theme,
  showLineNumbers: boolean,
): string {
  let out = "";

  if (showLineNumbers) {
    out += renderGutter(line, theme);
  } else {
    out += "\x1b[0m";
  }

  if (line.kind === "addition") {
    out += openStyle({ bg: theme.additionBg, fg: theme.additionAccent, bold: true }) + "+";
    out += renderTokens(tokens, theme.additionBg);
  } else if (line.kind === "deletion") {
    out += openStyle({ bg: theme.deletionBg, fg: theme.deletionAccent, bold: true }) + "-";
    out += renderTokens(tokens, theme.deletionBg);
  } else {
    out += openStyle({ fg: theme.metaFg, dim: true }) + " ";
    out += renderTokens(tokens, undefined);
  }

  return out;
}

function renderGutter(line: RenderLine, theme: Theme): string {
  const oldNum = line.oldLine !== undefined ? pad(line.oldLine, 4) : "    ";
  const newNum = line.newLine !== undefined ? pad(line.newLine, 4) : "    ";

  const oldActive = line.kind === "deletion";
  const newActive = line.kind === "addition";

  let out = "";
  out += openStyle({
    bg: theme.metaBg,
    fg: oldActive ? theme.deletionAccent : theme.metaFg,
    bold: oldActive,
    dim: !oldActive,
  }) + oldNum;
  out += " ";
  out += openStyle({
    bg: theme.metaBg,
    fg: newActive ? theme.additionAccent : theme.metaFg,
    bold: newActive,
    dim: !newActive,
  }) + newNum;
  out += openStyle({
    bg: theme.metaBg,
    fg: theme.metaFg,
    dim: true,
  }) + "│";
  out += " ";
  return out;
}

function renderTokens(tokens: ThemedToken[], bgHex: number | undefined): string {
  let out = "";
  let lastFg: number | undefined;
  for (const t of tokens) {
    const fgColor = parseHex(t.color);
    if (fgColor !== lastFg) {
      out += openStyle({
        bg: bgHex,
        fg: fgColor ?? 0xcccccc,
      });
      lastFg = fgColor;
    }
    out += t.content;
  }
  if (tokens.length > 0) out += "\x1b[0m";
  return out;
}

function parseHex(input: string | undefined): number | undefined {
  if (!input) return undefined;
  const hex = input.replace("#", "");
  if (hex.length !== 6) return undefined;
  return parseInt(hex, 16);
}

function pad(n: number, w: number): string {
  return n.toString().padStart(w, " ");
}

function bg(hex: number): string {
  const r = (hex >> 16) & 0xff;
  const g = (hex >> 8) & 0xff;
  const b = hex & 0xff;
  return `48;2;${r};${g};${b}`;
}

function fg(hex: number): string {
  const r = (hex >> 16) & 0xff;
  const g = (hex >> 8) & 0xff;
  const b = hex & 0xff;
  return `38;2;${r};${g};${b}`;
}
