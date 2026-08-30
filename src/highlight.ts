import {
  getSharedHighlighter,
  getHighlighterOptions,
} from "@pierre/diffs";
import type {
  DiffsHighlighter,
  SupportedLanguages,
} from "@pierre/diffs";
import type { ThemedToken } from "@pierre/diffs";

export interface HighlightOptions {
  theme: string;
  lang: SupportedLanguages;
}

const SHIKI_THEME = "github-dark-default";

const cache = new Map<string, Promise<DiffsHighlighter>>();

async function getHighlighter(lang: SupportedLanguages): Promise<DiffsHighlighter> {
  const key = lang;
  const cached = cache.get(key);
  if (cached) return cached;
  const opts = getHighlighterOptions(lang, {
    theme: SHIKI_THEME,
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
): Promise<ThemedToken[]> {
  if (lang === "text") {
    return [{ content: line, color: "#cccccc", htmlStyle: {}, fg: "#cccccc" } as unknown as ThemedToken];
  }
  const highlighter = await getHighlighter(lang);
  try {
    const tokens = await highlighter.codeToTokensBase(line, {
      lang: lang as never,
      theme: SHIKI_THEME,
    });
    return tokens[0] ?? [];
  } catch {
    return [{ content: line, color: "#cccccc", htmlStyle: {}, fg: "#cccccc" } as unknown as ThemedToken];
  }
}

export async function preloadLanguage(lang: SupportedLanguages): Promise<void> {
  await getHighlighter(lang);
}

export const DEFAULT_SYNTAX_THEME = SHIKI_THEME;
