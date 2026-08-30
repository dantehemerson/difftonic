import {
  getSharedHighlighter,
  getHighlighterOptions,
} from "@pierre/diffs";
import type {
  DiffsHighlighter,
  SupportedLanguages,
} from "@pierre/diffs";
import type { ThemedToken } from "@pierre/diffs";

export const DEFAULT_SYNTAX_THEME = "github-dark-default";

const cache = new Map<string, Promise<DiffsHighlighter>>();

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

export async function tokenizeLine(
  line: string,
  lang: SupportedLanguages,
  syntaxTheme: string = DEFAULT_SYNTAX_THEME,
): Promise<ThemedToken[]> {
  if (lang === "text") {
    return [{ content: line, color: "#cccccc", htmlStyle: {}, fg: "#cccccc" } as unknown as ThemedToken];
  }
  const highlighter = await getHighlighter(lang, syntaxTheme);
  try {
    const tokens = await highlighter.codeToTokensBase(line, {
      lang: lang as never,
      theme: syntaxTheme,
    });
    return tokens[0] ?? [];
  } catch {
    return [{ content: line, color: "#cccccc", htmlStyle: {}, fg: "#cccccc" } as unknown as ThemedToken];
  }
}

export async function preloadLanguage(
  lang: SupportedLanguages,
  syntaxTheme: string = DEFAULT_SYNTAX_THEME,
): Promise<void> {
  await getHighlighter(lang, syntaxTheme);
}
