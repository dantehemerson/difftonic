import {
  getSharedHighlighter,
  getHighlighterOptions,
} from "@pierre/diffs";
import type {
  DiffsHighlighter,
  SupportedLanguages,
  ThemedToken,
} from "@pierre/diffs";

export const DEFAULT_SYNTAX_THEME = "github-dark-default";

const cache = new Map<string, Promise<DiffsHighlighter>>();
const FALLBACK_TOKEN: ThemedToken = {
  content: "",
  color: "#cccccc",
  htmlStyle: {},
  fg: "#cccccc",
} as unknown as ThemedToken;

function fallbackTokens(line: string): ThemedToken[] {
  return [{ ...FALLBACK_TOKEN, content: line }];
}

async function getHighlighter(
  lang: SupportedLanguages,
  syntaxTheme: string,
): Promise<DiffsHighlighter> {
  const key = `${lang}:${syntaxTheme}`;
  const cached = cache.get(key);
  if (cached) return cached;
  const opts = getHighlighterOptions(lang, {
    theme: syntaxTheme,
    preferredHighlighter: "shiki-wasm",
  });
  const promise = getSharedHighlighter({
    langs: opts.langs,
    themes: opts.themes,
    preferredHighlighter: opts.preferredHighlighter,
  });
  cache.set(key, promise);
  return promise;
}

/** Tokenize a batch of lines in one Shiki call so the highlighter can share grammar state. */
export async function tokenizeLines(
  lines: string[],
  lang: SupportedLanguages,
  syntaxTheme: string = DEFAULT_SYNTAX_THEME,
): Promise<ThemedToken[][]> {
  if (lines.length === 0) return [];
  if (lang === "text") return lines.map(fallbackTokens);
  const highlighter = await getHighlighter(lang, syntaxTheme);
  const stripped = lines.map((l) => l.replace(/\n+$/, ""));
  try {
    const joined = stripped.join("\n");
    const tokens = await highlighter.codeToTokensBase(joined, {
      lang: lang as never,
      theme: syntaxTheme,
    });
    if (tokens.length === stripped.length) return tokens;
    const padded: ThemedToken[][] = [];
    for (let i = 0; i < stripped.length; i++) {
      padded.push(tokens[i] ?? fallbackTokens(stripped[i]!));
    }
    return padded;
  } catch {
    return lines.map(fallbackTokens);
  }
}

/** Tokenize a single line. Prefer {@link tokenizeLines} for multiple lines at once. */
export async function tokenizeLine(
  line: string,
  lang: SupportedLanguages,
  syntaxTheme: string = DEFAULT_SYNTAX_THEME,
): Promise<ThemedToken[]> {
  const [tokens] = await tokenizeLines([line], lang, syntaxTheme);
  return tokens ?? fallbackTokens(line);
}

export async function preloadLanguage(
  lang: SupportedLanguages,
  syntaxTheme: string = DEFAULT_SYNTAX_THEME,
): Promise<void> {
  await getHighlighter(lang, syntaxTheme);
}
