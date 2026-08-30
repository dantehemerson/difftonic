import type {
  FileDiffMetadata,
  ParsedPatch,
  SupportedLanguages,
  ThemedToken,
} from "@pierre/diffs";
import { setLanguageOverride } from "@pierre/diffs";
import { DEFAULT_SYNTAX_THEME, tokenizeLine } from "./highlight";
import { languageForPath } from "./language";
import { DEFAULT_THEME, type Theme } from "./theme";

type RenderLineKind =
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
  syntaxTheme?: string;
  /** Padded width of the file title bar. Defaults to terminal width or 80. */
  titleWidth?: number;
}

const DEFAULT_TITLE_WIDTH = 80;
const MIN_TITLE_WIDTH = 12;
const SEPARATOR_CHAR = "─";
/** Generic file icon from Nerd Fonts (nf-seti-default, codepoint U+F15B). */
const FILE_ICON = "\u{F15B}";
const ICON_TEXT_GAP = " ";

/** Detect the terminal width to use for title bars and separators. */
export function detectTitleWidth(): number {
  const stdout = process.stdout as { columns?: number };
  if (typeof stdout.columns === "number" && stdout.columns > 0) {
    return stdout.columns;
  }
  const env = process.env.COLUMNS;
  if (env) {
    const parsed = parseInt(env, 10);
    if (!Number.isNaN(parsed) && parsed > 0) return parsed;
  }
  return DEFAULT_TITLE_WIDTH;
}

function resolveTitleWidth(option: number | undefined): number {
  if (option !== undefined && option >= MIN_TITLE_WIDTH) return option;
  return detectTitleWidth();
}

export async function renderPatch(
  patches: ParsedPatch[],
  options: RenderOptions = {},
): Promise<string> {
  const theme = options.theme ?? DEFAULT_THEME;
  const syntaxTheme = options.syntaxTheme ?? DEFAULT_SYNTAX_THEME;
  const titleWidth = resolveTitleWidth(options.titleWidth);
  const out: string[] = [];
  let first = true;

  for (const patch of patches) {
    for (const file of patch.files) {
      if (!first) {
        out.push(renderFileSeparator(theme, titleWidth));
        out.push("");
      }
      first = false;
      await renderFile(file, theme, out, { ...options, syntaxTheme, titleWidth });
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
  const syntaxTheme = options.syntaxTheme ?? DEFAULT_SYNTAX_THEME;
  const titleWidth = resolveTitleWidth(options.titleWidth);
  const body = buildFileLines(file);
  const showLineNumbers = options.showLineNumbers !== false;

  for (const line of renderFileTitle(file, theme, titleWidth)) {
    out.push(line);
  }

  for (const rl of body) {
    if (rl.kind === "hunk-header") {
      out.push(renderHunkHeader(rl, theme));
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
        tokens = await tokenizeLine(rl.text, lang, syntaxTheme);
      } catch {
        tokens = [{ content: rl.text, color: "#cccccc" } as ThemedToken];
      }
    }

    out.push(renderCodeLine(rl, tokens, theme, showLineNumbers));
  }
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

function renderFileSeparator(theme: Theme, width: number): string {
  return paint(SEPARATOR_CHAR.repeat(width), {
    fg: theme.fileSeparatorFg,
    dim: true,
  });
}

interface FileStats {
  additions: number;
  deletions: number;
  isNew: boolean;
  isDeleted: boolean;
  isRenamed: boolean;
  stateLabel: string | null;
}

function fileStats(file: FileDiffMetadata): FileStats {
  let additions = 0;
  let deletions = 0;
  for (const hunk of file.hunks) {
    for (const block of hunk.hunkContent) {
      if (block.type === "change") {
        additions += block.additions;
        deletions += block.deletions;
      }
    }
  }

  const isNew = file.type === "new";
  const isDeleted = file.type === "deleted";
  const isRenamed =
    file.type === "rename-pure" || file.type === "rename-changed";

  let stateLabel: string | null = null;
  if (isNew) stateLabel = "new";
  else if (isDeleted) stateLabel = "deleted";
  else if (isRenamed) stateLabel = "renamed";

  return { additions, deletions, isNew, isDeleted, isRenamed, stateLabel };
}

function renderFileTitle(file: FileDiffMetadata, theme: Theme, width: number): string[] {
  const stats = fileStats(file);
  const displayPath = file.prevName && file.prevName !== file.name
    ? `${file.prevName} → ${file.name}`
    : file.name;

  const statsText = formatStats(stats, theme);
  const titleText = displayPath;
  const titlePrefix = `${FILE_ICON}${ICON_TEXT_GAP}`;

  const sidePad = 1;
  const middle = Math.max(1, width - charWidth(titlePrefix) - charWidth(titleText) - charWidth(statsText) - sidePad * 2);
  const trailing = sidePad;

  const titleLine = (() => {
    let out = "\x1b[0m";
    out += openStyle({ bg: theme.fileHeaderBg, fg: theme.fileHeaderFg, bold: true });
    out += " ".repeat(sidePad);
    out += FILE_ICON;
    out += ICON_TEXT_GAP;
    out += titleText;
    out += openStyle({ bg: theme.fileHeaderBg, fg: theme.fileHeaderFg, bold: true });
    out += " ".repeat(middle);
    out += renderStats(stats, theme);
    out += openStyle({ bg: theme.fileHeaderBg });
    out += " ".repeat(trailing);
    out += "\x1b[0m";
    return out;
  })();

  const bgLine = "\x1b[0m" + openStyle({ bg: theme.fileHeaderBg }) + " ".repeat(width);
  return [bgLine, titleLine, bgLine];
}

function formatStats(stats: FileStats, _theme: Theme): string {
  const parts: string[] = [];
  if (stats.isNew) parts.push("new");
  if (stats.isDeleted) parts.push("deleted");
  if (stats.isRenamed) parts.push("renamed");
  parts.push(`+${stats.additions}`);
  parts.push(`-${stats.deletions}`);
  return parts.join(" ");
}

function renderStats(stats: FileStats, theme: Theme): string {
  const tokens: Array<{ kind: "label" | "add" | "del"; text: string }> = [];
  const labels: string[] = [];
  if (stats.isNew) labels.push("new");
  if (stats.isDeleted) labels.push("deleted");
  if (stats.isRenamed) labels.push("renamed");
  if (labels.length > 0) tokens.push({ kind: "label", text: labels.join(" ") });
  tokens.push({ kind: "add", text: `+${stats.additions}` });
  tokens.push({ kind: "del", text: `-${stats.deletions}` });

  let out = "";
  for (let i = 0; i < tokens.length; i++) {
    const t = tokens[i]!;
    if (i > 0) {
      out += openStyle({ bg: theme.fileHeaderBg, fg: theme.fileHeaderMutedFg });
      out += " ";
    }
    if (t.kind === "add") {
      out += openStyle({ bg: theme.fileHeaderBg, fg: theme.additionAccent, bold: true }) + t.text;
    } else if (t.kind === "del") {
      out += openStyle({ bg: theme.fileHeaderBg, fg: theme.deletionAccent, bold: true }) + t.text;
    } else {
      out += openStyle({ bg: theme.fileHeaderBg, fg: theme.fileHeaderMutedFg }) + t.text;
    }
  }
  return out;
}

function charWidth(s: string): number {
  let w = 0;
  for (let i = 0; i < s.length; i++) {
    const code = s.charCodeAt(i);
    if (code === 0x1b) {
      while (i + 1 < s.length && s.charCodeAt(i + 1) !== 109) i++;
      continue;
    }
    w++;
  }
  return w;
}

interface PaintStyle {
  bg?: number;
  fg?: number;
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
  if (style.fg !== undefined) parts.push(fg(style.fg));
  if (style.bold) parts.push("1");
  if (style.dim) parts.push("2");
  if (parts.length === 0) return "";
  return "\x1b[" + parts.join(";") + "m";
}

function renderCodeLine(
  line: RenderLine,
  tokens: ThemedToken[],
  theme: Theme,
  showLineNumbers: boolean,
): string {
  let out = "\x1b[0m";
  out += renderRail(line, theme);

  if (showLineNumbers) {
    out += renderGutter(line, theme);
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

const RAIL = "▌";

function renderRail(line: RenderLine, theme: Theme): string {
  let railStyle: { bg?: number; fg: number; bold?: boolean };
  if (line.kind === "addition") {
    railStyle = { fg: theme.additionAccent, bold: true };
  } else if (line.kind === "deletion") {
    railStyle = { fg: theme.deletionAccent, bold: true };
  } else {
    railStyle = { fg: theme.railContextFg };
  }
  return openStyle(railStyle) + RAIL;
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
